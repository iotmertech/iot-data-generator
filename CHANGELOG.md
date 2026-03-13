# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- (New changes go here.)

---

## [0.0.1] - 2025-03-12

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
- Example configs in `examples/`
- Docker image (ghcr.io/iotmertech/mer) and docker-compose
- GitHub Actions: CI and release workflow (binaries + Docker)

[Unreleased]: https://github.com/iotmertech/iot-data-generator/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/iotmertech/iot-data-generator/releases/tag/v0.0.1
