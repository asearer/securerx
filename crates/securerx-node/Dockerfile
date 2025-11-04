# ================================
# Stage 1: Rust build stage
# ================================
FROM rust:1.73 AS builder

# Set working directory inside container
WORKDIR /usr/src/securerx

# -------------------------------
# Step 1: Copy workspace manifests only
# This allows Docker to cache dependencies if Cargo.toml hasn't changed
# -------------------------------
COPY Cargo.toml Cargo.lock ./

# -------------------------------
# Step 2: Copy crate manifests for caching
# This copies Cargo.toml for all crates into their respective paths
# -------------------------------
RUN mkdir -p crates/securerx-core/src \
    && mkdir -p crates/securerx-node/src \
    && mkdir -p crates/securerx-api/src \
    && mkdir -p crates/securerx-cli/src

COPY crates/securerx-core/Cargo.toml crates/securerx-core/
COPY crates/securerx-node/Cargo.toml crates/securerx-node/
COPY crates/securerx-api/Cargo.toml crates/securerx-api/
COPY crates/securerx-cli/Cargo.toml crates/securerx-cli/

# -------------------------------
# Step 3: Pre-fetch dependencies for all crates
# Creating empty src directories prevents "no targets specified" errors
# -------------------------------
RUN cargo fetch

# -------------------------------
# Step 4: Copy full source code
# -------------------------------
COPY crates/ ./crates/

# -------------------------------
# Step 5: Build the main node binary in release mode
# -------------------------------
RUN cargo build --release --package securerx-node

# ================================
# Stage 2: Minimal runtime image
# ================================
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built binary from builder stage
COPY --from=builder /usr/src/securerx/target/release/securerx-node .

# Expose ports used by the node
EXPOSE 8081 9090

# Default command
CMD ["./securerx-node"]
