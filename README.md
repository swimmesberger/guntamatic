
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
 - `stream`: read the current system state regularly and pump it into a data sink (currently [influxdb v2.0](https://docs.influxdata.com/influxdb/v2.0/) only)

Example commands:
```sh
# Using web/HTTP API
cargo run web <device IP> <API key> stream influxdb <url> <token> <bucket> <org>

# Using Modbus/TCP
cargo run modbus <device IP> stream influxdb <url> <token> <bucket> <org>
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

### Available Environment Variables

- `GUNTAMATIC_ADDRESS` (required) - IP address of the Guntamatic device
- `GUNTAMATIC_TOKEN` (required) - Authentication key
- `GUNTAMATIC_INTERFACE` (optional) - Local network interface IP to bind to
- `GUNTAMATIC_POLL_INTERVAL_SECONDS` (optional, default: 30) - Polling interval
- `INFLUXDB_URL` (required) - InfluxDB server URL
- `INFLUXDB_TOKEN` (required) - InfluxDB authentication token
- `INFLUXDB_BUCKET` (required) - InfluxDB bucket name
- `INFLUXDB_ORGANIZATION` (required) - InfluxDB organization name

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