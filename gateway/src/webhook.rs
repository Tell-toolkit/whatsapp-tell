//! WhatsApp webhook handling

use lambda_http::{run, service_fn, Error, Request, Response};
use tracing::info;

/// Webhook handler for incoming WhatsApp messages
pub struct WebhookHandler {
    // TODO: Add webhook configuration
}

impl WebhookHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle_webhook(&self, request: Request) -> Result<Response<String>, Error> {
        info!("Received webhook request");

        // TODO: Implement webhook processing
        Ok(Response::builder()
            .status(200)
            .body("OK".to_string())
            .unwrap())
    }
}

/// Lambda handler function
pub async fn handler(request: Request) -> Result<Response<String>, Error> {
    let webhook_handler = WebhookHandler::new();
    webhook_handler.handle_webhook(request).await
}
