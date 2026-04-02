# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`desktop-notify` is a Rust library crate providing cross-platform desktop notifications with three backends:

- **Linux**: D-Bus via `zbus` (`org.freedesktop.Notifications` spec) — supports urgency hints, notification grouping/replacement via `replaces_id`, and click-to-open URLs via `xdg-open`
- **macOS**: `terminal-notifier` (preferred, supports grouping and clickable URLs) with `osascript` fallback (no grouping, URL appended to body)
- **Fallback**: `SilentNotifier` that only logs — used on unsupported platforms and in tests

## Build Commands

```bash
cargo build             # Build
cargo test              # Run tests
cargo fmt               # Format
cargo clippy            # Lint
```

## Architecture

The crate uses compile-time platform selection via `cfg` attributes in `src/lib.rs`:

- `src/lib.rs` — Public API: `Notifier` trait, `Notification` struct, `NotificationLevel` enum, `init()` entry point, `SilentNotifier`
- `src/linux.rs` — D-Bus backend using `zbus` `#[proxy]` macro for the freedesktop Notifications interface
- `src/macos.rs` — `terminal-notifier` CLI and `osascript` fallback backends
- `src/fallback.rs` — Silent no-op for unsupported platforms

Each platform module exposes a `detect() -> Box<dyn Notifier>` function. The public `init()` calls `platform::detect()` and wraps the result in `Arc<dyn Notifier>`.

## Key Design Decisions

- **Rust edition 2024** — uses `let chains` in pattern matching (see `linux.rs` action listener)
- `NotificationLevel::Off` suppresses sending entirely (checked before any OS call)
- Notification grouping: same `group` key replaces the previous notification (D-Bus `replaces_id` / terminal-notifier `-group`)
- All formatting happens in the caller; backends only dispatch to the OS
- D-Bus calls have a 5-second timeout; action listeners time out after 10 minutes
- macOS backends use synchronous `std::process::Command` with background thread reaping (10s timeout)
