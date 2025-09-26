//! Shared types and utilities for WhatsApp-Tell service
//! 
//! This module contains common data structures, error types, and AWS client
//! configurations used across all service modules.

pub mod types;
pub mod errors;
pub mod aws;
pub mod tracing;

pub use errors::{WhatsAppTellError, Result};
