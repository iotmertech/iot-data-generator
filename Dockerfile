# ---- Build stage ----
FROM rust:1.77-slim AS builder

WORKDIR /app

# Cache dependencies separately from source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release --locked && rm -rf src

# Build the real binary
COPY src ./src
# Touch main.rs so Cargo rebuilds it (not just the deps)
RUN touch src/main.rs
RUN cargo build --release --locked

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/mer /usr/local/bin/mer

# Copy examples so users can get started quickly
COPY examples ./examples

ENTRYPOINT ["mer"]
CMD ["--help"]
