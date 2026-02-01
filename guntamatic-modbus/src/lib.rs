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
//! use guntamatic_modbus::ModbusClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ModbusClient::new("192.168.1.100:502", "your-key-here");
//!     let daq_data = client.load_and_parse_daq_data().await?;
//!     println!("{:?}", daq_data);
//!     Ok(())
//! }
//! ```

//! Guntamatic Modbus/TCP client library.
//!
//! This crate provides a client for communicating with Guntamatic heating systems
//! via the Modbus/TCP protocol.
//!
//! # Example
//!
//! ```ignore
//! use guntamatic_modbus::ModbusSource;
//! use guntamatic_core::DaqSource;
//!
//! let mut source = ModbusSource::connect("192.168.1.100", "your-key").await?;
//! loop {
//!     let data = source.poll().await?;
//!     println!("{:?}", data);
//!     tokio::time::sleep(std::time::Duration::from_secs(30)).await;
//! }
//! ```

mod client;
mod decode;
mod mapping;

pub use client::ModbusSource;
pub use mapping::{ModbusMapping, ModbusMappingEntry, fetch_mapping};

// Re-export core types for convenience
pub use guntamatic_core::{DaqData, DaqDescription, DaqSource, DaqValue, DataType, Unit};
