FROM alpine:3.19

RUN apk add bash curl

RUN mkdir -p /app
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/guntamatic /app/guntamatic
ENTRYPOINT [ "/app/guntamatic", "-vv", "web", "stream", "influxdb" ]