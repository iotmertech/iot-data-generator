use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub bearer_token: Option<String>,
    pub api_key_header: Option<String>,
    pub api_key_value: Option<String>,
}
