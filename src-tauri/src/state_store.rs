use std::path::PathBuf;

use rusqlite::{Connection, OpenFlags};

use crate::{
    codex_paths::CodexPaths,
    models::{ApiError, ApiErrorCode},
};

#[derive(Debug, Clone)]
pub struct ThreadRow {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub source: String,
    pub model_provider: String,
    pub cwd: String,
    pub title: String,
    pub first_user_message: String,
    pub tokens_used: i64,
    pub archived: bool,
}

#[derive(Debug, Clone)]
pub struct StateDeleteRow {
    pub session_id: String,
    pub rollout_path: PathBuf,
}

pub fn open_state_db_read_only(paths: &CodexPaths) -> Result<Connection, ApiError> {
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

pub fn open_state_db_read_write(paths: &CodexPaths) -> Result<Connection, ApiError> {
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

pub fn load_threads(
    connection: &Connection,
    include_archived: bool,
) -> Result<Vec<ThreadRow>, ApiError> {
    let sql = if include_archived {
        "SELECT id, created_at, updated_at, source, model_provider, cwd, title, \
         first_user_message, tokens_used, archived \
         FROM threads \
         ORDER BY updated_at DESC, id DESC"
    } else {
        "SELECT id, created_at, updated_at, source, model_provider, cwd, title, \
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
            let archived: i64 = row.get(9)?;
            Ok(ThreadRow {
                id: row.get(0)?,
                created_at: row.get(1)?,
                updated_at: row.get(2)?,
                source: row.get(3)?,
                model_provider: row.get(4)?,
                cwd: row.get(5)?,
                title: row.get(6)?,
                first_user_message: row.get(7)?,
                tokens_used: row.get(8)?,
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

    collect_rows(rows, "failed to decode a thread row")
}

pub fn load_delete_rows(
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

    collect_rows(rows, "failed to decode a delete lookup row")
}

pub fn delete_thread(connection: &Connection, session_id: &str) -> Result<bool, String> {
    connection
        .execute(
            "DELETE FROM threads WHERE id = ?1",
            rusqlite::params![session_id],
        )
        .map(|changed_rows| changed_rows > 0)
        .map_err(|error| format!("failed to delete state row: {}", error))
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
    decode_error_message: &str,
) -> Result<Vec<T>, ApiError> {
    let mut collected_rows = Vec::new();

    for row in rows {
        collected_rows.push(row.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::StateDbQueryFailed,
                decode_error_message,
                vec![error.to_string()],
            )
        })?);
    }

    Ok(collected_rows)
}
