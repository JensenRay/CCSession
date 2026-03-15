use crate::{
    delete_service,
    models::{
        ApiResponse, DeleteSessionsData, DeleteSessionsRequest, ListSessionsData,
        ListSessionsRequest,
    },
    session_list,
};

#[tauri::command]
pub fn list_sessions(input: ListSessionsRequest) -> ApiResponse<ListSessionsData> {
    match session_list::list_sessions(input) {
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
