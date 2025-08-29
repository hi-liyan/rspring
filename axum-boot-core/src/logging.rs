use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::config::LoggingConfig;

pub fn init_logging(config: &LoggingConfig) -> crate::error::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));
    
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
    
    tracing::info!("Logging initialized with level: {}", config.level);
    
    Ok(())
}