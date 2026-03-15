use crate::{
    models::{ApiResponse, ListSessionsData, ListSessionsRequest},
    session_list,
};

#[tauri::command]
pub fn list_sessions(input: ListSessionsRequest) -> ApiResponse<ListSessionsData> {
    match session_list::list_sessions(input) {
        Ok(data) => ApiResponse::ok(data),
        Err(error) => ApiResponse::err(error),
    }
}
