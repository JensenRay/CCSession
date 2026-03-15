use std::{
    collections::HashMap,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use rusqlite::{Connection, OpenFlags};

use crate::{
    codex_paths::CodexPaths,
    history_store::{self, HistoryAggregation},
    log_store,
    models::{ApiError, ApiErrorCode, ListSessionsData, ListSessionsRequest, SessionListItem},
};

const CONTENT_PREVIEW_LIMIT: usize = 5;

#[derive(Debug, Clone)]
struct ThreadRow {
    id: String,
    rollout_path: String,
    created_at: i64,
    updated_at: i64,
    source: String,
    model_provider: String,
    cwd: String,
    title: String,
    first_user_message: String,
    tokens_used: i64,
    archived: bool,
}

pub fn list_sessions(input: ListSessionsRequest) -> Result<ListSessionsData, ApiError> {
    let paths = CodexPaths::discover()?;
    list_sessions_with_paths(&paths, input)
}

pub(crate) fn list_sessions_with_paths(
    paths: &CodexPaths,
    input: ListSessionsRequest,
) -> Result<ListSessionsData, ApiError> {
    let state_connection = open_state_db_read_only(paths)?;
    let logs_connection = log_store::open_logs_db_read_only(paths)?;
    let threads = load_threads(&state_connection, input.include_archived)?;
    let history = history_store::scan_history_previews(&paths.history_file, CONTENT_PREVIEW_LIMIT)?;
    let log_counts = log_store::load_log_counts(&logs_connection)?;

    build_list_response(paths, threads, history, log_counts)
}

fn build_list_response(
    paths: &CodexPaths,
    threads: Vec<ThreadRow>,
    history: HistoryAggregation,
    log_counts: HashMap<String, usize>,
) -> Result<ListSessionsData, ApiError> {
    let mut warnings = history.warnings;
    let mut sessions = Vec::with_capacity(threads.len());

    for thread in threads {
        let history_summary = history.summaries.get(&thread.id);
        let has_rollout = match paths.validate_rollout_path(Path::new(&thread.rollout_path)) {
            Ok(path) => {
                let exists = path.exists();
                if !exists {
                    warnings.push(format!("rollout file missing for session {}", thread.id));
                }
                exists
            }
            Err(error) => {
                warnings.push(format!(
                    "rollout path validation failed for session {}: {}",
                    thread.id, error
                ));
                false
            }
        };

        let has_snapshot = match paths.snapshot_path_for_session(&thread.id) {
            Ok(path) => path.exists(),
            Err(error) => {
                warnings.push(format!(
                    "snapshot path validation failed for session {}: {}",
                    thread.id, error
                ));
                false
            }
        };

        let summary = if thread.first_user_message.trim().is_empty() {
            thread.title.clone()
        } else {
            thread.first_user_message.clone()
        };

        sessions.push(SessionListItem {
            id: thread.id.clone(),
            title: thread.title,
            summary,
            content_preview: history_summary
                .map(|summary| summary.preview())
                .unwrap_or_default(),
            cwd: thread.cwd,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            tokens_used: thread.tokens_used,
            archived: thread.archived,
            source: thread.source,
            model_provider: thread.model_provider,
            has_rollout,
            has_snapshot,
            history_count: history_summary
                .map(|summary| summary.count)
                .unwrap_or_default(),
            structured_log_count: log_counts.get(&thread.id).copied().unwrap_or_default(),
        });
    }

    Ok(ListSessionsData {
        total: sessions.len(),
        scanned_at: unix_timestamp_now(),
        codex_root: paths.root.display().to_string(),
        warnings,
        sessions,
    })
}

pub(crate) fn open_state_db_read_only(paths: &CodexPaths) -> Result<Connection, ApiError> {
    Connection::open_with_flags(&paths.state_db, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(
        |error| {
            ApiError::with_details(
                ApiErrorCode::StateDbOpenFailed,
                "failed to open state_5.sqlite",
                vec![error.to_string(), paths.state_db.display().to_string()],
            )
        },
    )
}

pub(crate) fn open_state_db_read_write(paths: &CodexPaths) -> Result<Connection, ApiError> {
    Connection::open_with_flags(
        &paths.state_db,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI,
    )
    .map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::StateDbOpenFailed,
            "failed to open state_5.sqlite for deletion",
            vec![error.to_string(), paths.state_db.display().to_string()],
        )
    })
}

