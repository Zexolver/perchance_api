use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PerchanceError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

	// Handles Playwright::initialize()
    #[error("Playwright error: {0}")]
    PlaywrightError(#[from] playwright::Error),

    // Handles browser/page operations
    #[error("Playwright Arc error: {0}")]
    PlaywrightArcError(#[from] Arc<playwright::Error>),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PerchanceError>;
