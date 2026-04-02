use crate::{Notifier, SilentNotifier};

pub async fn detect() -> Box<dyn Notifier> {
    tracing::warn!("No notification backend available on this platform");
    Box::new(SilentNotifier)
}
