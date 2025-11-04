# ------------------------------
# Stage 0: Builder
# ------------------------------
    FROM rust:1.73 AS builder
    WORKDIR /usr/src/securerx
    
    # Copy root Cargo.toml and Cargo.lock for workspace
    COPY Cargo.toml Cargo.lock ./
    
    # Copy each crate's Cargo.toml individually to allow caching
    # This avoids re-downloading dependencies if only source changes
    # Path inside container: /usr/src/securerx/crates/<crate-name>/Cargo.toml
    COPY crates/securerx-core/Cargo.toml crates/securerx-core/
    COPY crates/securerx-node/Cargo.toml crates/securerx-node/
    # COPY crates/securerx-api/Cargo.toml crates/securerx-api/   # Uncomment if this crate exists
    
    # Optional: Pre-fetch dependencies for all workspace members
    RUN mkdir -p crates/securerx-core/src \
        && mkdir -p crates/securerx-node/src \
        && cargo fetch
    
    # Copy full source for all crates
    COPY crates/securerx-core/src ./crates/securerx-core/src
    COPY crates/securerx-node/src ./crates/securerx-node/src
    # COPY crates/securerx-api/src ./crates/securerx-api/src   # Uncomment if exists
    
    # Build the main node crate in release mode
    RUN cargo build --release -p securerx-node
    
    # ------------------------------
    # Stage 1: Minimal runtime image
    # ------------------------------
    FROM debian:bookworm-slim
    WORKDIR /app
    
    # Install curl or other runtime dependencies
    RUN apt-get update && apt-get install -y --no-install-recommends curl \
        && rm -rf /var/lib/apt/lists/*
    
    # Copy the compiled binary from builder
    COPY --from=builder /usr/src/securerx/target/release/securerx-node .
    
    # Expose ports if needed (example)
    EXPOSE 30333
    
    # Command to run
    CMD ["./securerx-node"]
    
