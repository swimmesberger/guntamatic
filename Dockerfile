FROM alpine:3.19

RUN apk add bash curl

RUN mkdir -p /app
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/guntamatic /app/guntamatic

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