use crate::{
    codex_paths::CodexPaths,
    history_store,
    models::{ApiError, ApiErrorCode, SessionPromptsData, SessionPromptsRequest},
};

pub fn session_prompts(input: SessionPromptsRequest) -> Result<SessionPromptsData, ApiError> {
    let paths = CodexPaths::discover()?;
    session_prompts_with_paths(&paths, input)
}

pub(crate) fn session_prompts_with_paths(
    paths: &CodexPaths,
    input: SessionPromptsRequest,
) -> Result<SessionPromptsData, ApiError> {
    let session_id = input.session_id.trim();
    if session_id.is_empty() {
        return Err(ApiError::new(
            ApiErrorCode::InvalidInput,
            "sessionId must not be empty",
        ));
    }

    let history = history_store::load_session_prompts(&paths.history_file, session_id)?;

    Ok(SessionPromptsData {
        session_id: session_id.to_string(),
        prompts: history.prompts,
        warnings: history.warnings,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        models::SessionPromptsRequest,
        test_support::{append_history, create_test_codex_root},
    };

    use super::session_prompts_with_paths;

    #[test]
    fn returns_all_prompts_for_a_single_session_in_order() {
        let fixture = create_test_codex_root();

        append_history(&fixture, "session-a", 3, "third");
        append_history(&fixture, "session-b", 2, "other");
        append_history(&fixture, "session-a", 1, "first");
        append_history(&fixture, "session-a", 2, "second");

        let data = session_prompts_with_paths(
            &fixture.paths,
            SessionPromptsRequest {
                session_id: "session-a".to_string(),
            },
        )
        .unwrap();

        assert_eq!(data.session_id, "session-a");
        assert_eq!(data.prompts, vec!["first", "second", "third"]);
    }
}
