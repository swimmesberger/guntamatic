//! Guntamatic Modbus/TCP client library.
//!
//! This crate provides a client for communicating with Guntamatic heating systems
//! via the Modbus/TCP protocol. It supports:
//!
//! - Authentication using the device key
//! - Reading DAQ data from Modbus registers
//! - Decoding values (float, int, bool, string) from raw register data
//! - Fetching the register mapping dynamically via HTTP
//!
//! # Example
//!
//! ```no_run
//! use guntamatic_modbus::ModbusSource;
//! use guntamatic_core::DaqSource;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut source = ModbusSource::connect("192.168.1.100:502", "your-key-here").await?;
//!     let daq_data = source.poll().await?;
//!     println!("{:?}", daq_data);
//!     Ok(())
//! }
//! ```

mod client;
mod decode;
mod mapping;

pub use client::ModbusSource;
pub use mapping::{ModbusMapping, ModbusMappingEntry, fetch_mapping};

// Re-export core types for convenience
pub use guntamatic_core::{DaqData, DaqDescription, DaqSource, DaqValue, DataType, Unit};
