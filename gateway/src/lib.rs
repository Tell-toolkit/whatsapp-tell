//! WhatsApp webhook gateway and message handling
//!
//! This module handles incoming WhatsApp webhooks, validates signatures,
//! processes messages, and manages the conversation state machine.

pub mod consent_flow;
pub mod message_handler;
pub mod signature;
pub mod webhook;

pub use webhook::WebhookHandler;
