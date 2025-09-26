//! Audio processing orchestration and Lambda invocation
//!
//! This module orchestrates the audio processing pipeline by invoking
//! existing Lambda functions for preprocessing, feature extraction, and aggregation.

pub mod audio_processor;
pub mod lambda_orchestrator;
pub mod media_fetcher;

pub use audio_processor::AudioProcessor;
pub use media_fetcher::MediaFetcher;
