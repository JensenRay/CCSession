use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
    pub details: Vec<String>,
}

impl ApiError {
    pub fn new(code: ApiErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: Vec::new(),
        }
    }

    pub fn with_details(
        code: ApiErrorCode,
        message: impl Into<String>,
        details: Vec<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            details,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    CodexRootNotFound,
    StateDbOpenFailed,
    StateDbQueryFailed,
    LogsDbOpenFailed,
    HistoryFileReadFailed,
    HistoryFileWriteFailed,
    CodexRunningDetected,
    InvalidInput,
    DeleteExecutionFailed,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { ok: bool, data: T },
    Failure { ok: bool, error: ApiError },
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::Success { ok: true, data }
    }

    pub fn err(error: ApiError) -> Self {
        Self::Failure { ok: false, error }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsRequest {
    #[serde(default = "default_true")]
    pub include_archived: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSessionsRequest {
    pub session_ids: Vec<String>,
    #[serde(default = "default_true")]
    pub require_codex_stopped: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsData {
    pub sessions: Vec<SessionListItem>,
    pub total: usize,
    pub scanned_at: i64,
    pub codex_root: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionListItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub content_preview: Vec<String>,
    pub cwd: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub tokens_used: i64,
    pub archived: bool,
    pub source: String,
    pub model_provider: String,
    pub has_rollout: bool,
    pub has_snapshot: bool,
    pub history_count: usize,
    pub structured_log_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSessionsData {
    pub requested_count: usize,
    pub deleted_count: usize,
    pub partial_failure_count: usize,
    pub failed_count: usize,
    pub not_found_count: usize,
    pub reports: Vec<SessionDeleteReport>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDeleteReport {
    pub session_id: String,
    pub status: SessionDeleteStatus,
    pub deleted_state_row: bool,
    pub deleted_history_entries: usize,
    pub deleted_structured_log_rows: usize,
    pub deleted_rollout_file: bool,
    pub deleted_snapshot_file: bool,
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDeleteStatus {
    Deleted,
    PartialFailure,
    Failed,
    NotFound,
}

impl SessionDeleteReport {
    pub fn not_found(session_id: String) -> Self {
        Self {
            session_id,
            status: SessionDeleteStatus::NotFound,
            deleted_state_row: false,
            deleted_history_entries: 0,
            deleted_structured_log_rows: 0,
            deleted_rollout_file: false,
            deleted_snapshot_file: false,
            warnings: Vec::new(),
            error: None,
        }
    }
}

impl Default for DeleteSessionsRequest {
    fn default() -> Self {
        Self {
            session_ids: Vec::new(),
            require_codex_stopped: true,
        }
    }
}

pub fn default_true() -> bool {
    true
}
