# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please disclose it responsibly by emailing:

**support@iotmer.com**

Include as much detail as possible:

- A description of the vulnerability and its potential impact
- Steps to reproduce or a proof-of-concept
- The affected version(s)
- Any suggested mitigations (if known)

You will receive a response within **72 hours** acknowledging your report. We will work with you to understand and resolve the issue and will keep you informed of the progress.

## Security Considerations for Users

### Secrets in Config Files

- **Never commit config files containing plaintext secrets** to version control.
- Use environment variable expansion for all sensitive values:
  ```yaml
  auth:
    type: username_password
    username: myuser
    password: "${MQTT_PASSWORD}"
  ```
- Add `mer.yaml` and any `*.yaml` files containing secrets to your `.gitignore`.

### TLS

- For production MQTT brokers, always use `mqtts://` (TLS) instead of `mqtt://`.
- For HTTP endpoints, always use `https://` in production.

### Environment Variables

- Store secrets in environment variables or a secrets manager (e.g., HashiCorp Vault, AWS Secrets Manager).
- Avoid logging environment variable values.

### Docker

- When using Docker, pass secrets via environment variables (`-e` flag or `--env-file`), not baked into the image.
- Never build Docker images with secrets in the `ARG` or `ENV` layers.

## Dependency Security

We use `cargo audit` in CI to detect known vulnerabilities in dependencies. To run it locally:

```bash
cargo install cargo-audit
cargo audit
```
