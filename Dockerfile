# ================================
# Stage 1: Rust build stage
# ================================
FROM rust:1.73 AS builder

# Set working directory inside the container
WORKDIR /usr/src/securerx

# Copy the Cargo.toml and Cargo.lock for the workspace
COPY Cargo.toml Cargo.lock ./

# Copy per-crate Cargo.toml files
COPY crates/securerx-core/Cargo.toml crates/securerx-core/
COPY crates/securerx-node/Cargo.toml crates/securerx-node/
# COPY crates/securerx-api/Cargo.toml crates/securerx-api/  # Uncomment if crate exists

# Optional: prefetch dependencies for all crates
# This requires the crates' src folders to exist if they are library crates
RUN mkdir -p crates/securerx-core/src \
    && mkdir -p crates/securerx-node/src \
    && cargo fetch

# Copy full source code for all crates
COPY crates/securerx-core/src ./crates/securerx-core/src
COPY crates/securerx-node/src ./crates/securerx-node/src
# COPY crates/securerx-api/src ./crates/securerx-api/src  # Uncomment if crate exists

# Build the main binary crate
RUN cargo build --release -p securerx-node

# ================================
# Stage 2: Minimal runtime image
# ================================
FROM debian:bookworm-slim AS runtime

# Set working directory in the runtime image
WORKDIR /app

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/securerx/target/release/securerx-node .

# Expose port if your node listens (adjust as needed)
EXPOSE 8080

# Default command to run the node
CMD ["./securerx-node"]
