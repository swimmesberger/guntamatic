//! Modbus/TCP client for Guntamatic devices.

use std::net::SocketAddr;

use log::{debug, trace};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;

use guntamatic_core::{DaqData, DaqSource, DaqValue};

use crate::decode::decode_value;
use crate::mapping::{ModbusMapping, fetch_mapping};

/// Parse the address string into socket address and HTTP address.
fn parse_address(addr: &str) -> (SocketAddr, String) {
    if addr.contains(':') {
        if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
            let http_addr = socket_addr.ip().to_string();
            return (socket_addr, http_addr);
        }
    }

    let socket_addr: SocketAddr = format!("{}:502", addr).parse().expect("Invalid IP address");
    let http_addr = addr.to_string();

    (socket_addr, http_addr)
}

/// Encode the authentication key for Modbus register writing.
fn encode_key(key: &str) -> Vec<u16> {
    let mut registers = Vec::with_capacity((key.len() / 2) + 1);

    let bytes = key.as_bytes();
    for chunk in bytes.chunks(2) {
        let high = chunk[0] as u16;
        let low = if chunk.len() > 1 { chunk[1] as u16 } else { 0 };
        registers.push((high << 8) | low);
    }

    registers.push(0x0000);
    registers
}

/// A persistent Modbus source that maintains a connection and caches the mapping.
///
/// This efficiently polls DAQ data by:
/// - Keeping the TCP connection open
/// - Caching the register mapping (fetched once at connection time)
/// - Only re-authenticating when reconnecting
///
/// # Example
///
/// ```ignore
/// use guntamatic_modbus::ModbusSource;
/// use guntamatic_core::DaqSource;
///
/// let mut source = ModbusSource::connect("192.168.1.100", "your-key").await?;
/// loop {
///     let data = source.poll().await?;
///     println!("{:?}", data);
///     tokio::time::sleep(std::time::Duration::from_secs(30)).await;
/// }
/// ```
pub struct ModbusSource {
    ctx: tokio_modbus::client::Context,
    mapping: ModbusMapping,
    key: String,
    addr: SocketAddr,
}

impl ModbusSource {
    /// Connect to a Guntamatic device and prepare for polling.
    ///
    /// This method:
    /// 1. Fetches the register mapping via HTTP (once)
    /// 2. Establishes a TCP connection to Modbus
    /// 3. Authenticates with the device
    pub async fn connect(addr: &str, key: &str) -> Result<Self, anyhow::Error> {
        let (socket_addr, http_addr) = parse_address(addr);

        debug!("Fetching Modbus mapping from {}", http_addr);
        let mapping = fetch_mapping(&http_addr, key).await?;
        trace!("Mapping has {} entries", mapping.entries.len());

        debug!("Connecting to Modbus at {}", socket_addr);
        let mut ctx = tcp::connect(socket_addr).await?;

        debug!("Authenticating...");
        let key_registers = encode_key(key);
        ctx.write_multiple_registers(0x0100, &key_registers)
            .await??;
        debug!("Authentication complete");

        Ok(Self { ctx, mapping, key: key.to_string(), addr: socket_addr })
    }

    /// Reconnect to the device (e.g., after a connection failure).
    ///
    /// Re-establishes the TCP connection and re-authenticates,
    /// but reuses the cached mapping.
    pub async fn reconnect(&mut self) -> Result<(), anyhow::Error> {
        debug!("Reconnecting to Modbus at {}", self.addr);
        self.ctx = tcp::connect(self.addr).await?;

        debug!("Re-authenticating...");
        let key_registers = encode_key(&self.key);
        self.ctx
            .write_multiple_registers(0x0100, &key_registers)
            .await??;
        debug!("Re-authentication complete");

        Ok(())
    }

