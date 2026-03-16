# CCSession

Codex currently does not provide built-in session cleanup, so I made this small tool.

CCSession cleans session-related data under `~/.codex`, including JSONL files and database records. JSONL files are moved to the system Trash instead of being permanently deleted, so users still have a recovery path if they remove something by mistake.

The goal is simple: keep `~/.codex` clean and easier to manage.

## What It Cleans

- Session records in `state_5.sqlite`
- Structured logs in `logs_1.sqlite`
- Session entries in `history.jsonl`
- Rollout `rollout-*.jsonl` files (move to Trash)
- Shell snapshot files

## Tech Stack

- App: Tauri 2 + TypeScript + Vue3 + Vue Router + Vite
- Local data access: `rusqlite` + standard filesystem APIs
- Trash support: Rust `trash` crate

## Platform Support

- macOS
- Linux

Windows may also work, but macOS and Linux are the primary targets.
