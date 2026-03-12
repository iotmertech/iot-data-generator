use crate::config::model::Config;
use crate::error::{MerError, Result};
use crate::protocols::sender::{OutboundMessage, SendResult, Sender};
use async_trait::async_trait;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct TcpSender {
    host: String,
    port: u16,
    line_delimiter: bool,
    timeout: Duration,
}

impl TcpSender {
    pub fn from_config(config: &Config) -> Result<Self> {
        let host = config
            .target
            .host
            .clone()
            .ok_or_else(|| MerError::Config("target.host is required for TCP".to_string()))?;
        let port = config
            .target
            .port
            .ok_or_else(|| MerError::Config("target.port is required for TCP".to_string()))?;
        let timeout = Duration::from_secs(config.target.timeout_secs.unwrap_or(5));
        let line_delimiter = config.target.line_delimiter.unwrap_or(true);

        Ok(Self {
            host,
            port,
            line_delimiter,
            timeout,
        })
    }
}

#[async_trait]
impl Sender for TcpSender {
    async fn send(&self, message: &OutboundMessage) -> Result<SendResult> {
        let addr = format!("{}:{}", self.host, self.port);
        let connect = tokio::time::timeout(self.timeout, TcpStream::connect(&addr)).await;

        let mut stream = match connect {
            Ok(Ok(s)) => s,
            Ok(Err(e)) => {
                return Ok(SendResult {
                    success: false,
                    bytes_sent: 0,
                    error: Some(format!("TCP connection failed to {}: {}", addr, e)),
                })
            }
            Err(_) => {
                return Ok(SendResult {
                    success: false,
                    bytes_sent: 0,
                    error: Some(format!("TCP connection timed out to {}", addr)),
                })
            }
        };

        let mut data = message.payload.as_bytes().to_vec();
        if self.line_delimiter {
            data.push(b'\n');
        }
        let bytes_sent = data.len();

        match stream.write_all(&data).await {
            Ok(()) => Ok(SendResult {
                success: true,
                bytes_sent,
                error: None,
            }),
            Err(e) => Ok(SendResult {
                success: false,
                bytes_sent: 0,
                error: Some(format!("TCP write failed: {}", e)),
            }),
        }
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
