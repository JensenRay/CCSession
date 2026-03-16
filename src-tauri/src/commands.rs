use crate::{
    delete_service,
    models::{
        ApiResponse, DeleteSessionsData, DeleteSessionsRequest, ListSessionsData,
        ListSessionsRequest, SessionPromptsData, SessionPromptsRequest,
    },
    session_detail, session_list,
};

#[tauri::command]
pub fn list_sessions(input: ListSessionsRequest) -> ApiResponse<ListSessionsData> {
    match session_list::list_sessions(input) {
        Ok(data) => ApiResponse::ok(data),
        Err(error) => ApiResponse::err(error),
    }
}

#[tauri::command]
pub fn session_prompts(input: SessionPromptsRequest) -> ApiResponse<SessionPromptsData> {
    match session_detail::session_prompts(input) {
        Ok(data) => ApiResponse::ok(data),
        Err(error) => ApiResponse::err(error),
    }
}

#[tauri::command]
pub fn delete_sessions(input: DeleteSessionsRequest) -> ApiResponse<DeleteSessionsData> {
    match delete_service::delete_sessions(input) {
        Ok(data) => ApiResponse::ok(data),
        Err(error) => ApiResponse::err(error),
    }
}
