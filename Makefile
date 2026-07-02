.PHONY: build release test lint fmt fmt-check check ci audit setup-hooks clean docker docker-compose install help

BIN := mer
INSTALL_DIR := /usr/local/bin

## Build debug binary
build:
	cargo build

## Build optimized release binary
release:
	cargo build --release

## Run all tests
test:
	cargo test

## Run tests with stdout output
test-verbose:
	cargo test -- --nocapture

## Run clippy linter (fail on warnings)
lint:
	cargo clippy -- -D warnings

## Auto-format all source files
fmt:
	cargo fmt

## Check formatting without making changes
fmt-check:
	cargo fmt --check

## Run all CI checks: build + test + lint + fmt
check: build test lint fmt-check

## Run full CI locally (same checks as GitHub Actions test + security-audit jobs)
ci: check audit

## Run cargo security audit (requires: cargo install cargo-audit)
audit:
	@command -v cargo-audit >/dev/null 2>&1 || { \
		echo "error: cargo-audit not found. Install with: cargo install cargo-audit"; \
		exit 1; \
	}
	cargo audit

## Enable project git hooks (run once after clone)
setup-hooks:
	@git config core.hooksPath .githooks
	@chmod +x .githooks/pre-commit
	@echo "Git hooks enabled: pre-commit runs 'make ci' before each commit"

## Build Docker image
docker:
	docker build -t $(BIN):latest .

## Run mer against your broker via Docker Compose (set MQTT_BROKER in .env first)
docker-compose:
	docker compose up

## Install release binary to $(INSTALL_DIR)
install: release
	cp target/release/$(BIN) $(INSTALL_DIR)/$(BIN)
	@echo "Installed to $(INSTALL_DIR)/$(BIN)"

## Remove build artifacts
clean:
	cargo clean

## Show this help message
help:
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@grep -E '^## ' Makefile | sed 's/## /  /' | awk -F': ' '{printf "  %-20s %s\n", $$1, $$2}' || true
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:' Makefile | grep -v '^.PHONY' | awk -F':' '{printf "  %-20s\n", $$1}'
	@echo ""
