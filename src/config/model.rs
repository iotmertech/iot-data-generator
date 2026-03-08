use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub protocol: Protocol,
    pub target: Target,
    pub device: DeviceConfig,
    pub payload: PayloadConfig,
    pub run: RunConfig,
    #[serde(default)]
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Mqtt,
    Http,
    Tcp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Mqtt => write!(f, "mqtt"),
            Protocol::Http => write!(f, "http"),
            Protocol::Tcp => write!(f, "tcp"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Target {
    // MQTT
    #[serde(default)]
    pub broker: Option<String>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub qos: Option<u8>,
    #[serde(default)]
    pub retain: Option<bool>,
    /// MQTT broker auth (username/password). Also accepted at root as `auth` with `type: username_password`.
    #[serde(default)]
    pub auth: Option<TargetAuth>,

    // HTTP
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,

    // TCP
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub line_delimiter: Option<bool>,
}

/// Simple username/password auth for MQTT (e.g. under target.auth in YAML).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceConfig {
    pub count: usize,
    #[serde(default = "default_id_prefix")]
    pub id_prefix: String,
}

fn default_id_prefix() -> String {
    "device".to_string()
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            count: 1,
            id_prefix: "device".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayloadConfig {
    #[serde(default = "default_payload_mode")]
    pub mode: PayloadMode,
    #[serde(default)]
    pub template_file: Option<String>,
    #[serde(default)]
    pub template_inline: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PayloadMode {
    Random,
    Template,
}

fn default_payload_mode() -> PayloadMode {
    PayloadMode::Random
}

impl Default for PayloadConfig {
    fn default() -> Self {
        Self {
            mode: PayloadMode::Random,
            template_file: None,
            template_inline: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunConfig {
    #[serde(default = "default_total_messages")]
    pub total_messages: usize,
    #[serde(default = "default_interval_ms")]
    pub interval_ms: u64,
    #[serde(default)]
    pub duration_secs: Option<u64>,
}

fn default_total_messages() -> usize {
    10
}

fn default_interval_ms() -> u64 {
    1000
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            total_messages: 10,
            interval_ms: 1000,
            duration_secs: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    UsernamePassword {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    ApiKey {
        header: String,
        value: String,
    },
}
