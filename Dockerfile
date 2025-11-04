# Multi-stage build for SecureRx blockchain node
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/securerx

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create all workspace member directories (ALL 4 CRATES)
RUN mkdir -p crates/securerx-core/src \
    && mkdir -p crates/securerx-node/src \
    && mkdir -p crates/securerx-api/src \
    && mkdir -p crates/securerx-cli/src

# Copy workspace member Cargo.toml files
COPY crates/securerx-core/Cargo.toml crates/securerx-core/
COPY crates/securerx-node/Cargo.toml crates/securerx-node/
COPY crates/securerx-api/Cargo.toml crates/securerx-api/
COPY crates/securerx-cli/Cargo.toml crates/securerx-cli/

# Now fetch dependencies (after all Cargo.toml files are in place)
RUN cargo fetch

# Copy source code
COPY crates ./crates

# Build the node binary
RUN cargo build --release --bin securerx-node

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/securerx/target/release/securerx-node /usr/local/bin/

EXPOSE 8081 9090

CMD ["securerx-node"]
