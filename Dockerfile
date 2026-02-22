# Stage 1: Build
FROM rust:1.75-slim as builder
WORKDIR /usr/src/lumina
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/lumina/target/release/lumina /app/lumina
COPY --from=builder /usr/src/lumina/config.yaml /app/config.yaml

EXPOSE 8080
CMD ["./lumina"]
