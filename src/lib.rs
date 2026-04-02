//! Cross-platform desktop notifications.
//!
//! Provides a [`Notifier`] trait with platform backends:
//! - **Linux**: D-Bus via `org.freedesktop.Notifications` (falls back to silent if unavailable)
//! - **macOS**: `terminal-notifier` (preferred) or `osascript` (fallback)

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod fallback;
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
use fallback as platform;

/// Notification urgency level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    /// Suppressed entirely — the notification is not sent.
    Off,
    /// Low priority — short timeout, subtle presentation.
    Low,
    /// Normal priority — standard timeout and presentation.
    Normal,
    /// Critical — persistent, bypasses quiet hours / DND on some platforms.
    Critical,
}

/// Pre-computed notification data. All formatting happens before this is
/// passed to the platform backend — the backend only does the OS dispatch.
pub struct Notification {
    pub title: String,
    pub body: String,
    pub level: NotificationLevel,
    /// Optional URL to open when the notification is clicked.
    pub url: Option<String>,
    /// Grouping key — notifications with the same group replace each other.
    pub group: String,
    /// Human-readable source identifier shown in the OS notification chrome.
    pub app_name: String,
}

/// Platform notification backend.
pub trait Notifier: Send + Sync {
    /// Backend name for diagnostics (e.g. "dbus", "terminal-notifier").
    fn name(&self) -> &'static str;

    /// Send a notification. Implementations should not block the caller on
    /// user interaction (e.g. clicking the notification).
    fn send(&self, n: &Notification) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

/// Detect and initialize the best available notification backend for this platform.
pub async fn init() -> Arc<dyn Notifier> {
    let n = platform::detect().await;
    tracing::info!("Using notification backend: {}", n.name());
    Arc::from(n)
}

/// Silent fallback notifier — logs at debug level, sends nothing.
/// Also useful in tests.
pub struct SilentNotifier;

impl Notifier for SilentNotifier {
    fn name(&self) -> &'static str {
        "silent"
    }

    fn send(&self, n: &Notification) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let title = n.title.clone();
        Box::pin(async move {
            tracing::debug!(title = %title, "Notification suppressed (no backend)");
        })
    }
}
