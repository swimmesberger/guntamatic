# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Native InfluxDB 3 sink (`stream influxdb3`) using the `influxdb3-client` crate
  and the `/api/v3/write_lp` API, behind the opt-in `sink_influxdb3` feature
  (configured via `INFLUXDB3_URL`/`INFLUXDB3_TOKEN`/`INFLUXDB3_DATABASE`)
- GitHub Actions CI/CD workflows for automated testing, linting, and releases
- Comprehensive linting configuration with rustfmt and clippy
- Code coverage reporting with cargo-llvm-cov
- Integration tests for CLI commands
- Security audit workflow
- Dependabot configuration for automated dependency updates
- Docker image publishing to GitHub Container Registry
- Project metadata (description, repository, license, keywords, categories)
- License files (dual MIT/Apache-2.0)
- This CHANGELOG

### Changed
- **BREAKING**: Upgraded to Rust Edition 2024
- Migrated from `lazy_static` to `std::sync::LazyLock` (stdlib)
- Migrated from `async-trait` crate to native async fn in traits
- Replaced `pretty_env_logger` with `env_logger`
- Implemented workspace dependency management for unified versions
- Unified `reqwest` to version 0.13 across the workspace crates, pinned to the
  rustls `ring` provider (installed process-wide in `main`) so the musl Docker
  build does not need to compile `aws-lc-rs` (which requires cmake/perl)
- Upgraded `thiserror` to version 2.0 across all crates
- Updated `tokio` to 1.49
- Bumped dependencies to latest, including major versions: `reqwest` 0.13,
  `serde_qs` 1.1, `scraper` 0.27, `tokio-modbus` 0.17, `http` 1.4, `clap` 4.6,
  `regex` 1.12, `ctrlc` 3.5
- Bumped Docker build image to Rust 1.96 (alpine) and runtime to alpine 3.22

### Fixed
- Dependency version conflicts between crates

### Removed
- `lazy_static` dependency (replaced by stdlib)
- `async-trait` dependency (replaced by native Rust feature)
- `pretty_env_logger` dependency (replaced by `env_logger`)

## [0.3.0] - 2024-01-xx

### Added
- Initial release with Modbus and Web support
- InfluxDB sink for data export
- Support for Guntamatic heating device monitoring

[Unreleased]: https://github.com/swimmes/guntamatic/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/swimmes/guntamatic/releases/tag/v0.3.0
