use rusqlite::{Connection, OpenFlags};

use crate::{
    codex_paths::CodexPaths,
    models::{ApiError, ApiErrorCode},
};

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
