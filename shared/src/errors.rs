//! Error types for the WhatsApp-Tell service

use thiserror::Error;

/// Main error type for the WhatsApp-Tell service
#[derive(Error, Debug)]
pub enum WhatsAppTellError {
    #[error("AWS service error: {0}")]
    Aws(#[from] aws_sdk_s3::Error),

    #[error("DynamoDB error: {0}")]
    DynamoDb(#[from] aws_sdk_dynamodb::Error),

    #[error("SQS error: {0}")]
    Sqs(#[from] aws_sdk_sqs::Error),

    #[error("Lambda error: {0}")]
    Lambda(#[from] aws_sdk_lambda::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WhatsApp API error: {message}")]
    WhatsAppApi { message: String },

    #[error("Invalid webhook signature")]
    InvalidSignature,

    #[error("Unsupported media format: {format}")]
    UnsupportedMediaFormat { format: String },

    #[error("Media too large: {size} bytes (max: {max_size})")]
    MediaTooLarge { size: u64, max_size: u64 },

    #[error("Consent not granted for operation: {operation}")]
    ConsentNotGranted { operation: String },

    #[error("Invalid conversation state: {current_state} -> {target_state}")]
    InvalidStateTransition {
        current_state: String,
        target_state: String,
    },

    #[error("Audio processing error: {message}")]
    AudioProcessing { message: String },

    #[error("Report generation error: {message}")]
    ReportGeneration { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Result type alias for the service
pub type Result<T> = std::result::Result<T, WhatsAppTellError>;
