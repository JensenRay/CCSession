use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;

use crate::models::{ApiError, ApiErrorCode};

#[derive(Debug, Default, Clone)]
pub struct HistorySummary {
    pub count: usize,
    preview_entries: Vec<PreviewEntry>,
}

impl HistorySummary {
    fn push(&mut self, entry: PreviewEntry, preview_limit: usize) {
        self.count += 1;
        self.preview_entries.push(entry);
        self.preview_entries
            .sort_by_key(|item| (item.ts, item.order));

        while self.preview_entries.len() > preview_limit {
            self.preview_entries.remove(0);
        }
    }

    pub fn preview(&self) -> Vec<String> {
        self.preview_entries
            .iter()
            .map(|entry| entry.text.clone())
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct HistoryAggregation {
    pub summaries: HashMap<String, HistorySummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoadedHistoryFile {
    lines: Vec<HistoryLine>,
    pub warnings: Vec<String>,
}

impl LoadedHistoryFile {
    pub fn rewrite_without(
        &self,
        history_path: &Path,
        session_ids_to_remove: &HashSet<String>,
    ) -> Result<HashMap<String, usize>, ApiError> {
        if session_ids_to_remove.is_empty() {
            return Ok(HashMap::new());
        }

        let temp_path = history_path.with_extension(format!(
            "jsonl.tmp-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
                .map_err(|error| {
                    ApiError::with_details(
                        ApiErrorCode::HistoryFileWriteFailed,
                        "failed to create the temporary history file",
                        vec![error.to_string(), temp_path.display().to_string()],
                    )
                })?,
        );

        let mut deleted_counts = HashMap::new();
        let mut write_result = Ok(());

        for line in &self.lines {
            let should_remove = line
                .session_id
                .as_ref()
                .map(|session_id| session_ids_to_remove.contains(session_id))
                .unwrap_or(false);

            if should_remove {
                if let Some(session_id) = &line.session_id {
                    *deleted_counts.entry(session_id.clone()).or_insert(0) += 1;
                }
                continue;
            }

            if write_result.is_ok() {
                write_result = writer
                    .write_all(line.raw.as_bytes())
                    .and_then(|_| writer.write_all(b"\n"))
                    .map_err(|error| {
                        ApiError::with_details(
                            ApiErrorCode::HistoryFileWriteFailed,
                            "failed while rewriting history.jsonl",
                            vec![error.to_string(), history_path.display().to_string()],
                        )
                    });
            }
        }

        if write_result.is_ok() {
            write_result = writer.flush().map_err(|error| {
                ApiError::with_details(
                    ApiErrorCode::HistoryFileWriteFailed,
                    "failed to flush the rewritten history.jsonl",
                    vec![error.to_string(), history_path.display().to_string()],
                )
            });
        }

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temp_path);
            return Err(error);
        }

        drop(writer);

        fs::rename(&temp_path, history_path).map_err(|error| {
            let _ = fs::remove_file(&temp_path);
            ApiError::with_details(
                ApiErrorCode::HistoryFileWriteFailed,
                "failed to replace history.jsonl with the rewritten file",
                vec![error.to_string(), history_path.display().to_string()],
            )
        })?;

        Ok(deleted_counts)
    }
}

#[derive(Debug, Clone)]
struct HistoryLine {
    raw: String,
    session_id: Option<String>,
}

#[derive(Debug, Clone)]
struct PreviewEntry {
    ts: i64,
    order: usize,
    text: String,
}

#[derive(Debug, Deserialize)]
struct HistoryRecord {
    session_id: String,
    #[serde(default)]
    ts: i64,
    text: String,
}

pub fn scan_history_previews(
    history_path: &Path,
    preview_limit: usize,
) -> Result<HistoryAggregation, ApiError> {
    let file = File::open(history_path).map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::HistoryFileReadFailed,
            "failed to open history.jsonl",
            vec![error.to_string(), history_path.display().to_string()],
        )
    })?;
    let reader = BufReader::new(file);

    let mut summaries = HashMap::<String, HistorySummary>::new();
    let mut warnings = Vec::new();

    for (line_index, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::HistoryFileReadFailed,
                "failed to read history.jsonl",
                vec![error.to_string(), history_path.display().to_string()],
            )
        })?;

        match serde_json::from_str::<HistoryRecord>(&line) {
            Ok(record) => {
                let summary = summaries.entry(record.session_id).or_default();
                summary.push(
                    PreviewEntry {
                        ts: record.ts,
                        order: line_index,
                        text: record.text,
                    },
                    preview_limit,
                );
            }
            Err(error) => warnings.push(format!(
                "skipped malformed history line {}: {}",
                line_index + 1,
                error
            )),
        }
    }

    Ok(HistoryAggregation {
        summaries,
        warnings,
    })
}

pub fn load_history_file(history_path: &Path) -> Result<LoadedHistoryFile, ApiError> {
    let file = File::open(history_path).map_err(|error| {
        ApiError::with_details(
            ApiErrorCode::HistoryFileReadFailed,
            "failed to open history.jsonl",
            vec![error.to_string(), history_path.display().to_string()],
        )
    })?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut warnings = Vec::new();

    for (line_index, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|error| {
            ApiError::with_details(
                ApiErrorCode::HistoryFileReadFailed,
                "failed to read history.jsonl",
                vec![error.to_string(), history_path.display().to_string()],
            )
        })?;

        let session_id = match serde_json::from_str::<HistoryRecord>(&line) {
            Ok(record) => Some(record.session_id),
            Err(error) => {
                warnings.push(format!(
                    "preserved malformed history line {}: {}",
                    line_index + 1,
                    error
                ));
                None
            }
        };

        lines.push(HistoryLine {
            raw: line,
            session_id,
        });
    }

    Ok(LoadedHistoryFile { lines, warnings })
}
