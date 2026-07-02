use crate::error::Result;
use std::path::Path;

const MQTT_TEMPLATE: &str = r#"protocol: mqtt

target:
  broker: mqtt://localhost:1883
  topic: "devices/{device_id}/telemetry"
  client_id: "mer-client-{device_id}"
  qos: 1
  retain: false

device:
  count: 5
  id_prefix: device

payload:
  mode: random

run:
  total_messages: 100
  interval_ms: 1000
"#;

const HTTP_TEMPLATE: &str = r#"protocol: http

target:
  url: "http://localhost:8080/api/data"
  method: POST
  timeout_secs: 10
  headers:
    Content-Type: application/json

device:
  count: 5
  id_prefix: device

payload:
  mode: random

run:
  total_messages: 100
  interval_ms: 1000
"#;

const TCP_TEMPLATE: &str = r#"protocol: tcp

target:
  host: localhost
  port: 9000
  timeout_secs: 5
  line_delimiter: true

device:
  count: 5
  id_prefix: device

payload:
  mode: random

run:
  total_messages: 100
  interval_ms: 1000
"#;

pub fn init(protocol: &str, output: Option<&Path>) -> Result<()> {
    let template = match protocol {
        "mqtt" => MQTT_TEMPLATE,
        "http" => HTTP_TEMPLATE,
        "tcp" => TCP_TEMPLATE,
        _ => MQTT_TEMPLATE,
    };
    if let Some(path) = output {
        std::fs::write(path, template)?;
    } else {
        print!("{}", template);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_writes_utf8_file() {
        let path = std::env::temp_dir().join(format!("mer-init-{}.yaml", uuid::Uuid::new_v4()));
        init("mqtt", Some(&path)).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert!(!bytes.starts_with(&[0xFF, 0xFE]), "should not be UTF-16 LE");
        assert!(String::from_utf8(bytes).is_ok());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("protocol: mqtt"));
        let _ = std::fs::remove_file(path);
    }
}
