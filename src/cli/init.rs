use crate::error::Result;

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

pub fn init(protocol: &str) -> Result<()> {
    let template = match protocol {
        "mqtt" => MQTT_TEMPLATE,
        "http" => HTTP_TEMPLATE,
        "tcp" => TCP_TEMPLATE,
        _ => MQTT_TEMPLATE,
    };
    print!("{}", template);
    Ok(())
}
