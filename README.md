# iot-data-generator (`mer`)

[CI](https://github.com/iotmertech/iot-data-generator/actions/workflows/ci.yml)
[License: MIT](LICENSE)

A developer-friendly IoT test data generator CLI written in Rust.

Generate realistic IoT sensor payloads and send them to your system over **MQTT**, **HTTP**, or **TCP** — with zero infrastructure required to get started.

> **This is not a load-testing tool.** It is focused on high-quality, realistic IoT test data.

---

## Demo

From setup to sending data over MQTT:

![mer demo](docs/demo.gif)

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

### Option 1 — crates.io (recommended)

Install the latest release straight from [crates.io](https://crates.io/crates/mer-iot) with Cargo (installs the `mer` binary):

```bash
cargo install mer-iot
```

Requires [Rust](https://rustup.rs/) (stable). Upgrade later with `cargo install mer-iot --force`.

### Option 2 — Prebuilt binary

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

### Option 3 — Build from source

Requires [Rust](https://rustup.rs/) (stable):

```bash
git clone https://github.com/iotmertech/iot-data-generator.git
cd iot-data-generator
cargo build --release
# Binary: ./target/release/mer
sudo cp target/release/mer /usr/local/bin/   # optional: add to PATH
```

### Option 4 — Docker

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

### Option 5 — Docker Compose (bring your own broker)

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
mer init --protocol mqtt -f mer.yaml
```

On Windows, prefer `-f` over shell redirection (`> mer.yaml`). PowerShell writes UTF-16 by default, which older `mer` versions could not read.

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
| `mer init --protocol <mqtt|http|tcp> [-f <file>]` | Print or write a starter config (UTF-8)     |
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

## Simulated Timestamps

By default each payload's timestamp is the current wall-clock time. Add an
optional `time` section to generate a **simulated clock** instead — useful for
backfilling historical data or producing deterministic, evenly-spaced series.

```yaml
time:
  mode: fixed                    # real | fixed | random   (default: real)
  start: "2026-01-01T00:00:00Z"  # also accepts "2026-01-01 00:00:00" (UTC)
  step_secs: 300                 # fixed mode: +5 min per message
  min_secs: 60                   # random mode: minimum +1 min
  max_secs: 1800                 # random mode: maximum +30 min
  field: ts                      # JSON key for the timestamp (default "ts")
```

- **`real`** — current time (default, unchanged behavior).
- **`fixed`** — `start + seq * step_secs`. Deterministic and evenly spaced
  (e.g. start at `2026-01-01 00:00:00`, then `00:05:00`, `00:10:00`, …).
- **`random`** — starts at `start`, then advances by a random amount between
  `min_secs` and `max_secs` for each message.

The clock is global and advances once per message (in the same order as the
`seq` counter), shared across all devices.

In `template` mode the simulated timestamp is available as `{{ts}}` (RFC3339).
`{{now_utc}}` always returns real wall-clock time regardless of the `time`
setting.

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
| `{{ts}}`                   | —                            | simulated time (see `time` config) |
| `{{random_int min max}}`   | `{{random_int 0 100}}`       | `42`                   |
| `{{random_float min max}}` | `{{random_float 10.0 50.0}}` | `27.35`                |
| `{{random_bool}}`          | —                            | `true` or `false`      |
| `{{device_id}}`            | —                            | `sensor-0003`          |
| `{{device.index}}`         | —                            | `3`                    |
| `{{seq}}`                  | —                            | `17`                   |
| `{{seq_pulse min max peak_at total_steps}}`     | `{{seq_pulse 8.0 90.0 1344 2688}}`     | triangle: `min` at edges, `max` at `peak_at` |
| `{{seq_inv_pulse min max peak_at total_steps}}` | `{{seq_inv_pulse 0.80 0.99 1344 2688}}` | inverted triangle: `max` at edges, `min` at `peak_at` |
| `{{seq_after offset scale}}`                     | `{{seq_after 1344 0.5}}`               | delayed ramp: `max(0, seq - offset) * scale` |
| `{{seq_pulse_rand min max peak_min peak_max total_steps}}`     | `{{seq_pulse_rand 8.0 90.0 1000 1700 2688}}`     | triangle with a **random** peak in `[peak_min, peak_max]` (stable per device) |
| `{{seq_inv_pulse_rand min max peak_min peak_max total_steps}}` | `{{seq_inv_pulse_rand 0.80 0.99 1000 1700 2688}}` | inverted triangle sharing the device's random peak |


The `seq_*` helpers read the current message `seq` (and device index)
automatically from the context — you don't pass them as arguments. They're handy
for shaping values over a run (e.g. a mid-cycle triangle load, a cos φ dip, or a
compensation bank that starts accumulating after a threshold).

Use the `_rand` variants when each device should peak at a **different, random
time**: the peak is drawn once from `[peak_min, peak_max]` and stays fixed for
that device's whole run. All `_rand` helpers for a given device share the same
peak, so active/reactive load and the cos φ dip line up in time. See
[`energy-triangle.yaml`](examples/energy-triangle.yaml).

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
| `[demo-mqtt.yaml](examples/demo-mqtt.yaml)`               | Demo: mqtt.iotmer.cloud, public broker, README GIF       |
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
| `[time-fixed.yaml](examples/time-fixed.yaml)`             | Simulated timestamps, fixed +5 min step                 |
| `[time-random.yaml](examples/time-random.yaml)`           | Simulated timestamps, random 1–30 min step              |
| `[time-template.yaml](examples/time-template.yaml)`       | Simulated `{{ts}}` inside a custom template              |
| `[energy-triangle.yaml](examples/energy-triangle.yaml)`   | Mid-month triangle load, cos φ dip & compensation ramp  |


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
