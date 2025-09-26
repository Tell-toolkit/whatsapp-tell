//! WhatsApp webhook signature validation

use hmac::{Hmac, Mac};
use sha2::Sha256;
use shared::errors::{Result, WhatsAppTellError};

type HmacSha256 = Hmac<Sha256>;

/// Validates WhatsApp webhook signature
pub struct SignatureValidator {
    app_secret: String,
}

impl SignatureValidator {
    pub fn new(app_secret: String) -> Self {
        Self { app_secret }
    }

    /// Validates the webhook signature
    pub fn validate_signature(&self, payload: &str, signature: &str) -> Result<()> {
        let mut mac = HmacSha256::new_from_slice(self.app_secret.as_bytes())
            .map_err(|_| WhatsAppTellError::InvalidSignature)?;

        mac.update(payload.as_bytes());

        let expected_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        if signature == expected_signature {
            Ok(())
        } else {
            Err(Box::new(WhatsAppTellError::InvalidSignature))
        }
    }
}
