# iot-data-generator (`mer`)

A developer-friendly IoT test data generator CLI written in Rust.

Generate meaningful IoT test data and send it to target systems over **MQTT**, **HTTP**, or **TCP**.

> **This is not a load-testing tool.** It is a simple, high-quality IoT test data generator.

---

## Features

- Generate random IoT payloads (temperature, humidity, voltage, power, etc.)
- Custom JSON template payloads using Handlebars helpers
- Send data over MQTT, HTTP, or TCP
- Configure interval, total message count, or duration limit
- Validate config before running
- Preview generated payloads before sending
- Simple run summary at the end
- Environment variable expansion in config (`${VAR_NAME}`)

---

## Installation

### Build from source

```bash
git clone https://github.com/iotmertech/iot-data-generator.git
cd iot-data-generator
cargo build --release
# Binary is at: ./target/release/mer
```

---

## Quickstart

### 1. Generate a config

```bash
mer init --protocol mqtt > mer.yaml
```

This outputs a starter YAML config you can customize.

### 2. Preview payloads

```bash
mer preview payload -f mer.yaml
```

See sample payloads before sending anything.

### 3. Run

```bash
# MQTT
mer mqtt run -f mer.yaml

# HTTP
mer http run -f mer.yaml

# TCP
mer tcp run -f mer.yaml
```

---

## Commands

| Command | Description |
|---|---|
| `mer mqtt run -f <config>` | Send MQTT messages |
| `mer http run -f <config>` | Send HTTP requests |
| `mer tcp run -f <config>` | Send TCP messages |
| `mer validate config -f <config>` | Validate a config file |
| `mer preview payload -f <config>` | Preview generated payloads |
| `mer init --protocol <mqtt\|http\|tcp>` | Generate a starter config |

---

## Config Reference

### MQTT example

```yaml
protocol: mqtt

target:
  broker: mqtt://localhost:1883
  topic: "devices/{device_id}/telemetry"
  client_id: "mer-{device_id}"
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
```

### HTTP example

```yaml
protocol: http

target:
  url: "http://localhost:8080/api/v1/devices/{device_id}/data"
  method: POST
  timeout_secs: 10
  headers:
    Authorization: "Bearer ${API_TOKEN}"

device:
  count: 3
  id_prefix: sensor

payload:
  mode: random

run:
  total_messages: 50
  interval_ms: 500
```

### TCP example

```yaml
protocol: tcp

target:
  host: localhost
  port: 9000
  timeout_secs: 5
  line_delimiter: true

device:
  count: 2
  id_prefix: node

payload:
  mode: random

run:
  total_messages: 20
  interval_ms: 1000
```

---

## Payload Modes

### `random` (default)

Generates meaningful IoT fields automatically:

```json
{
  "device_id": "device-0001",
  "device_index": 1,
  "seq": 42,
  "ts": "2024-01-01T00:00:00Z",
  "temperature": 23.45,
  "humidity": 61.2,
  "voltage": 230.5,
  "current": 3.2,
  "power": 738.0,
  "energy_total": 1523.7,
  "status": "online"
}
```

### `template`

Use a Handlebars template for full control:

```yaml
payload:
  mode: template
  template_inline: |
    {
      "device_id": "{{device_id}}",
      "ts": "{{now_utc}}",
      "temperature": {{random_float 18.0 35.0}},
      "humidity": {{random_float 30.0 90.0}},
      "active": {{random_bool}},
      "seq": {{seq}},
      "level": {{random_int 0 5}}
    }
```

Or use an external file:

```yaml
payload:
  mode: template
  template_file: ./my-template.json
```

#### Available template helpers

| Helper | Example | Description |
|---|---|---|
| `{{now_utc}}` | `2024-01-01T00:00:00Z` | Current UTC timestamp |
| `{{random_int min max}}` | `{{random_int 0 100}}` | Random integer in range |
| `{{random_float min max}}` | `{{random_float 10.0 50.0}}` | Random float in range |
| `{{random_bool}}` | `true` or `false` | Random boolean |
| `{{device_id}}` | `device-0003` | Current device ID |
| `{{device.index}}` | `3` | Current device index |
| `{{seq}}` | `42` | Message sequence number |

---

## Environment Variables

Use `${VAR_NAME}` in your config to inject secrets from environment variables:

```yaml
auth:
  type: username_password
  username: admin
  password: "${MQTT_PASS}"
```

If the variable is not set, `mer` will report a clear error:

```
Error: Environment variable 'MQTT_PASS' is not set
```

---

## Auth

### Username/Password (MQTT, HTTP)

```yaml
auth:
  type: username_password
  username: myuser
  password: "${MY_PASSWORD}"
```

### Bearer Token (HTTP)

```yaml
auth:
  type: bearer
  token: "${API_TOKEN}"
```

### API Key Header (HTTP)

```yaml
auth:
  type: api_key
  header: X-API-Key
  value: "${API_KEY}"
```

---

## Run Summary

After each run, `mer` prints a summary:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Run Summary — MQTT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total sent    : 100
  Successful    : 98
  Failed        : 2
  Success rate  : 98.0%
  Errors        : 2
  Bytes sent    : 24.5 KB
  Elapsed       : 100.12s
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Examples

See the [`examples/`](examples/) directory:

- [`mqtt-basic.yaml`](examples/mqtt-basic.yaml)
- [`http-basic.yaml`](examples/http-basic.yaml)
- [`tcp-basic.yaml`](examples/tcp-basic.yaml)
- [`custom-template.yaml`](examples/custom-template.yaml)

---

## License

MIT
