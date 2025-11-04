# ===== Build Stage =====
FROM rust:1.73 AS builder
WORKDIR /usr/src/securerx

# Copy manifests for dependency caching (workspace + all members)
COPY Cargo.toml Cargo.lock ./
COPY crates/securerx-core/Cargo.toml crates/securerx-core/
COPY crates/securerx-node/Cargo.toml crates/securerx-node/
COPY crates/securerx-api/Cargo.toml crates/securerx-api/
COPY crates/securerx-cli/Cargo.toml crates/securerx-cli/

# Create minimal src trees to satisfy target discovery during cargo fetch
RUN mkdir -p crates/securerx-core/src \
    && mkdir -p crates/securerx-node/src \
    && mkdir -p crates/securerx-api/src \
    && mkdir -p crates/securerx-cli/src \
    && cargo fetch

# Copy full source
COPY . .

# Build the API binary
RUN cargo build --release --package securerx-api

# ===== Runtime Stage =====
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy built binary from builder
COPY --from=builder /usr/src/securerx/target/release/securerx-api .

# Expose API port
EXPOSE 8080

# Default command
CMD ["./securerx-api"]

