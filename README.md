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

## Install

If you're using Homebrew on macOS, you can install it with:

```bash
brew tap jensenray/toys
brew install --cask ccsession
```

Other installation methods can be found on the Releases page.

## Uninstall

```bash
brew uninstall --cask --zap ccsession
brew untap jensenray/toys
```

## Unsigned Builds

All distributed binaries are unsigned.

If you download and run a prebuilt release, especially on macOS, you must remove the quarantine attribute first:

```bash
xattr -dr com.apple.quarantine /Applications/CCSession.app
```

Then use **Run Anyway** in macOS Privacy & Security if Gatekeeper still blocks the app.

If you do not trust the prebuilt binaries, the best option is to build the app yourself from source.

## Build From Source

### Requirements

- [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies)
- [Rust](https://v2.tauri.app/start/prerequisites/#rust)
- [Bun](https://bun.com)

### Clone Project

```bash
git clone https://github.com/JensenRay/CCSession.git
cd CCSession
```

and

```bash
bun install
```

### Development

```bash
bun tauri dev
```

### Production Build

```bash
bun tauri build
```

The generated application bundles and installers will be placed under `src-tauri/target/release/bundle/`.

## Platform Support

- macOS
- Linux
- Windows

The Windows and Linux releases have not been tested in a wide range of real-world environments yet. I'd appreciate feedback.
