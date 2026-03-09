# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Unit tests for config loading, validation, device pool, payload generation, and metrics
- `Makefile` with common development commands
- `Dockerfile` and `docker-compose.yml` for containerized usage
- GitHub Actions CI/CD: build, test, lint on Ubuntu, macOS, and Windows
- GitHub Actions release workflow: cross-platform prebuilt binaries on tag push
- New example configs: `mqtt-tls.yaml`, `mqtt-auth.yaml`, `http-bearer.yaml`, `http-api-key.yaml`, `smart-home.yaml`, `industrial.yaml`, `fleet-tracking.yaml`
- `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`
- `LICENSE` file (MIT)
- Improved `README.md` with badges, Docker quickstart, and install instructions for all platforms

### Changed
- `Cargo.toml`: added `authors`, `homepage`, `documentation`, `keywords`, `categories`, `rust-version`
- Removed unused `auth::model::Credentials` struct (superseded by `config::model::AuthConfig`)
- Removed `#[allow(dead_code)]` from `error.rs`

### Fixed
- N/A

---

## [0.1.0] - 2024-01-01

### Added
- Initial release
- MQTT, HTTP, and TCP protocol support
- Random IoT payload generation (temperature, humidity, voltage, current, power, energy, status)
- Handlebars template payload mode with helpers: `now_utc`, `random_int`, `random_float`, `random_bool`, `device_id`, `device.index`, `seq`
- `mer init` command to generate a starter config
- `mer validate config` command to validate a config file
- `mer preview payload` command to preview generated payloads
- Environment variable expansion (`${VAR_NAME}`) in config files
- Auth support: username/password, Bearer token, API key header
- MQTT TLS support (`mqtts://`)
- Run summary output after each run
- Four example configs in `examples/`

[Unreleased]: https://github.com/iotmertech/iot-data-generator/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/iotmertech/iot-data-generator/releases/tag/v0.1.0
