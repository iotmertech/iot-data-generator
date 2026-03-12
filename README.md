# iot-data-generator (`mer`)

[CI](https://github.com/iotmertech/iot-data-generator/actions/workflows/ci.yml)
[License: MIT](LICENSE)

A developer-friendly IoT test data generator CLI written in Rust.

Generate realistic IoT sensor payloads and send them to your system over **MQTT**, **HTTP**, or **TCP** — with zero infrastructure required to get started.

> **This is not a load-testing tool.** It is focused on high-quality, realistic IoT test data.

---

## Features

- **Random payload** generation — temperature, humidity, voltage, current, power, energy, status
- **Custom Handlebars templates** — full control over payload structure
- **Three protocols** — MQTT (plain & TLS), HTTP, TCP
- **Authentication** — username/password, Bearer token, API key header
- **Environment variable expansion** — keep secrets out of config files with `${VAR_NAME}`
- **Validation & preview** — check config and see sample payloads before sending
- **Run control** — total message count, interval, or hard time limit
- **Cross-platform** — Linux, macOS, Windows (prebuilt binaries available)
- **Docker ready** — single image, compose with a local Mosquitto broker included

---

## Installation

### Option 1 — Prebuilt binary (recommended)

Download the latest binary for your platform from [GitHub Releases](https://github.com/iotmertech/iot-data-generator/releases). Binaries are built automatically when a [release is published](https://github.com/iotmertech/iot-data-generator/releases):

```bash
# Linux x86_64
curl -L https://github.com/iotmertech/iot-data-generator/releases/latest/download/mer-latest-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mer /usr/local/bin/

# Linux ARM64
curl -L https://github.com/iotmertech/iot-data-generator/releases/latest/download/mer-latest-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mer /usr/local/bin/

# macOS Apple Silicon
curl -L https://github.com/iotmertech/iot-data-generator/releases/latest/download/mer-latest-aarch64-apple-darwin.tar.gz | tar xz
sudo mv mer /usr/local/bin/

# macOS Intel
curl -L https://github.com/iotmertech/iot-data-generator/releases/latest/download/mer-latest-x86_64-apple-darwin.tar.gz | tar xz
sudo mv mer /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/iotmertech/iot-data-generator/releases/latest/download/mer-latest-x86_64-pc-windows-msvc.zip" -OutFile mer.zip
Expand-Archive mer.zip -DestinationPath .
# Add mer.exe to your PATH
```

### Option 2 — Build from source

Requires [Rust](https://rustup.rs/) (stable):

```bash
git clone https://github.com/iotmertech/iot-data-generator.git
cd iot-data-generator
cargo build --release
# Binary: ./target/release/mer
sudo cp target/release/mer /usr/local/bin/   # optional: add to PATH
```

### Option 3 — Docker

```bash
# Pull and run (no local Rust needed)
docker run --rm ghcr.io/iotmertech/mer:latest --help

# Run an example against a local broker
docker run --rm \
  -v $(pwd)/examples:/app/examples:ro \
  --network host \
  ghcr.io/iotmertech/mer:latest \
  mqtt run -f /app/examples/mqtt-basic.yaml
```

### Option 4 — Docker Compose (bring your own broker)

Runs `mer` in a container and connects to your existing broker:

```bash
git clone https://github.com/iotmertech/iot-data-generator.git
cd iot-data-generator
cp .env.example .env           # set MQTT_BROKER and MQTT_TOPIC
docker compose up
```

For brokers that require auth, edit the `command` in `docker-compose.yml` to use `docker/mer.docker-auth.yaml` and fill in `MQTT_USERNAME` / `MQTT_PASSWORD` in your `.env`.

> No broker is bundled — `mer` connects to whatever broker you point it at.

---

## Quickstart

### 1. Generate a starter config

```bash
mer init --protocol mqtt > mer.yaml
```

### 2. Preview payloads

```bash
mer preview payload -f mer.yaml
mer preview payload -f mer.yaml -n 5   # show 5 samples
```

### 3. Validate the config

```bash
mer validate config -f mer.yaml
```

### 4. Send data

```bash
mer mqtt run -f mer.yaml
mer http run -f mer.yaml
mer tcp run -f mer.yaml
```

---

## Commands


| Command                                | Description                                 |
| -------------------------------------- | ------------------------------------------- |
| `mer init --protocol <mqtt|http|tcp>`  | Print a starter config to stdout            |
| `mer validate config -f <file>`        | Validate a config file (exits 0 on success) |
| `mer preview payload -f <file> [-n N]` | Preview N generated payloads (default 3)    |
| `mer mqtt run -f <file>`               | Send MQTT messages                          |
| `mer http run -f <file>`               | Send HTTP requests                          |
| `mer tcp run -f <file>`                | Send TCP messages                           |


---

## Config Reference

### MQTT

```yaml
protocol: mqtt

target:
  broker: "mqtt://mqtt.iotmer.cloud:1883"     # or mqtts:// for TLS (port 8883)
  topic: "devices/{device_id}/telemetry"
  client_id: "mer-{device_id}"        # optional, auto-generated if omitted
  qos: 1                              # 0, 1, or 2
  retain: false

device:
  count: 5
  id_prefix: device                   # device IDs: device-0000, device-0001, …

payload:
  mode: random                        # or template

run:
  total_messages: 100
  interval_ms: 1000                   # ms between messages
  duration_secs: 60                   # optional hard time limit
```

### HTTP

```yaml
protocol: http

target:
  url: "http://ingest.iotmer.cloud:8080/api/v1/devices/{device_id}/data"
  method: POST                        # GET, POST, PUT, PATCH
  timeout_secs: 10
  headers:
    Content-Type: "application/json"
    X-Custom-Header: "value"

device:
  count: 3
  id_prefix: sensor

payload:
  mode: random

run:
  total_messages: 50
  interval_ms: 500
```

### TCP

```yaml
protocol: tcp

target:
  host: localhost
  port: 9000
  timeout_secs: 5
  line_delimiter: true                # append \n to each message

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

Generates a realistic IoT JSON payload automatically:

```json
{
  "device_id": "device-0001",
  "device_index": 1,
  "seq": 42,
  "ts": "2024-01-01T12:00:00Z",
  "temperature": 23.45,
  "humidity": 61.2,
  "voltage": 230.5,
  "current": 3.2,
  "power": 738.0,
  "energy_total": 1523.7,
  "status": "online"
}
```

Field ranges:


| Field          | Range                                          |
| -------------- | ---------------------------------------------- |
| `temperature`  | 15.0 – 45.0 °C                                 |
| `humidity`     | 20.0 – 95.0 %                                  |
| `voltage`      | 220.0 – 240.0 V                                |
| `current`      | 0.1 – 15.0 A                                   |
| `power`        | 10.0 – 3600.0 W                                |
| `energy_total` | 0.0 – 10000.0 kWh                              |
| `status`       | `online` (4/6), `offline` (1/6), `error` (1/6) |


### `template`

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

Or load from an external file:

```yaml
payload:
  mode: template
  template_file: ./my-template.json
```

#### Template helpers


| Helper                     | Example                      | Output                 |
| -------------------------- | ---------------------------- | ---------------------- |
| `{{now_utc}}`              | —                            | `2024-01-01T12:00:00Z` |
| `{{random_int min max}}`   | `{{random_int 0 100}}`       | `42`                   |
| `{{random_float min max}}` | `{{random_float 10.0 50.0}}` | `27.35`                |
| `{{random_bool}}`          | —                            | `true` or `false`      |
| `{{device_id}}`            | —                            | `sensor-0003`          |
| `{{device.index}}`         | —                            | `3`                    |
| `{{seq}}`                  | —                            | `17`                   |


---

## Authentication

### Username / Password (MQTT, HTTP)

```yaml
auth:
  type: username_password
  username: myuser
  password: "${MQTT_PASSWORD}"
```

For MQTT, you can also set credentials directly under `target`:

```yaml
target:
  broker: "mqtts://mqtt.iotmer.cloud:8883"
  topic: "test/{device_id}"
  auth:
    username: myuser
    password: "${MQTT_PASSWORD}"
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

## Environment Variables

Use `${VAR_NAME}` anywhere in your config to inject secrets at runtime:

```yaml
auth:
  type: bearer
  token: "${MY_API_TOKEN}"
```

```bash
export MY_API_TOKEN="secret-token"
mer http run -f mer.yaml
```

If a variable is not set, `mer` exits with a clear error:

```
Error: Environment variable 'MY_API_TOKEN' is not set
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

The `[examples/](examples/)` directory contains ready-to-use configs:


| File                                                      | Description                                             |
| --------------------------------------------------------- | ------------------------------------------------------- |
| `[mqtt-basic.yaml](examples/mqtt-basic.yaml)`             | 5 devices, 20 messages, random payload, MQTT localhost  |
| `[mqtt-auth.yaml](examples/mqtt-auth.yaml)`               | MQTT with username/password auth                        |
| `[mqtt-tls.yaml](examples/mqtt-tls.yaml)`                 | MQTT over TLS (`mqtts://`) with auth                    |
| `[http-basic.yaml](examples/http-basic.yaml)`             | 3 sensors, HTTP POST with custom header                 |
| `[http-bearer.yaml](examples/http-bearer.yaml)`           | HTTP POST with Bearer token auth                        |
| `[http-api-key.yaml](examples/http-api-key.yaml)`         | HTTP POST with API key header auth                      |
| `[tcp-basic.yaml](examples/tcp-basic.yaml)`               | 2 nodes, TCP with line delimiter                        |
| `[custom-template.yaml](examples/custom-template.yaml)`   | MQTT with inline Handlebars template                    |
| `[smart-home.yaml](examples/smart-home.yaml)`             | Smart home sensors (temp, plug, door, CO₂, lux)         |
| `[industrial.yaml](examples/industrial.yaml)`             | Factory floor machines (vibration, pressure, flow, RPM) |
| `[fleet-tracking.yaml](examples/fleet-tracking.yaml)`     | Vehicle GPS tracking over HTTP                          |
| `[energy-meter.yaml](examples/energy-meter.yaml)`         | Three-phase smart energy meters over MQTT               |
| `[weather-station.yaml](examples/weather-station.yaml)`   | Weather stations over TCP (wind, rain, UV, pressure)    |
| `[duration-limited.yaml](examples/duration-limited.yaml)` | Time-capped run with `duration_secs`                    |


---

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# All checks (same as CI)
make check
```

See `[CONTRIBUTING.md](CONTRIBUTING.md)` for the full contribution guide.

---

## Security

Please read our [Security Policy](SECURITY.md) before reporting vulnerabilities.

**Key points:**

- Never commit plaintext secrets — use `${ENV_VAR}` in config files
- Use `mqtts://` for production MQTT connections
- Use `https://` for production HTTP endpoints

---

## License

[MIT](LICENSE) © 2026 iotmertech
