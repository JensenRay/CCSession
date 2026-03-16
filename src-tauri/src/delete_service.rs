use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use rusqlite::{params, Connection, Error as SqlError, ErrorCode};

use crate::{
    codex_paths::CodexPaths,
    history_store, log_store,
    models::{
        ApiError, ApiErrorCode, DeleteSessionsData, DeleteSessionsRequest, SessionDeleteReport,
        SessionDeleteStatus,
    },
    session_list,
};

#[derive(Debug, Clone)]
struct DeletePlan {
    session_id: String,
    rollout_path: Option<PathBuf>,
    snapshot_path: Option<PathBuf>,
}

pub fn delete_sessions(input: DeleteSessionsRequest) -> Result<DeleteSessionsData, ApiError> {
    let paths = CodexPaths::discover()?;
    delete_sessions_with_paths(&paths, input)
}

pub(crate) fn delete_sessions_with_paths(
    paths: &CodexPaths,
    input: DeleteSessionsRequest,
) -> Result<DeleteSessionsData, ApiError> {
    let ordered_session_ids = dedupe_session_ids(input.session_ids);
    if ordered_session_ids.is_empty() {
        return Err(ApiError::new(
            ApiErrorCode::InvalidInput,
            "sessionIds must contain at least one unique session id",
        ));
    }

    let state_connection = session_list::open_state_db_read_write(paths)?;
    if input.require_codex_stopped {
        ensure_database_is_writable(&state_connection, ApiErrorCode::CodexRunningDetected)?;
    }
    state_connection
        .pragma_update(None, "foreign_keys", true)
        .map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::DeleteExecutionFailed,
                "failed to enable SQLite foreign keys for state_5.sqlite",
                vec![error.to_string()],
            )
        })?;

    let state_rows = load_threads_for_delete(&state_connection, &ordered_session_ids)?;
    if state_rows.is_empty() {
        let reports = ordered_session_ids
            .into_iter()
            .map(SessionDeleteReport::not_found)
            .collect::<Vec<_>>();
        return Ok(summarize_reports(reports, Vec::new()));
    }

    let logs_connection = log_store::open_logs_db_read_write(paths)?;
    if input.require_codex_stopped {
        ensure_database_is_writable(&logs_connection, ApiErrorCode::CodexRunningDetected)?;
    }
    let history_file = history_store::load_history_file(&paths.history_file)?;

    let mut reports = ordered_session_ids
        .iter()
        .map(|session_id| {
            (
                session_id.clone(),
                SessionDeleteReport::not_found(session_id.clone()),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut warnings = history_file.warnings.clone();
    let mut delete_plans = Vec::new();

    for row in state_rows {
        let mut report = SessionDeleteReport {
            session_id: row.session_id.clone(),
            status: SessionDeleteStatus::Failed,
            deleted_state_row: false,
            deleted_history_entries: 0,
            deleted_structured_log_rows: 0,
            deleted_rollout_file: false,
            deleted_snapshot_file: false,
            warnings: Vec::new(),
            error: None,
        };

        let rollout_path = match paths.validate_rollout_path(&row.rollout_path) {
            Ok(path) => Some(path),
            Err(error) => {
                report.error = Some(format!(
                    "rollout path validation failed before deletion: {}",
                    error
                ));
                report.status = SessionDeleteStatus::Failed;
                reports.insert(row.session_id, report);
                continue;
            }
        };

        let snapshot_path = match paths.snapshot_path_for_session(&row.session_id) {
            Ok(path) => Some(path),
            Err(error) => {
                report
                    .warnings
                    .push(format!("snapshot path validation failed: {}", error));
                None
            }
        };

        delete_plans.push(DeletePlan {
            session_id: row.session_id.clone(),
            rollout_path,
            snapshot_path,
        });
        reports.insert(row.session_id, report);
    }

    let mut state_deleted_ids = HashSet::new();

    for plan in &delete_plans {
        let report = reports
            .get_mut(&plan.session_id)
            .expect("delete report must exist for validated plans");

        match delete_state_row(&state_connection, &plan.session_id) {
            Ok(true) => {
                report.deleted_state_row = true;
                state_deleted_ids.insert(plan.session_id.clone());
            }
            Ok(false) => {
                report.status = SessionDeleteStatus::NotFound;
            }
            Err(error) => {
                report.error = Some(error);
                report.status = SessionDeleteStatus::Failed;
            }
        }
    }

    for plan in &delete_plans {
        if !state_deleted_ids.contains(&plan.session_id) {
            continue;
        }

        let report = reports
            .get_mut(&plan.session_id)
            .expect("delete report must exist for state-deleted plans");

        match delete_structured_logs(&logs_connection, &plan.session_id) {
            Ok(deleted_rows) => {
                report.deleted_structured_log_rows = deleted_rows;
            }
            Err(error) => {
                report
                    .warnings
                    .push("structured log cleanup was incomplete".to_string());
                report.error = Some(match &report.error {
                    Some(existing) => format!("{}; {}", existing, error),
                    None => error,
                });
            }
        }
    }

    if !state_deleted_ids.is_empty() {
        match history_file.rewrite_without(&paths.history_file, &state_deleted_ids) {
            Ok(history_deleted_counts) => {
                for (session_id, deleted_entries) in history_deleted_counts {
                    if let Some(report) = reports.get_mut(&session_id) {
                        report.deleted_history_entries = deleted_entries;
                    }
                }
            }
            Err(error) => {
                let message = error.message;
                let details = error.details.join("; ");
                warnings.push(format!("history rewrite failed: {}", details));
                for session_id in &state_deleted_ids {
                    if let Some(report) = reports.get_mut(session_id) {
                        report
                            .warnings
                            .push("history cleanup was incomplete".to_string());
                        report.error = Some(match &report.error {
                            Some(existing) => format!("{}; {}", existing, message),
                            None => message.clone(),
                        });
                    }
                }
            }
        }
    }

    for plan in &delete_plans {
        if !state_deleted_ids.contains(&plan.session_id) {
            continue;
        }

        let report = reports
            .get_mut(&plan.session_id)
            .expect("delete report must exist for file cleanup");

        if let Some(rollout_path) = &plan.rollout_path {
            match trash_optional_file(rollout_path) {
                Ok(Some(true)) => {
                    report.deleted_rollout_file = true;
                }
                Ok(Some(false)) => {
                    report
                        .warnings
                        .push("rollout file was missing before deletion".to_string());
                }
                Ok(None) => {}
                Err(error) => {
                    report.error = Some(match &report.error {
                        Some(existing) => format!("{}; {}", existing, error),
                        None => error,
                    });
                }
            }
        }

        if let Some(snapshot_path) = &plan.snapshot_path {
            match delete_optional_file(snapshot_path) {
                Ok(Some(true)) => {
                    report.deleted_snapshot_file = true;
                }
                Ok(Some(false)) => {
                    report
                        .warnings
                        .push("snapshot file was missing before deletion".to_string());
                }
                Ok(None) => {}
                Err(error) => {
                    report.error = Some(match &report.error {
                        Some(existing) => format!("{}; {}", existing, error),
                        None => error,
                    });
                }
            }
        }
    }

    let mut ordered_reports = Vec::with_capacity(ordered_session_ids.len());
    for session_id in ordered_session_ids {
        let mut report = reports
            .remove(&session_id)
            .unwrap_or_else(|| SessionDeleteReport::not_found(session_id.clone()));
        finalize_report(&mut report);
        ordered_reports.push(report);
    }

    Ok(summarize_reports(ordered_reports, warnings))
}

#[derive(Debug)]
struct StateDeleteRow {
    session_id: String,
    rollout_path: PathBuf,
}

fn load_threads_for_delete(
    connection: &Connection,
    session_ids: &[String],
) -> Result<Vec<StateDeleteRow>, ApiError> {
    if session_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat_n("?", session_ids.len())
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT id, rollout_path FROM threads WHERE id IN ({})",
        placeholders
    );
    let mut statement = connection.prepare(&sql).map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::StateDbQueryFailed,
            "failed to prepare the delete lookup query",
            vec![error.to_string()],
        )
    })?;

    let rows = statement
        .query_map(rusqlite::params_from_iter(session_ids.iter()), |row| {
            Ok(StateDeleteRow {
                session_id: row.get(0)?,
                rollout_path: PathBuf::from(row.get::<_, String>(1)?),
            })
        })
        .map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::StateDbQueryFailed,
                "failed to execute the delete lookup query",
                vec![error.to_string()],
            )
        })?;

    let mut state_rows = Vec::new();
    for row in rows {
        state_rows.push(row.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::StateDbQueryFailed,
                "failed to decode a delete lookup row",
                vec![error.to_string()],
            )
        })?);
    }

    Ok(state_rows)
}

