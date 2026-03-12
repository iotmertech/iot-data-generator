# Contributing to mer (iot-data-generator)

Thank you for your interest in contributing! This document explains how to get involved.

## Table of Contents

- [Contributing to mer (iot-data-generator)](#contributing-to-mer-iot-data-generator)
  - [Table of Contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [Getting Started](#getting-started)
  - [Development Setup](#development-setup)
    - [Prerequisites](#prerequisites)
    - [Build](#build)
    - [Build release binary](#build-release-binary)
  - [Running Tests](#running-tests)
  - [How to Contribute](#how-to-contribute)
    - [Reporting Bugs](#reporting-bugs)
    - [Requesting Features](#requesting-features)
    - [Submitting Code](#submitting-code)
  - [Submitting a Pull Request](#submitting-a-pull-request)
  - [Code Style](#code-style)
  - [Questions?](#questions)

---

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We expect all contributors to treat each other with respect.

---

## Getting Started

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/iotmertech/iot-data-generator.git
   cd iot-data-generator
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/iotmertech/iot-data-generator.git
   ```

---

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) stable toolchain (pinned via `rust-toolchain.toml`)

### Build

```bash
cargo build
```

### Build release binary

```bash
cargo build --release
# Binary: ./target/release/mer
```

---

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_expand_env_vars
```

---

## How to Contribute

### Reporting Bugs

Please [open an issue](https://github.com/iotmertech/iot-data-generator/issues/new?template=bug_report.md) with:

- A clear title and description
- Steps to reproduce
- Expected vs. actual behavior
- Your OS, Rust version (`rustc --version`), and `mer` version (`mer --version`)
- Relevant config file (redact any secrets)

### Requesting Features

Please [open an issue](https://github.com/iotmertech/iot-data-generator/issues/new?template=feature_request.md) with:

- A description of the problem you want to solve
- Your proposed solution (if you have one)
- Any alternatives you've considered

### Submitting Code

1. Create a branch from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```
2. Make your changes.
3. Add or update tests where appropriate.
4. Ensure the build passes and tests are green:
   ```bash
   cargo build
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```
5. Commit with a clear message:
   ```
   feat: add WebSocket protocol support
   fix: resolve MQTT reconnect loop on disconnect
   docs: add TLS configuration example
   ```
6. Push and open a Pull Request against `main`.

---

## Submitting a Pull Request

- Keep PRs focused — one feature or fix per PR.
- Reference the related issue in the PR description (e.g., `Closes #42`).
- Update `CHANGELOG.md` under the `[Unreleased]` section.
- Add or update examples if your change introduces new functionality.
- Ensure `cargo clippy` and `cargo fmt` pass.

---

## Code Style

- Use `cargo fmt` for formatting (enforced in CI).
- Use `cargo clippy` for linting (enforced in CI).
- Follow idiomatic Rust — prefer `?` over `.unwrap()`, use `thiserror` for errors.
- Keep functions small and focused.
- Add comments only where the intent is not obvious from the code.

---

## Questions?

Open a [GitHub Discussion](https://github.com/iotmertech/iot-data-generator/discussions) or an issue.
