# =========================
# Stage 1: Builder
# =========================
FROM rust:1.73 as builder

# Set working directory to root of workspace
WORKDIR /usr/src/securerx

# Copy root workspace manifest for caching
COPY Cargo.toml Cargo.lock ./

# Dynamically copy all crate Cargo.toml files
RUN for crate in crates/*; do \
        if [ -f "$crate/Cargo.toml" ]; then \
            mkdir -p "$crate"; \
            cp "$crate/Cargo.toml" "$crate/"; \
        fi; \
    done

# Dynamically copy all crate source directories
RUN for crate in crates/*; do \
        if [ -d "$crate/src" ]; then \
            mkdir -p "$crate/src"; \
            cp -r "$crate/src" "$crate/src"; \
        fi; \
    done

# Fetch dependencies for all workspace members
RUN cargo fetch

# Build the main node crate in release mode
WORKDIR /usr/src/securerx/crates/securerx-node
RUN cargo build --release

# =========================
# Stage 2: Runtime
# =========================
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Set working directory in runtime image
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/securerx/crates/securerx-node/target/release/securerx-node .

# Set default command
CMD ["./securerx-node"]
