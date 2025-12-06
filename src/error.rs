//! Error types for MISP API operations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MispError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("API returned error: {status} {message}")]
    Api { status: u16, message: String },

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Attribute not found: {0}")]
    AttributeNotFound(String),

    #[error("Invalid response structure: {0}")]
    InvalidResponse(String),
}
