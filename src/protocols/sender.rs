use crate::error::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct OutboundMessage {
    #[allow(dead_code)]
    pub topic_or_path: String,
    pub payload: String,
    pub device_id: String,
}

#[derive(Debug, Clone)]
pub struct SendResult {
    pub success: bool,
    pub bytes_sent: usize,
    pub error: Option<String>,
}

#[async_trait]
pub trait Sender: Send + Sync {
    async fn send(&self, message: &OutboundMessage) -> Result<SendResult>;
    async fn close(&self) -> Result<()>;
}
