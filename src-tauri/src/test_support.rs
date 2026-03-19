use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use rusqlite::Connection;
use tempfile::TempDir;

use crate::codex_paths::CodexPaths;

pub struct TestCodexRoot {
    _temp_dir: TempDir,
    pub root: PathBuf,
    pub paths: CodexPaths,
}

pub fn create_test_codex_root() -> TestCodexRoot {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path().join(".codex");
    fs::create_dir_all(root.join("sessions")).unwrap();
    fs::create_dir_all(root.join("shell_snapshots")).unwrap();
    fs::write(root.join("history.jsonl"), "").unwrap();

    let state_connection = Connection::open(root.join("state_5.sqlite")).unwrap();
    state_connection
        .execute_batch(
            "CREATE TABLE threads (
                id TEXT PRIMARY KEY,
                rollout_path TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                source TEXT NOT NULL,
                model_provider TEXT NOT NULL,
                cwd TEXT NOT NULL,
                title TEXT NOT NULL,
                sandbox_policy TEXT NOT NULL,
                approval_mode TEXT NOT NULL,
                tokens_used INTEGER NOT NULL DEFAULT 0,
                has_user_event INTEGER NOT NULL DEFAULT 0,
                archived INTEGER NOT NULL DEFAULT 0,
                archived_at INTEGER,
                git_sha TEXT,
                git_branch TEXT,
                git_origin_url TEXT,
                cli_version TEXT NOT NULL DEFAULT '',
                first_user_message TEXT NOT NULL DEFAULT '',
                agent_nickname TEXT,
                agent_role TEXT,
                memory_mode TEXT NOT NULL DEFAULT 'enabled'
            );
            CREATE TABLE thread_dynamic_tools (
                thread_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                input_schema TEXT NOT NULL,
                PRIMARY KEY(thread_id, position),
                FOREIGN KEY(thread_id) REFERENCES threads(id) ON DELETE CASCADE
            );",
        )
        .unwrap();

    let logs_connection = Connection::open(root.join("logs_1.sqlite")).unwrap();
    logs_connection
        .execute_batch(
            "CREATE TABLE logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts INTEGER NOT NULL,
                ts_nanos INTEGER NOT NULL,
                level TEXT NOT NULL,
                target TEXT NOT NULL,
                message TEXT,
                module_path TEXT,
                file TEXT,
                line INTEGER,
                thread_id TEXT,
                process_uuid TEXT,
                estimated_bytes INTEGER NOT NULL DEFAULT 0
            );",
        )
        .unwrap();

    let paths = CodexPaths::from_root(root.clone()).unwrap();

    TestCodexRoot {
        _temp_dir: temp_dir,
        root,
        paths,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn insert_thread(
    fixture: &TestCodexRoot,
    session_id: &str,
    title: &str,
    first_user_message: &str,
    cwd: &str,
    rollout_relative_path: &str,
    created_at: i64,
    updated_at: i64,
    tokens_used: i64,
    archived: bool,
) {
    let rollout_path = fixture.root.join("sessions").join(rollout_relative_path);
    let connection = Connection::open(&fixture.paths.state_db).unwrap();
    connection
        .execute(
            "INSERT INTO threads (
                id, rollout_path, created_at, updated_at, source, model_provider, cwd, title,
                sandbox_policy, approval_mode, tokens_used, archived, cli_version,
                first_user_message, memory_mode
            ) VALUES (?1, ?2, ?3, ?4, 'cli', 'openai', ?5, ?6, 'workspace-write',
                'default', ?7, ?8, '1.0.0', ?9, 'enabled')",
            rusqlite::params![
                session_id,
                rollout_path.display().to_string(),
                created_at,
                updated_at,
                cwd,
                title,
                tokens_used,
                if archived { 1 } else { 0 },
                first_user_message,
            ],
        )
        .unwrap();
}

pub fn insert_log(fixture: &TestCodexRoot, session_id: &str, message: &str, ts: i64) {
    let connection = Connection::open(&fixture.paths.logs_db).unwrap();
    connection
        .execute(
            "INSERT INTO logs (ts, ts_nanos, level, target, message, thread_id) \
             VALUES (?1, 0, 'INFO', 'test', ?2, ?3)",
            rusqlite::params![ts, message, session_id],
        )
        .unwrap();
}

pub fn append_history(fixture: &TestCodexRoot, session_id: &str, ts: i64, text: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&fixture.paths.history_file)
        .unwrap();
    writeln!(
        file,
        "{{\"session_id\":\"{}\",\"ts\":{},\"text\":\"{}\"}}",
        session_id, ts, text
    )
    .unwrap();
}

pub fn append_raw_history_line(fixture: &TestCodexRoot, line: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&fixture.paths.history_file)
        .unwrap();
    writeln!(file, "{}", line).unwrap();
}

pub fn touch_rollout(fixture: &TestCodexRoot, rollout_relative_path: &str) {
    let path = fixture.root.join("sessions").join(rollout_relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, "{\"type\":\"session_meta\"}\n").unwrap();
}

pub fn touch_snapshot(fixture: &TestCodexRoot, session_id: &str) {
    fs::write(
        fixture
            .paths
            .shell_snapshots_root
            .join(format!("{session_id}.sh")),
        "#!/bin/sh\n",
    )
    .unwrap();
}

pub fn read_history_file(fixture: &TestCodexRoot) -> String {
    fs::read_to_string(&fixture.paths.history_file).unwrap()
}

#[allow(dead_code)]
pub fn exists(path: &Path) -> bool {
    path.exists()
}