fn delete_state_row(connection: &Connection, session_id: &str) -> Result<bool, String> {
    connection
        .execute("DELETE FROM threads WHERE id = ?1", params![session_id])
        .map(|changed_rows| changed_rows > 0)
        .map_err(|error| format!("failed to delete state row: {}", error))
}

fn delete_structured_logs(connection: &Connection, session_id: &str) -> Result<usize, String> {
    connection
        .execute("DELETE FROM logs WHERE thread_id = ?1", params![session_id])
        .map_err(|error| format!("failed to delete structured logs: {}", error))
}

fn delete_optional_file(path: &PathBuf) -> Result<Option<bool>, String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(Some(true)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Some(false)),
        Err(error) => Err(format!("failed to delete {}: {}", path.display(), error)),
    }
}

fn trash_optional_file(path: &Path) -> Result<Option<bool>, String> {
    if !path.exists() {
        return Ok(Some(false));
    }

    move_to_trash(path).map(|()| Some(true))
}

#[cfg(not(test))]
fn move_to_trash(path: &Path) -> Result<(), String> {
    trash::delete(path)
        .map_err(|error| format!("failed to move {} to trash: {}", path.display(), error))
}

#[cfg(test)]
fn move_to_trash(path: &Path) -> Result<(), String> {
    let codex_root = path
        .ancestors()
        .find(|ancestor| ancestor.file_name().is_some_and(|name| name == ".codex"))
        .ok_or_else(|| {
            format!(
                "failed to locate a test trash directory for {}",
                path.display()
            )
        })?;
    let trash_root = codex_root.join(".trash");
    fs::create_dir_all(&trash_root).map_err(|error| {
        format!(
            "failed to prepare the test trash directory {}: {}",
            trash_root.display(),
            error
        )
    })?;

    let file_name = path.file_name().ok_or_else(|| {
        format!(
            "failed to move {} to trash: file name is missing",
            path.display()
        )
    })?;
    let mut trash_path = trash_root.join(file_name);

    if trash_path.exists() {
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("rollout");
        let extension = path.extension().and_then(|extension| extension.to_str());
        let file_name = match extension {
            Some(extension) => format!("{stem}-trashed.{extension}"),
            None => format!("{stem}-trashed"),
        };
        trash_path = trash_root.join(file_name);
    }

    fs::rename(path, &trash_path).map_err(|error| {
        format!(
            "failed to move {} to trash at {}: {}",
            path.display(),
            trash_path.display(),
            error
        )
    })
}

