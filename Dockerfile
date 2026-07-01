# Build stage
FROM rust:1.96-alpine AS builder

# Install build dependencies for musl target compilation
RUN apk add --no-cache \
    musl-dev \
    build-base

WORKDIR /usr/src/guntamatic
COPY . .

# Tell Cargo to use gcc as the linker for musl target (Alpine's gcc is musl)
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=gcc

RUN cargo build --release --all-features --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:3.22

RUN apk add bash curl

RUN mkdir -p /app
WORKDIR /app
COPY --from=builder /usr/src/guntamatic/target/x86_64-unknown-linux-musl/release/guntamatic /app/guntamatic

# The image is built with --all-features, so both the InfluxDB 2 (`influxdb`)
# and native InfluxDB 3 (`influxdb3`) sinks are available. The v3 sink pulls in a
# heavy dependency tree (Arrow, gRPC); if a smaller image is desired, drop
# `--all-features` from the build above to ship only the default v2 sink.
#
# Default to web protocol, but allow override via CMD
# All connection parameters can be set via environment variables:
# - GUNTAMATIC_ADDRESS (required)
# - GUNTAMATIC_TOKEN (required)
# - GUNTAMATIC_INTERFACE (optional)
# - GUNTAMATIC_POLL_INTERVAL_SECONDS (default: 30)
#
# InfluxDB 2 sink (`... stream influxdb`):
# - INFLUXDB_URL (required)
# - INFLUXDB_TOKEN (required)
# - INFLUXDB_BUCKET (required)
# - INFLUXDB_ORGANIZATION (required)
#
# Native InfluxDB 3 sink (`... stream influxdb3`):
# - INFLUXDB3_URL (required)
# - INFLUXDB3_TOKEN (optional; omit for unauthenticated InfluxDB 3 Core)
# - INFLUXDB3_DATABASE (required)
ENTRYPOINT [ "/app/guntamatic" ]
CMD [ "-vv", "web", "stream", "influxdb" ]