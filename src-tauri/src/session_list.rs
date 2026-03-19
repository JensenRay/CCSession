use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    codex_paths::CodexPaths,
    history_store::{self, HistoryAggregation},
    models::{ApiError, ListSessionsData, ListSessionsRequest, SessionListItem},
    state_store::{self, ThreadRow},
};

const CONTENT_PREVIEW_LIMIT: usize = 12;

pub fn list_sessions(input: ListSessionsRequest) -> Result<ListSessionsData, ApiError> {
    let paths = CodexPaths::discover()?;
    list_sessions_with_paths(&paths, input)
}

pub(crate) fn list_sessions_with_paths(
    paths: &CodexPaths,
    input: ListSessionsRequest,
) -> Result<ListSessionsData, ApiError> {
    let state_connection = state_store::open_state_db_read_only(paths)?;
    let threads = state_store::load_threads(&state_connection, input.include_archived)?;
    let history = history_store::scan_history_previews(&paths.history_file, CONTENT_PREVIEW_LIMIT)?;

    build_list_response(paths, threads, history)
}

fn build_list_response(
    paths: &CodexPaths,
    threads: Vec<ThreadRow>,
    history: HistoryAggregation,
) -> Result<ListSessionsData, ApiError> {
    let warnings = history.warnings;
    let mut sessions = Vec::with_capacity(threads.len());

    for thread in threads {
        let history_summary = history.summaries.get(&thread.id);
        let summary = if thread.first_user_message.trim().is_empty() {
            thread.title.clone()
        } else {
            thread.first_user_message.clone()
        };

        sessions.push(SessionListItem {
            id: thread.id.clone(),
            summary,
            content_preview: history_summary
                .map(|summary| summary.preview())
                .unwrap_or_default(),
            cwd: thread.cwd,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            tokens_used: thread.tokens_used,
            archived: thread.archived,
            source: thread.source,
            model_provider: thread.model_provider,
        });
    }

    Ok(ListSessionsData {
        total: sessions.len(),
        scanned_at: unix_timestamp_now(),
        codex_root: paths.root.display().to_string(),
        warnings,
        sessions,
    })
}

fn unix_timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use crate::{
        models::ListSessionsRequest,
        test_support::{
            append_history, append_raw_history_line, create_test_codex_root, insert_thread,
        },
    };

    use super::list_sessions_with_paths;

    #[test]
    fn builds_sorted_session_list_with_history_preview() {
        let fixture = create_test_codex_root();

        insert_thread(
            &fixture,
            "session-a",
            "Older title",
            "",
            "/tmp/project-a",
            "2026/03/14/rollout-session-a.jsonl",
            100,
            150,
            3,
            false,
        );
        insert_thread(
            &fixture,
            "session-b",
            "Newer title",
            "first message",
            "/tmp/project-b",
            "2026/03/15/rollout-session-b.jsonl",
            120,
            250,
            8,
            true,
        );

        append_history(&fixture, "session-a", 1, "a1");
        append_history(&fixture, "session-b", 2, "b1");
        append_history(&fixture, "session-b", 3, "b2");

        let data = list_sessions_with_paths(
            &fixture.paths,
            ListSessionsRequest {
                include_archived: true,
            },
        )
        .unwrap();

        assert_eq!(data.total, 2);
        assert_eq!(data.sessions[0].id, "session-b");
        assert_eq!(data.sessions[0].summary, "first message");
        assert_eq!(data.sessions[0].content_preview, vec!["b1", "b2"]);
        assert_eq!(data.sessions[1].summary, "Older title");
    }

    #[test]
    fn keeps_the_most_recent_twelve_preview_entries() {
        let fixture = create_test_codex_root();

        insert_thread(
            &fixture,
            "session-a",
            "Preview title",
            "first message",
            "/tmp/project-a",
            "2026/03/15/rollout-session-a.jsonl",
            110,
            240,
            4,
            false,
        );

        for index in 0..15 {
            append_history(
                &fixture,
                "session-a",
                index as i64,
                &format!("line-{index}"),
            );
        }

        let data = list_sessions_with_paths(
            &fixture.paths,
            ListSessionsRequest {
                include_archived: true,
            },
        )
        .unwrap();

        assert_eq!(
            data.sessions[0].content_preview,
            vec![
                "line-3", "line-4", "line-5", "line-6", "line-7", "line-8", "line-9", "line-10",
                "line-11", "line-12", "line-13", "line-14"
            ]
        );
    }

    #[test]
    fn skips_malformed_history_lines_but_returns_warnings() {
        let fixture = create_test_codex_root();

        insert_thread(
            &fixture,
            "session-a",
            "Preview title",
            "first message",
            "/tmp/project-a",
            "2026/03/15/rollout-session-a.jsonl",
            110,
            240,
            4,
            false,
        );

        append_history(&fixture, "session-a", 1, "before");
        append_raw_history_line(&fixture, "{\"session_id\":\"session-a\"");
        append_history(&fixture, "session-a", 2, "after");

        let data = list_sessions_with_paths(
            &fixture.paths,
            ListSessionsRequest {
                include_archived: true,
            },
        )
        .unwrap();

        assert_eq!(data.sessions[0].content_preview, vec!["before", "after"]);
        assert_eq!(data.warnings.len(), 1);
    }
}
