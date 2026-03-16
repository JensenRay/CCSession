mod codex_paths;
mod commands;
mod delete_service;
mod history_store;
mod log_store;
mod models;
mod session_detail;
mod session_list;
mod state_store;
#[cfg(test)]
mod test_support;

type ListSessionsCommand =
    fn(models::ListSessionsRequest) -> models::ApiResponse<models::ListSessionsData>;
type SessionPromptsCommand =
    fn(models::SessionPromptsRequest) -> models::ApiResponse<models::SessionPromptsData>;
type DeleteSessionsCommand =
    fn(models::DeleteSessionsRequest) -> models::ApiResponse<models::DeleteSessionsData>;
type OpenStateDbReadOnlyFn =
    fn(&codex_paths::CodexPaths) -> Result<rusqlite::Connection, models::ApiError>;
type OpenStateDbReadWriteFn =
    fn(&codex_paths::CodexPaths) -> Result<rusqlite::Connection, models::ApiError>;
type LoadThreadsFn =
    fn(&rusqlite::Connection, bool) -> Result<Vec<state_store::ThreadRow>, models::ApiError>;
type LoadDeleteRowsFn = fn(
    &rusqlite::Connection,
    &[String],
) -> Result<Vec<state_store::StateDeleteRow>, models::ApiError>;
type DeleteThreadFn = fn(&rusqlite::Connection, &str) -> Result<bool, String>;

// Keep rust-analyzer aware that these command functions are used via Tauri's macro registration.
const _: ListSessionsCommand = commands::list_sessions;
const _: SessionPromptsCommand = commands::session_prompts;
const _: DeleteSessionsCommand = commands::delete_sessions;
// Keep rust-analyzer aware of state-store helpers that are only reached through command-driven flows.
const _: OpenStateDbReadOnlyFn = state_store::open_state_db_read_only;
const _: OpenStateDbReadWriteFn = state_store::open_state_db_read_write;
const _: LoadThreadsFn = state_store::load_threads;
const _: LoadDeleteRowsFn = state_store::load_delete_rows;
const _: DeleteThreadFn = state_store::delete_thread;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[cfg(not(test))]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::list_sessions,
            commands::session_prompts,
            commands::delete_sessions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
pub fn run() {}
