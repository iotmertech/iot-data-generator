use crate::config::model::{AuthConfig, Config};
use crate::error::{MerError, Result};
use crate::protocols::sender::{OutboundMessage, SendResult, Sender};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;

pub struct HttpSender {
    client: Client,
    url_template: String,
    method: String,
    headers: HashMap<String, String>,
    auth: Option<AuthConfig>,
}

impl HttpSender {
    pub fn from_config(config: &Config) -> Result<Self> {
        let url_template = config
            .target
            .url
            .clone()
            .ok_or_else(|| MerError::Config("target.url is required for HTTP".to_string()))?;

        let method = config
            .target
            .method
            .clone()
            .unwrap_or_else(|| "POST".to_string())
            .to_uppercase();

        let timeout = Duration::from_secs(config.target.timeout_secs.unwrap_or(10));

        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| MerError::Http(format!("Failed to build HTTP client: {}", e)))?;

        let headers = config
            .target
            .headers
            .clone()
            .unwrap_or_default();

        Ok(Self {
            client,
            url_template,
            method,
            headers,
            auth: config.auth.clone(),
        })
    }
}

fn resolve_url(template: &str, device_id: &str) -> String {
    template.replace("{device_id}", device_id)
}

#[async_trait]
impl Sender for HttpSender {
    async fn send(&self, message: &OutboundMessage) -> Result<SendResult> {
        let url = resolve_url(&self.url_template, &message.device_id);
        let bytes_sent = message.payload.len();

        let mut header_map = HeaderMap::new();
        header_map.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        for (k, v) in &self.headers {
            if let (Ok(name), Ok(value)) = (
                k.parse::<HeaderName>(),
                HeaderValue::from_str(v),
            ) {
                header_map.insert(name, value);
            }
        }

        if let Some(auth) = &self.auth {
            match auth {
                AuthConfig::Bearer { token } => {
                    if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                        header_map.insert(AUTHORIZATION, val);
                    }
                }
                AuthConfig::UsernamePassword { username, password } => {
                    use base64::Engine;
                    let encoded = base64::engine::general_purpose::STANDARD
                        .encode(format!("{}:{}", username, password));
                    if let Ok(val) = HeaderValue::from_str(&format!("Basic {}", encoded)) {
                        header_map.insert(AUTHORIZATION, val);
                    }
                }
                AuthConfig::ApiKey { header, value } => {
                    if let (Ok(name), Ok(val)) = (
                        header.parse::<HeaderName>(),
                        HeaderValue::from_str(value),
                    ) {
                        header_map.insert(name, val);
                    }
                }
            }
        }

        let req = match self.method.as_str() {
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "PATCH" => self.client.patch(&url),
            m => {
                return Ok(SendResult {
                    success: false,
                    bytes_sent: 0,
                    error: Some(format!("Unsupported HTTP method: {}", m)),
                })
            }
        };

        let result = req
            .headers(header_map)
            .body(message.payload.clone())
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    Ok(SendResult {
                        success: true,
                        bytes_sent,
                        error: None,
                    })
                } else {
                    Ok(SendResult {
                        success: false,
                        bytes_sent,
                        error: Some(format!("HTTP {} {}", status.as_u16(), status.canonical_reason().unwrap_or(""))),
                    })
                }
            }
            Err(e) => Ok(SendResult {
                success: false,
                bytes_sent: 0,
                error: Some(e.to_string()),
            }),
        }
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
