use crate::config::model::{AuthConfig, Config, PayloadMode, Protocol};
use crate::error::{MerError, Result};

pub fn validate_config(config: &Config) -> Result<()> {
    validate_target(config)?;
    validate_device(config)?;
    validate_payload(config)?;
    validate_run(config)?;
    validate_auth(config)?;
    Ok(())
}

fn validate_target(config: &Config) -> Result<()> {
    match config.protocol {
        Protocol::Mqtt => {
            require_field(config.target.broker.as_deref(), "target.broker")?;
            require_field(config.target.topic.as_deref(), "target.topic")?;
            if let Some(qos) = config.target.qos {
                if qos > 2 {
                    return Err(MerError::Validation(
                        "target.qos must be 0, 1, or 2".to_string(),
                    ));
                }
            }
        }
        Protocol::Http => {
            require_field(config.target.url.as_deref(), "target.url")?;
            let url = config.target.url.as_deref().unwrap();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(MerError::Validation(format!(
                    "target.url must start with http:// or https://, got: {url}"
                )));
            }
        }
        Protocol::Tcp => {
            require_field(config.target.host.as_deref(), "target.host")?;
            if config.target.port.is_none() {
                return Err(MerError::Validation(
                    "Required config field is missing: target.port".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_device(config: &Config) -> Result<()> {
    if config.device.count == 0 {
        return Err(MerError::Validation(
            "device.count must be at least 1".to_string(),
        ));
    }
    Ok(())
}

fn validate_payload(config: &Config) -> Result<()> {
    if config.payload.mode == PayloadMode::Template {
        if config.payload.template_file.is_none() && config.payload.template_inline.is_none() {
            return Err(MerError::Validation(
                "payload.mode is 'template' but neither payload.template_file nor payload.template_inline is set".to_string(),
            ));
        }
        if let Some(file) = &config.payload.template_file {
            if !std::path::Path::new(file).exists() {
                return Err(MerError::Validation(format!(
                    "payload.template_file '{}' does not exist",
                    file
                )));
            }
        }
    }
    Ok(())
}

fn validate_run(config: &Config) -> Result<()> {
    if config.run.total_messages == 0 {
        return Err(MerError::Validation(
            "run.total_messages must be at least 1".to_string(),
        ));
    }
    Ok(())
}

fn validate_auth(config: &Config) -> Result<()> {
    if let Some(auth) = &config.auth {
        match auth {
            AuthConfig::UsernamePassword { username, password } => {
                if username.is_empty() {
                    return Err(MerError::Validation(
                        "auth.username must not be empty".to_string(),
                    ));
                }
                if password.is_empty() {
                    return Err(MerError::Validation(
                        "auth.password must not be empty".to_string(),
                    ));
                }
            }
            AuthConfig::Bearer { token } => {
                if token.is_empty() {
                    return Err(MerError::Validation(
                        "auth.token must not be empty".to_string(),
                    ));
                }
            }
            AuthConfig::ApiKey { header, value } => {
                if header.is_empty() {
                    return Err(MerError::Validation(
                        "auth.header must not be empty".to_string(),
                    ));
                }
                if value.is_empty() {
                    return Err(MerError::Validation(
                        "auth.value must not be empty".to_string(),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn require_field(value: Option<&str>, field: &str) -> Result<()> {
    match value {
        Some(v) if !v.is_empty() => Ok(()),
        _ => Err(MerError::Validation(format!(
            "Required config field is missing: {}",
            field
        ))),
    }
}