    async fn read_daq_data(&mut self) -> Result<Vec<DaqValue>, anyhow::Error> {
        let mut values = Vec::with_capacity(self.mapping.entries.len());

        const MAX_REGISTERS_PER_READ: u16 = 124;

        if self.mapping.entries.is_empty() {
            return Ok(values);
        }

        let min_addr = self
            .mapping
            .entries
            .iter()
            .map(|e| e.address)
            .min()
            .unwrap_or(0x4000);
        let max_addr = self
            .mapping
            .entries
            .iter()
            .map(|e| e.address)
            .max()
            .unwrap_or(0x4000);

        let mut all_registers: Vec<u16> = Vec::new();
        let total_registers = (max_addr - min_addr + 2) as u16;

        let mut offset = 0u16;
        while offset < total_registers {
            let count = std::cmp::min(MAX_REGISTERS_PER_READ, total_registers - offset);
            let start_addr = min_addr as u16 + offset;

            trace!("Reading {} registers from address 0x{:04X}", count, start_addr);

            let response = self.ctx.read_input_registers(start_addr, count).await??;
            all_registers.extend(response);

            offset += count;
        }

        // Collect entries with extended text addresses first to avoid borrow conflicts
        let entries_with_ext: Vec<_> = self
            .mapping
            .entries
            .iter()
            .filter_map(|e| e.extended_text_address.map(|addr| (e.id, addr)))
            .collect();

        // Read all extended texts
        let mut ext_texts: std::collections::HashMap<u32, String> =
            std::collections::HashMap::new();
        for (id, ext_addr) in entries_with_ext {
            match read_extended_text(&mut self.ctx, ext_addr).await {
                Ok(ext_text) => {
                    ext_texts.insert(id, ext_text);
                },
                Err(e) => {
                    log::warn!("Failed to read extended text for id {}: {}", id, e);
                },
            }
        }

        for entry in &self.mapping.entries {
            let reg_offset = (entry.address - min_addr) as usize;

            if reg_offset + 1 >= all_registers.len() {
                log::warn!(
                    "Register offset {} out of bounds for entry {} ({})",
                    reg_offset,
                    entry.id,
                    entry.name
                );
                continue;
            }

            let raw_bytes = [
                (all_registers[reg_offset] >> 8) as u8,
                (all_registers[reg_offset] & 0xFF) as u8,
                (all_registers[reg_offset + 1] >> 8) as u8,
                (all_registers[reg_offset + 1] & 0xFF) as u8,
            ];

            let mut value = decode_value(&raw_bytes, &entry.data_type);

            if let Some(ext_text) = ext_texts.get(&entry.id) {
                trace!("Using extended text for id {} ({}): {}", entry.id, entry.name, ext_text);
                value = serde_json::Value::String(ext_text.clone());
            }

            values.push(DaqValue { value, description: entry.to_daq_description() });
        }

        debug!("Read {} DAQ values", values.len());
        Ok(values)
    }
}

async fn read_extended_text(
    ctx: &mut tokio_modbus::client::Context,
    address: u32,
) -> Result<String, anyhow::Error> {
    const EXTENDED_TEXT_REGISTERS: u16 = 32;

    trace!("Reading {} registers from address 0x{:04X}", EXTENDED_TEXT_REGISTERS, address);

    let registers = ctx
        .read_input_registers(address as u16, EXTENDED_TEXT_REGISTERS)
        .await??;

    let mut bytes = Vec::with_capacity(64);
    for reg in registers {
        bytes.push((reg >> 8) as u8);
        bytes.push((reg & 0xFF) as u8);
    }

    let mut text = String::new();
    for &byte in &bytes {
        if byte == 0 {
            break;
        }
        text.push(byte as char);
    }

    Ok(text)
}

impl DaqSource for ModbusSource {
    async fn poll(&mut self) -> Result<DaqData, anyhow::Error> {
        debug!("Polling Modbus at {}", self.addr);
        let values = self.read_daq_data().await?;
        Ok(DaqData { values })
    }

    fn source_name(&self) -> &'static str {
        "modbus"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_key() {
        let key = "2221C5AE387A344F963EFA533881ACCE63AA";
        let encoded = encode_key(key);

        assert_eq!(encoded[0], 0x3232);
        assert_eq!(encoded[1], 0x3231);
        assert_eq!(encoded[2], 0x4335);
        assert_eq!(*encoded.last().unwrap(), 0x0000);
        assert_eq!(encoded.len(), 19);
    }

    #[test]
    fn test_parse_address_with_port() {
        let (socket, http) = parse_address("192.168.1.100:502");
        assert_eq!(socket.port(), 502);
        assert_eq!(http, "192.168.1.100");
    }

    #[test]
    fn test_parse_address_without_port() {
        let (socket, http) = parse_address("192.168.1.100");
        assert_eq!(socket.port(), 502);
        assert_eq!(http, "192.168.1.100");
    }
}
