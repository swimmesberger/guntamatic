# Build stage
FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/guntamatic
COPY . .

RUN cargo build --release --all-features --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:3.19

RUN apk add bash curl

RUN mkdir -p /app
WORKDIR /app
COPY --from=builder /usr/src/guntamatic/target/x86_64-unknown-linux-musl/release/guntamatic /app/guntamatic

# Default to web protocol, but allow override via CMD
# All connection parameters can be set via environment variables:
# - GUNTAMATIC_ADDRESS (required)
# - GUNTAMATIC_TOKEN (required)
# - GUNTAMATIC_INTERFACE (optional)
# - GUNTAMATIC_POLL_INTERVAL_SECONDS (default: 30)
# - INFLUXDB_URL (required)
# - INFLUXDB_TOKEN (required)
# - INFLUXDB_BUCKET (required)
# - INFLUXDB_ORGANIZATION (required)
ENTRYPOINT [ "/app/guntamatic" ]
CMD [ "-vv", "web", "stream", "influxdb" ]