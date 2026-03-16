mod codex_paths;
mod commands;
mod delete_service;
mod history_store;
mod log_store;
mod models;
mod session_detail;
mod session_list;
#[cfg(test)]
mod test_support;

type ListSessionsCommand =
    fn(models::ListSessionsRequest) -> models::ApiResponse<models::ListSessionsData>;
type SessionPromptsCommand =
    fn(models::SessionPromptsRequest) -> models::ApiResponse<models::SessionPromptsData>;
type DeleteSessionsCommand =
    fn(models::DeleteSessionsRequest) -> models::ApiResponse<models::DeleteSessionsData>;

// Keep rust-analyzer aware that these command functions are used via Tauri's macro registration.
const _: ListSessionsCommand = commands::list_sessions;
const _: SessionPromptsCommand = commands::session_prompts;
const _: DeleteSessionsCommand = commands::delete_sessions;

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
