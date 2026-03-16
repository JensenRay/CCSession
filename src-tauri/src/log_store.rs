use std::collections::HashMap;

use rusqlite::{Connection, OpenFlags};

use crate::{
    codex_paths::CodexPaths,
    models::{ApiError, ApiErrorCode},
};

pub fn open_logs_db_read_only(paths: &CodexPaths) -> Result<Connection, ApiError> {
    Connection::open_with_flags(&paths.logs_db, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::LogsDbOpenFailed,
            "failed to open logs_1.sqlite",
            vec![error.to_string(), paths.logs_db.display().to_string()],
        )
    })
}

pub fn open_logs_db_read_write(paths: &CodexPaths) -> Result<Connection, ApiError> {
    Connection::open_with_flags(
        &paths.logs_db,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI,
    )
    .map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::LogsDbOpenFailed,
            "failed to open logs_1.sqlite for deletion",
            vec![error.to_string(), paths.logs_db.display().to_string()],
        )
    })
}

pub fn load_log_counts(connection: &Connection) -> Result<HashMap<String, usize>, ApiError> {
    let mut statement = connection
        .prepare(
            "SELECT thread_id, COUNT(*) \
             FROM logs \
             WHERE thread_id IS NOT NULL \
             GROUP BY thread_id",
        )
        .map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::LogsDbQueryFailed,
                "failed to query structured log counts",
                vec![error.to_string()],
            )
        })?;

    let rows = statement
        .query_map([], |row| {
            let thread_id: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((thread_id, count.max(0) as usize))
        })
        .map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::LogsDbQueryFailed,
                "failed to scan structured log counts",
                vec![error.to_string()],
            )
        })?;

    let mut counts = HashMap::new();
    for row in rows {
        let (thread_id, count) = row.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::LogsDbQueryFailed,
                "failed to decode structured log count row",
                vec![error.to_string()],
            )
        })?;
        counts.insert(thread_id, count);
    }

    Ok(counts)
}
