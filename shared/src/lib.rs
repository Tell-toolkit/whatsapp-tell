//! Shared types and utilities for WhatsApp-Tell service
//!
//! This module contains common data structures, error types, and AWS client
//! configurations used across all service modules.

pub mod aws;
pub mod errors;
pub mod tracing;
pub mod types;

pub use errors::{Result, WhatsAppTellError};
