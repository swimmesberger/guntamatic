
<a href="https://gitpod.io/#https://github.com/swimmes/guntamatic"><img align="right" src="https://img.shields.io/badge/Gitpod-ready--to--code-908a85?logo=gitpod"></a>

# What this is

This is a tiny Rust library and cli to connect to [Guntamatic](https://www.guntamatic.com/) devices via HTTP and Modbus/TCP.

```sh
guntamatic 0.3.0
simon wimmesberger <wimmesberger@gmail.com>
CLI tool to connect to and extract data from Guntamatic Devices

USAGE:
    guntamatic [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Controls the log level. ex.: -v,  -vv or -vvv
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    modbus   Accessing devices using Modbus/TCP protocol
    web      Accessing devices using web/HTTP APIs
```

The cli allows to:
 - `get`: read the current system state once
 - `stream`: read the current system state regularly and pump it into a data sink. Two sinks are available:
   - `influxdb`: [InfluxDB v2](https://docs.influxdata.com/influxdb/v2/) (also works against InfluxDB 3 via its v2 write-compatibility endpoint). Enabled by default (`sink_influxdb` feature).
   - `influxdb3`: native [InfluxDB 3](https://docs.influxdata.com/influxdb3/) writes via the `/api/v3/write_lp` API. Opt-in via the `sink_influxdb3` feature (pulls a heavier dependency tree — Arrow/gRPC — so it is not in the default build).

Example commands:
```sh
# Using web/HTTP API -> InfluxDB 2
cargo run web <device IP> <API key> stream influxdb <url> <token> <bucket> <org>

# Using Modbus/TCP -> InfluxDB 2
cargo run modbus <device IP> stream influxdb <url> <token> <bucket> <org>

# Native InfluxDB 3 sink (requires the sink_influxdb3 feature)
cargo run --features sink_influxdb3 web <device IP> <API key> stream influxdb3 <url> <token> <database>
```


Last but not least there is an example Grafana dashboard for a PowerChip heating (the only device this code has been tested with so far): ![Screenshot](./docs/pc-dashboard-screenshot.png)

# ToDo

 - [x] add modbus protocol
 - [x] CD using GH actions:
   - [x] automated build
   (- [ ] automated release to cargo - necessary?)
   - [x] build docker image and publish to GitHub Container Registry

# How to build

```sh
cargo build --release
```

Should give you a standalone executable for linux.

# Docker Usage

Pre-built Docker images are available from GitHub Container Registry for tagged releases.

## Running with Docker

The Docker image supports configuration via environment variables. All connection parameters can be set using environment variables:

```sh
docker run \
  -e GUNTAMATIC_ADDRESS=192.168.1.100 \
  -e GUNTAMATIC_TOKEN=your-api-key \
  -e INFLUXDB_URL=http://influxdb:8086 \
  -e INFLUXDB_TOKEN=your-influxdb-token \
  -e INFLUXDB_BUCKET=guntamatic \
  -e INFLUXDB_ORGANIZATION=myorg \
  ghcr.io/swimmes/guntamatic:latest
```

### Using Modbus instead of Web API

```sh
docker run \
  -e GUNTAMATIC_ADDRESS=192.168.1.100 \
  -e GUNTAMATIC_TOKEN=your-token \
  -e INFLUXDB_URL=http://influxdb:8086 \
  -e INFLUXDB_TOKEN=your-influxdb-token \
  -e INFLUXDB_BUCKET=guntamatic \
  -e INFLUXDB_ORGANIZATION=myorg \
  ghcr.io/swimmes/guntamatic:latest \
  -vv modbus stream influxdb
```

### Using the native InfluxDB 3 sink

The published Docker image is built with `--all-features`, so the `influxdb3`
sink is available out of the box:

```sh
docker run \
  -e GUNTAMATIC_ADDRESS=192.168.1.100 \
  -e GUNTAMATIC_TOKEN=your-api-key \
  -e INFLUXDB3_URL=http://influxdb3:8181 \
  -e INFLUXDB3_TOKEN=your-influxdb3-token \
  -e INFLUXDB3_DATABASE=guntamatic \
  ghcr.io/swimmes/guntamatic:latest \
  -vv web stream influxdb3
```

### Available Environment Variables

- `GUNTAMATIC_ADDRESS` (required) - IP address of the Guntamatic device
- `GUNTAMATIC_TOKEN` (required) - Authentication key
- `GUNTAMATIC_INTERFACE` (optional) - Local network interface IP to bind to
- `GUNTAMATIC_POLL_INTERVAL_SECONDS` (optional, default: 30) - Polling interval

InfluxDB 2 sink (`stream influxdb`):

- `INFLUXDB_URL` (required) - InfluxDB server URL
- `INFLUXDB_TOKEN` (required) - InfluxDB authentication token
- `INFLUXDB_BUCKET` (required) - InfluxDB bucket name
- `INFLUXDB_ORGANIZATION` (required) - InfluxDB organization name

Native InfluxDB 3 sink (`stream influxdb3`):

- `INFLUXDB3_URL` (required) - InfluxDB 3 server URL
- `INFLUXDB3_TOKEN` (optional) - Auth token; omit for an unauthenticated InfluxDB 3 Core instance
- `INFLUXDB3_DATABASE` (required) - InfluxDB 3 database name (the v3 equivalent of a v2 bucket)

### Building Docker Image Locally

```sh
# Build for linux/musl target first
cargo build --release --target x86_64-unknown-linux-musl

# Build Docker image
docker build -t guntamatic:local .
```

Or use the provided build script:
```sh
./docker-build.sh
```

# How to Contribute

Easiest is to use https://www.gitpod.io: [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/swimmes/guntamatic)