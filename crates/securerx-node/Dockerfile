# ===== Build Stage =====
FROM rust:1.73 AS builder
WORKDIR /usr/src/securerx

# 1️⃣ Copy only workspace manifests first for caching
COPY Cargo.toml Cargo.lock ./

# 2️⃣ Copy all crate manifests (Cargo.toml) to leverage layer caching
COPY crates/*/Cargo.toml crates/

# 3️⃣ Copy full source code for all crates
COPY crates/ ./crates/

# 4️⃣ Fetch dependencies for all workspace members
RUN cargo fetch

# 5️⃣ Build the main node binary in release mode
RUN cargo build --release --package securerx-node

# ===== Runtime Stage =====
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built binary from builder stage
COPY --from=builder /usr/src/securerx/target/release/securerx-node .

# Expose ports used by the node
EXPOSE 8081 9090

# Default command
CMD ["./securerx-node"]
