//! Tracing and observability configuration

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing with structured logging
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize the subscriber with JSON formatting for structured logs
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    Ok(())
}

/// Initialize tracing for Lambda functions (CloudWatch optimized)
pub fn init_lambda_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // For Lambda, we'll use CloudWatch logs with structured JSON
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_ansi(false)
        .json()
        .init();

    Ok(())
}
