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

mod client;
mod decode;
mod mapping;

pub use client::ModbusClient;
pub use mapping::{fetch_mapping, ModbusMapping, ModbusMappingEntry};

// Re-export core types for convenience
pub use guntamatic_core::{DaqData, DaqDescription, DaqValue, DataType, Unit};

/// Load and parse DAQ data from a Guntamatic device via Modbus/TCP.
///
/// This is a convenience function that creates a client, connects, authenticates,
/// fetches the mapping, and reads all DAQ data.
///
/// # Arguments
///
/// * `addr` - The address of the Modbus device (IP:port or just IP, defaults to port 502)
/// * `key` - The authentication key for the device
///
/// # Returns
///
/// Returns the parsed DAQ data or an error.
pub async fn load_and_parse_daq_data(addr: &str, key: &str) -> Result<DaqData, anyhow::Error> {
    let client = ModbusClient::new(addr, key);
    client.load_and_parse_daq_data().await
}
