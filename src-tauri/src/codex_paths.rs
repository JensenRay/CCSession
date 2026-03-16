use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::models::{ApiError, ApiErrorCode};

#[derive(Debug, Clone)]
pub struct CodexPaths {
    pub root: PathBuf,
    pub state_db: PathBuf,
    pub logs_db: PathBuf,
    pub history_file: PathBuf,
    pub sessions_root: PathBuf,
    pub shell_snapshots_root: PathBuf,
}

impl CodexPaths {
    pub fn discover() -> Result<Self, ApiError> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            ApiError::new(
                ApiErrorCode::CodexRootNotFound,
                "could not resolve the current user's home directory",
            )
        })?;

        Self::from_root(home_dir.join(".codex"))
    }

    pub fn from_root(root: PathBuf) -> Result<Self, ApiError> {
        if !root.exists() || !root.is_dir() {
            return Err(ApiError::with_details(
                ApiErrorCode::CodexRootNotFound,
                "the local Codex root directory was not found",
                vec![root.display().to_string()],
            ));
        }

        Ok(Self {
            state_db: root.join("state_5.sqlite"),
            logs_db: root.join("logs_1.sqlite"),
            history_file: root.join("history.jsonl"),
            sessions_root: root.join("sessions"),
            shell_snapshots_root: root.join("shell_snapshots"),
            root,
        })
    }

    pub fn validate_rollout_path(&self, rollout_path: &Path) -> Result<PathBuf, String> {
        let normalized_sessions_root = normalize_absolute(&self.sessions_root)
            .map_err(|err| format!("invalid sessions root: {err}"))?;
        let normalized_rollout = normalize_absolute(rollout_path)
            .map_err(|err| format!("invalid rollout path: {err}"))?;

        if !normalized_rollout.starts_with(&normalized_sessions_root) {
            return Err("rollout path is outside ~/.codex/sessions".to_string());
        }

        let file_name = normalized_rollout
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "rollout file name is missing or not valid UTF-8".to_string())?;

        if !file_name.starts_with("rollout-") || !file_name.ends_with(".jsonl") {
            return Err("rollout file name does not match rollout-*.jsonl".to_string());
        }

        if rollout_path.exists() {
            let canonical_sessions_root = fs::canonicalize(&self.sessions_root)
                .map_err(|error| format!("failed to resolve sessions root: {error}"))?;
            let canonical_rollout = fs::canonicalize(rollout_path)
                .map_err(|error| format!("failed to resolve rollout path: {error}"))?;

            if !canonical_rollout.starts_with(&canonical_sessions_root) {
                return Err("rollout path resolves outside ~/.codex/sessions".to_string());
            }
        }

        Ok(normalized_rollout)
    }

    pub fn snapshot_path_for_session(&self, session_id: &str) -> Result<PathBuf, String> {
        if session_id.is_empty() {
            return Err("session id is empty".to_string());
        }

        if session_id.contains('/') || session_id.contains('\\') {
            return Err("session id contains path separators".to_string());
        }

        let normalized_snapshot_root = normalize_absolute(&self.shell_snapshots_root)
            .map_err(|err| format!("invalid shell snapshots root: {err}"))?;
        let snapshot_path = self.shell_snapshots_root.join(format!("{session_id}.sh"));
        let normalized_snapshot = normalize_absolute(&snapshot_path)
            .map_err(|err| format!("invalid snapshot path: {err}"))?;

        if !normalized_snapshot.starts_with(&normalized_snapshot_root) {
            return Err("snapshot path is outside ~/.codex/shell_snapshots".to_string());
        }

        Ok(normalized_snapshot)
    }
}

fn normalize_absolute(path: &Path) -> Result<PathBuf, String> {
    if !path.is_absolute() {
        return Err("path must be absolute".to_string());
    }

    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                return Err("path traversal is not allowed".to_string());
            }
        }
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::CodexPaths;

    #[test]
    fn rejects_rollout_paths_outside_codex_sessions() {
        let temp_dir = tempdir().unwrap();
        let codex_root = temp_dir.path().join(".codex");
        std::fs::create_dir_all(codex_root.join("sessions")).unwrap();
        std::fs::create_dir_all(codex_root.join("shell_snapshots")).unwrap();
        let paths = CodexPaths::from_root(codex_root).unwrap();
        let invalid = temp_dir.path().join("outside/rollout-test.jsonl");

        let result = paths.validate_rollout_path(&invalid);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_snapshot_paths_with_nested_session_ids() {
        let temp_dir = tempdir().unwrap();
        let codex_root = temp_dir.path().join(".codex");
        std::fs::create_dir_all(codex_root.join("sessions")).unwrap();
        std::fs::create_dir_all(codex_root.join("shell_snapshots")).unwrap();
        let paths = CodexPaths::from_root(codex_root).unwrap();

        let result = paths.snapshot_path_for_session("../danger");

        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_rollout_paths_that_escape_via_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = tempdir().unwrap();
        let codex_root = temp_dir.path().join(".codex");
        let sessions_root = codex_root.join("sessions");
        let shell_snapshots_root = codex_root.join("shell_snapshots");
        let outside_root = temp_dir.path().join("outside");

        std::fs::create_dir_all(&sessions_root).unwrap();
        std::fs::create_dir_all(&shell_snapshots_root).unwrap();
        std::fs::create_dir_all(&outside_root).unwrap();

        let escaped_rollout = outside_root.join("rollout-escaped.jsonl");
        std::fs::write(&escaped_rollout, "{}\n").unwrap();

        let link_root = sessions_root.join("2026/03/15");
        std::fs::create_dir_all(link_root.parent().unwrap()).unwrap();
        symlink(&outside_root, &link_root).unwrap();

        let paths = CodexPaths::from_root(codex_root).unwrap();
        let result = paths.validate_rollout_path(&link_root.join("rollout-escaped.jsonl"));

        assert!(result.is_err());
    }
}
