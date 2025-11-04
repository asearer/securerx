# ===== Build Stage =====
FROM rust:1.73 AS builder
WORKDIR /usr/src/securerx

# Copy workspace manifest for caching
COPY Cargo.toml Cargo.lock ./

# Copy only crate manifests first to cache dependencies
# This dynamically copies all Cargo.toml files in crates/
RUN for crate in crates/*; do \
        if [ -f "$crate/Cargo.toml" ]; then \
            mkdir -p "$crate"; \
            cp "$crate/Cargo.toml" "$crate/"; \
        fi; \
    done

# Copy full source of all crates
RUN for crate in crates/*; do \
        if [ -d "$crate/src" ]; then \
            mkdir -p "$crate/src"; \
            cp -r "$crate/src" "$crate/src"; \
        fi; \
    done

# Fetch dependencies for all workspace members
RUN cargo fetch

# Build the main node binary
RUN cargo build --release --package securerx-node

# ===== Runtime Stage =====
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy built binary from builder
COPY --from=builder /usr/src/securerx/target/release/securerx-node .

# Expose node ports
EXPOSE 8081 9090

# Default command
CMD ["./securerx-node"]

