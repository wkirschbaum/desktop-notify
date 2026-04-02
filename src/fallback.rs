use crate::{Notifier, SilentNotifier};

pub async fn detect() -> Box<dyn Notifier> {
    tracing::warn!("No notification backend available on this platform");
    Box::new(SilentNotifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn detect_returns_silent() {
        let notifier = detect().await;
        assert_eq!(notifier.name(), "silent");
    }
}