fn ensure_database_is_writable(
    connection: &Connection,
    error_code: ApiErrorCode,
) -> Result<(), ApiError> {
    connection
        .execute_batch("BEGIN IMMEDIATE; ROLLBACK;")
        .map_err(|error| {
            let message = if is_busy_error(&error) {
                "Codex appears to still be using the local SQLite files"
            } else {
                "failed to acquire a write lock on the local SQLite files"
            };

            ApiError::with_details(error_code, message, vec![error.to_string()])
        })
}

fn is_busy_error(error: &SqlError) -> bool {
    matches!(
        error,
        SqlError::SqliteFailure(
            rusqlite::ffi::Error {
                code: ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked,
                ..
            },
            _
        )
    )
}

fn finalize_report(report: &mut SessionDeleteReport) {
    if report.status == SessionDeleteStatus::NotFound {
        return;
    }

    if !report.deleted_state_row {
        report.status = SessionDeleteStatus::Failed;
        if report.error.is_none() {
            report.error = Some("state row was not deleted".to_string());
        }
        return;
    }

    if report.error.is_some() || !report.warnings.is_empty() {
        report.status = SessionDeleteStatus::PartialFailure;
        return;
    }

    report.status = SessionDeleteStatus::Deleted;
}

fn summarize_reports(
    reports: Vec<SessionDeleteReport>,
    warnings: Vec<String>,
) -> DeleteSessionsData {
    let requested_count = reports.len();
    let deleted_count = reports
        .iter()
        .filter(|report| report.status == SessionDeleteStatus::Deleted)
        .count();
    let partial_failure_count = reports
        .iter()
        .filter(|report| report.status == SessionDeleteStatus::PartialFailure)
        .count();
    let failed_count = reports
        .iter()
        .filter(|report| report.status == SessionDeleteStatus::Failed)
        .count();
    let not_found_count = reports
        .iter()
        .filter(|report| report.status == SessionDeleteStatus::NotFound)
        .count();

    DeleteSessionsData {
        requested_count,
        deleted_count,
        partial_failure_count,
        failed_count,
        not_found_count,
        reports,
        warnings,
    }
}