fn load_threads(
    connection: &Connection,
    include_archived: bool,
) -> Result<Vec<ThreadRow>, ApiError> {
    let sql = if include_archived {
        "SELECT id, rollout_path, created_at, updated_at, source, model_provider, cwd, title, \
         first_user_message, tokens_used, archived \
         FROM threads \
         ORDER BY updated_at DESC, id DESC"
    } else {
        "SELECT id, rollout_path, created_at, updated_at, source, model_provider, cwd, title, \
         first_user_message, tokens_used, archived \
         FROM threads \
         WHERE archived = 0 \
         ORDER BY updated_at DESC, id DESC"
    };

    let mut statement = connection.prepare(sql).map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::StateDbQueryFailed,
            "failed to prepare the threads query",
            vec![error.to_string()],
        )
    })?;

    let rows = statement
        .query_map([], |row| {
            let archived: i64 = row.get(10)?;
            Ok(ThreadRow {
                id: row.get(0)?,
                rollout_path: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                source: row.get(4)?,
                model_provider: row.get(5)?,
                cwd: row.get(6)?,
                title: row.get(7)?,
                first_user_message: row.get(8)?,
                tokens_used: row.get(9)?,
                archived: archived != 0,
            })
        })
        .map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::StateDbQueryFailed,
                "failed to execute the threads query",
                vec![error.to_string()],
            )
        })?;

    let mut threads = Vec::new();
    for row in rows {
        threads.push(row.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::StateDbQueryFailed,
                "failed to decode a thread row",
                vec![error.to_string()],
            )
        })?);
    }

    Ok(threads)
}

fn unix_timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use crate::{
        models::ListSessionsRequest,
        test_support::{
            append_history, create_test_codex_root, insert_log, insert_thread, touch_rollout,
            touch_snapshot,
        },
    };

    use super::list_sessions_with_paths;

    #[test]
    fn builds_sorted_session_list_with_history_and_log_counts() {
        let fixture = create_test_codex_root();

        touch_rollout(&fixture, "2026/03/15/rollout-session-b.jsonl");
        touch_snapshot(&fixture, "session-b");
        touch_rollout(&fixture, "2026/03/14/rollout-session-a.jsonl");

        insert_thread(
            &fixture,
            "session-a",
            "Older title",
            "",
            "/tmp/project-a",
            "2026/03/14/rollout-session-a.jsonl",
            100,
            150,
            3,
            false,
        );
        insert_thread(
            &fixture,
            "session-b",
            "Newer title",
            "first message",
            "/tmp/project-b",
            "2026/03/15/rollout-session-b.jsonl",
            120,
            250,
            8,
            true,
        );

        append_history(&fixture, "session-a", 1, "a1");
        append_history(&fixture, "session-b", 2, "b1");
        append_history(&fixture, "session-b", 3, "b2");
        insert_log(&fixture, "session-b", "log-1", 10);
        insert_log(&fixture, "session-b", "log-2", 11);

        let data = list_sessions_with_paths(
            &fixture.paths,
            ListSessionsRequest {
                include_archived: true,
            },
        )
        .unwrap();

        assert_eq!(data.total, 2);
        assert_eq!(data.sessions[0].id, "session-b");
        assert_eq!(data.sessions[0].summary, "first message");
        assert_eq!(data.sessions[0].content_preview, vec!["b1", "b2"]);
        assert_eq!(data.sessions[0].history_count, 2);
        assert_eq!(data.sessions[0].structured_log_count, 2);
        assert!(data.sessions[0].has_rollout);
        assert!(data.sessions[0].has_snapshot);
        assert_eq!(data.sessions[1].summary, "Older title");
    }
}
