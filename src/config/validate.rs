use crate::config::model::{AuthConfig, Config, PayloadMode, Protocol, TimeMode};
use crate::error::{MerError, Result};

pub fn validate_config(config: &Config) -> Result<()> {
    validate_target(config)?;
    validate_device(config)?;
    validate_payload(config)?;
    validate_run(config)?;
    validate_auth(config)?;
    validate_time(config)?;
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
    // Root-level auth
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
    // MQTT target.auth (under target:)
    if config.protocol == Protocol::Mqtt {
        if let Some(auth) = &config.target.auth {
            if auth.username.is_empty() {
                return Err(MerError::Validation(
                    "target.auth.username must not be empty".to_string(),
                ));
            }
            if auth.password.is_empty() {
                return Err(MerError::Validation(
                    "target.auth.password must not be empty".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_time(config: &Config) -> Result<()> {
    let time = match &config.time {
        Some(t) => t,
        None => return Ok(()),
    };

    match time.mode {
        TimeMode::Real => {}
        TimeMode::Fixed => {
            if time.start.is_none() {
                return Err(MerError::Validation(
                    "time.start is required when time.mode is 'fixed'".to_string(),
                ));
            }
            match time.step_secs {
                Some(step) if step > 0 => {}
                _ => {
                    return Err(MerError::Validation(
                        "time.step_secs must be a positive integer when time.mode is 'fixed'"
                            .to_string(),
                    ))
                }
            }
        }
        TimeMode::Random => {
            if time.start.is_none() {
                return Err(MerError::Validation(
                    "time.start is required when time.mode is 'random'".to_string(),
                ));
            }
            let min = time.min_secs.unwrap_or(0);
            let max = time.max_secs.unwrap_or(0);
            if min <= 0 || max <= 0 {
                return Err(MerError::Validation(
                    "time.min_secs and time.max_secs must be positive when time.mode is 'random'"
                        .to_string(),
                ));
            }
            if min > max {
                return Err(MerError::Validation(
                    "time.min_secs must be <= time.max_secs".to_string(),
                ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::{
        AuthConfig, Config, DeviceConfig, PayloadConfig, PayloadMode, Protocol, RunConfig, Target,
        TimeConfig, TimeMode,
    };

    fn base_target_mqtt() -> Target {
        Target {
            broker: Some("mqtt://localhost:1883".to_string()),
            topic: Some("test/{device_id}".to_string()),
            client_id: None,
            qos: Some(1),
            retain: Some(false),
            auth: None,
            url: None,
            method: None,
            headers: None,
            timeout_secs: None,
            host: None,
            port: None,
            line_delimiter: None,
        }
    }

    fn base_target_http() -> Target {
        Target {
            broker: None,
            topic: None,
            client_id: None,
            qos: None,
            retain: None,
            auth: None,
            url: Some("http://localhost:8080/data".to_string()),
            method: Some("POST".to_string()),
            headers: None,
            timeout_secs: Some(5),
            host: None,
            port: None,
            line_delimiter: None,
        }
    }

    fn base_target_tcp() -> Target {
        Target {
            broker: None,
            topic: None,
            client_id: None,
            qos: None,
            retain: None,
            auth: None,
            url: None,
            method: None,
            headers: None,
            timeout_secs: Some(5),
            host: Some("localhost".to_string()),
            port: Some(9000),
            line_delimiter: Some(true),
        }
    }

    fn base_config(protocol: Protocol, target: Target) -> Config {
        Config {
            protocol,
            target,
            device: DeviceConfig {
                count: 2,
                id_prefix: "dev".to_string(),
            },
            payload: PayloadConfig::default(),
            run: RunConfig::default(),
            auth: None,
            time: None,
        }
    }

    #[test]
    fn test_valid_mqtt_config() {
        let config = base_config(Protocol::Mqtt, base_target_mqtt());
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_valid_http_config() {
        let config = base_config(Protocol::Http, base_target_http());
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_valid_tcp_config() {
        let config = base_config(Protocol::Tcp, base_target_tcp());
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_mqtt_missing_broker() {
        let mut target = base_target_mqtt();
        target.broker = None;
        let config = base_config(Protocol::Mqtt, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_mqtt_missing_topic() {
        let mut target = base_target_mqtt();
        target.topic = None;
        let config = base_config(Protocol::Mqtt, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_mqtt_invalid_qos() {
        let mut target = base_target_mqtt();
        target.qos = Some(3);
        let config = base_config(Protocol::Mqtt, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_http_missing_url() {
        let mut target = base_target_http();
        target.url = None;
        let config = base_config(Protocol::Http, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_http_invalid_url_scheme() {
        let mut target = base_target_http();
        target.url = Some("ftp://bad.example.com".to_string());
        let config = base_config(Protocol::Http, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_tcp_missing_port() {
        let mut target = base_target_tcp();
        target.port = None;
        let config = base_config(Protocol::Tcp, target);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_device_count_zero_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.device.count = 0;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_run_total_messages_zero_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.run.total_messages = 0;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_template_mode_without_template_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.payload.mode = PayloadMode::Template;
        config.payload.template_file = None;
        config.payload.template_inline = None;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_template_mode_with_inline_ok() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.payload.mode = PayloadMode::Template;
        config.payload.template_inline = Some(r#"{"v":1}"#.to_string());
        assert!(validate_config(&config).is_ok());
    }

    fn time_cfg(mode: TimeMode) -> TimeConfig {
        TimeConfig {
            mode,
            start: Some("2026-01-01T00:00:00Z".to_string()),
            step_secs: Some(300),
            min_secs: Some(60),
            max_secs: Some(1800),
            field: "ts".to_string(),
        }
    }

    #[test]
    fn test_time_fixed_valid() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.time = Some(time_cfg(TimeMode::Fixed));
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_time_fixed_missing_start_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        let mut t = time_cfg(TimeMode::Fixed);
        t.start = None;
        config.time = Some(t);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_time_fixed_nonpositive_step_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        let mut t = time_cfg(TimeMode::Fixed);
        t.step_secs = Some(0);
        config.time = Some(t);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_time_random_min_greater_than_max_fails() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        let mut t = time_cfg(TimeMode::Random);
        t.min_secs = Some(1800);
        t.max_secs = Some(60);
        config.time = Some(t);
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_time_random_valid() {
        let mut config = base_config(Protocol::Mqtt, base_target_mqtt());
        config.time = Some(time_cfg(TimeMode::Random));
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_auth_bearer_empty_token_fails() {
        let mut config = base_config(Protocol::Http, base_target_http());
        config.auth = Some(AuthConfig::Bearer {
            token: String::new(),
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_auth_api_key_empty_header_fails() {
        let mut config = base_config(Protocol::Http, base_target_http());
        config.auth = Some(AuthConfig::ApiKey {
            header: String::new(),
            value: "some-key".to_string(),
        });
        assert!(validate_config(&config).is_err());
    }
}