fn dedupe_session_ids(session_ids: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for session_id in session_ids {
        if seen.insert(session_id.clone()) {
            deduped.push(session_id);
        }
    }

    deduped
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{DeleteSessionsRequest, SessionDeleteStatus},
        test_support::{
            append_history, create_test_codex_root, insert_log, insert_thread, read_history_file,
            touch_rollout, touch_snapshot,
        },
    };

    use super::delete_sessions_with_paths;

    #[test]
    fn deletes_state_history_logs_and_files_with_batch_reports() {
        let fixture = create_test_codex_root();

        touch_rollout(&fixture, "2026/03/15/rollout-session-a.jsonl");
        touch_snapshot(&fixture, "session-a");
        insert_thread(
            &fixture,
            "session-a",
            "Title A",
            "Preview A",
            "/tmp/a",
            "2026/03/15/rollout-session-a.jsonl",
            10,
            20,
            1,
            false,
        );
        insert_thread(
            &fixture,
            "session-b",
            "Title B",
            "Preview B",
            "/tmp/b",
            "../outside/rollout-session-b.jsonl",
            10,
            30,
            1,
            false,
        );
        append_history(&fixture, "session-a", 1, "a-1");
        append_history(&fixture, "session-a", 2, "a-2");
        append_history(&fixture, "session-b", 3, "b-1");
        insert_log(&fixture, "session-a", "message", 1);

        let result = delete_sessions_with_paths(
            &fixture.paths,
            DeleteSessionsRequest {
                session_ids: vec![
                    "session-a".to_string(),
                    "missing".to_string(),
                    "session-b".to_string(),
                ],
                require_codex_stopped: false,
            },
        )
        .unwrap();

        assert_eq!(result.requested_count, 3);
        assert_eq!(result.deleted_count, 1);
        assert_eq!(result.failed_count, 1);
        assert_eq!(result.not_found_count, 1);

        let deleted = &result.reports[0];
        assert_eq!(deleted.session_id, "session-a");
        assert_eq!(deleted.status, SessionDeleteStatus::Deleted);
        assert!(deleted.deleted_state_row);
        assert_eq!(deleted.deleted_history_entries, 2);
        assert_eq!(deleted.deleted_structured_log_rows, 1);
        assert!(deleted.deleted_rollout_file);
        assert!(deleted.deleted_snapshot_file);
        assert!(!fixture
            .root
            .join("sessions/2026/03/15/rollout-session-a.jsonl")
            .exists());
        assert!(fixture.root.join(".trash/rollout-session-a.jsonl").exists());

        let missing = &result.reports[1];
        assert_eq!(missing.status, SessionDeleteStatus::NotFound);

        let invalid = &result.reports[2];
        assert_eq!(invalid.status, SessionDeleteStatus::Failed);
        assert!(invalid
            .error
            .as_ref()
            .unwrap()
            .contains("rollout path validation failed"));

        assert!(
            !fixture.paths.state_db.exists()
                || !std::fs::read_to_string(fixture.paths.history_file.clone())
                    .unwrap()
                    .contains("\"session_id\":\"session-a\"")
        );
        assert!(!fixture
            .paths
            .shell_snapshots_root
            .join("session-a.sh")
            .exists());
        assert!(read_history_file(&fixture).contains("\"session_id\":\"session-b\""));
    }
}
