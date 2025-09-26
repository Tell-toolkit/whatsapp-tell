//! WhatsApp webhook gateway and message handling
//! 
//! This module handles incoming WhatsApp webhooks, validates signatures,
//! processes messages, and manages the conversation state machine.

pub mod webhook;
pub mod signature;
pub mod message_handler;
pub mod consent_flow;

pub use webhook::WebhookHandler;
