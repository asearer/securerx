FROM rust:1.72 as builder
WORKDIR /usr/src/securerx
# Copy workspace files
COPY Cargo.toml ./
COPY crates/securerx-core ./crates/securerx-core
COPY crates/securerx-api ./crates/securerx-api
# Build from workspace root
RUN cargo build --release --package securerx-api

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/securerx/target/release/securerx-api .
EXPOSE 8080
CMD ["./securerx-api"]
