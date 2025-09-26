//! Core data types for the WhatsApp-Tell service

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// WhatsApp message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MessageType {
    Text { body: String },
    Audio { 
        id: String,
        mime_type: String,
        sha256: String,
        filename: Option<String>
    },
    Document {
        id: String,
        mime_type: String,
        sha256: String,
        filename: String
    },
    Image {
        id: String,
        mime_type: String,
        sha256: String,
        caption: Option<String>
    }
}

/// WhatsApp message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub conversation_id: String,
}

/// Consent types for different processing modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Consent {
    pub id: Uuid,
    pub conversation_id: String,
    pub user_phone: String,
    pub audio_processing: bool,
    pub asr_processing: Option<bool>,
    pub research_mode: Option<bool>,
    pub retention_days: u32,
    pub granted_at: DateTime<Utc>,
    pub version: String, // Privacy policy version
}

/// Neurocognitive markers extracted from audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerSet {
    pub id: Uuid,
    pub conversation_id: String,
    pub message_id: String,
    pub audio_s3_key: String,
    
    // Temporal markers
    pub speech_rate: Option<f64>, // syllables per second
    pub articulation_rate: Option<f64>,
    pub phonation_time: Option<f64>, // percentage
    pub pause_frequency: Option<f64>, // pauses per minute
    pub average_pause_length: Option<f64>, // seconds
    
    // Prosodic markers
    pub f0_mean: Option<f64>, // Hz
    pub f0_variance: Option<f64>,
    pub pitch_range: Option<f64>,
    pub intensity_dynamics: Option<f64>,
    
    // Fluency markers
    pub disfluency_rate: Option<f64>, // per minute
    pub repetition_count: Option<u32>,
    pub repair_count: Option<u32>,
    
    // Lexical markers (if ASR enabled)
    pub type_token_ratio: Option<f64>,
    pub sentiment_polarity: Option<f64>, // -1 to 1
    
    pub confidence_score: f64, // 0.0 to 1.0
    pub processing_duration_ms: u64,
    pub created_at: DateTime<Utc>,
}

/// Conversation state machine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationState {
    WaitingIntent,
    ConsentRequested,
    ConsentGranted,
    ConsentDenied,
    IntakeReady,
    MediaPending,
    MediaDownloaded,
    Processing,
    ReportReady,
    DeliverReport,
    AskFollowup,
}

/// Conversation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub user_phone: String,
    pub state: ConversationState,
    pub last_transition: DateTime<Utc>,
    pub transition_reason: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Audit event for compliance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub conversation_id: String,
    pub event_type: String,
    pub actor: String, // "user", "system", "admin"
    pub timestamp: DateTime<Utc>,
    pub payload_hash: String, // SHA256 of sensitive data
    pub metadata: serde_json::Value,
}
