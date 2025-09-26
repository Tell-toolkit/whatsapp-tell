//! Report generation and WhatsApp message formatting
//! 
//! This module generates brief WhatsApp replies and optional detailed PDF reports
//! from neurocognitive marker analysis results.

pub mod report_generator;
pub mod pdf_generator;
pub mod message_formatter;

pub use report_generator::ReportGenerator;
pub use message_formatter::MessageFormatter;
