# ================================
# Stage 1: Rust build stage
# ================================
FROM rust:1.73 AS builder

# Set working directory inside the container
WORKDIR /usr/src/securerx

# -------------------------------
# Copy manifests for dependency caching
# -------------------------------
COPY Cargo.toml Cargo.lock ./
COPY crates/securerx-core/Cargo.toml crates/securerx-core/
COPY crates/securerx-node/Cargo.toml crates/securerx-node/
COPY crates/securerx-api/Cargo.toml crates/securerx-api/
COPY crates/securerx-cli/Cargo.toml crates/securerx-cli/

# -------------------------------
# Create minimal src directories for cargo fetch
# cargo fetch needs at least an empty src/ to avoid "no targets" error
# -------------------------------
RUN mkdir -p crates/securerx-core/src \
    && mkdir -p crates/securerx-node/src \
    && mkdir -p crates/securerx-api/src \
    && mkdir -p crates/securerx-cli/src \
    && cargo fetch

# -------------------------------
# Copy full source code for all crates
# -------------------------------
COPY crates/securerx-core/src ./crates/securerx-core/src
COPY crates/securerx-node/src ./crates/securerx-node/src
COPY crates/securerx-api/src ./crates/securerx-api/src
COPY crates/securerx-cli/src ./crates/securerx-cli/src

# Build the API binary
RUN cargo build --release --package securerx-api

# ================================
# Stage 2: Minimal runtime image
# ================================
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built binary from builder stage
COPY --from=builder /usr/src/securerx/target/release/securerx-api .

# Expose API port
EXPOSE 8080

# Default command
CMD ["./securerx-api"]
