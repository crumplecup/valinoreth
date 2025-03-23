use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initiates a subscriber for the tracing library. Used to instrument internal library functions
/// for debugging and diagnostics.
#[tracing::instrument]
pub fn trace_init() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "combat=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading program...");
}
