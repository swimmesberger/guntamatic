//! Modbus/TCP client for Guntamatic devices.

use std::net::SocketAddr;

use log::{debug, trace};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;

use guntamatic_core::{DaqData, DaqValue, DataType};

use crate::decode::decode_value;
use crate::mapping::{fetch_mapping, ModbusMapping};

/// Guntamatic Modbus TCP client.
pub struct ModbusClient {
    /// The socket address of the Modbus device (IP:502).
    addr: SocketAddr,
    /// The HTTP address for fetching mapping (IP without port or with port 80).
    http_addr: String,
    /// The authentication key.
    key: String,
}

impl ModbusClient {
    /// Create a new Modbus client.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the device. Can be "IP:port" or just "IP" (defaults to Modbus port 502).
    /// * `key` - The authentication key for the device.
    pub fn new(addr: &str, key: &str) -> Self {
        let (socket_addr, http_addr) = parse_address(addr);
        Self {
            addr: socket_addr,
            http_addr,
            key: key.to_string(),
        }
    }

    /// Load and parse all DAQ data from the device.
    ///
    /// This method:
    /// 1. Fetches the register mapping via HTTP
    /// 2. Connects to Modbus TCP
    /// 3. Authenticates with the key
    /// 4. Reads all DAQ registers
    /// 5. Decodes values based on their types
    pub async fn load_and_parse_daq_data(&self) -> Result<DaqData, anyhow::Error> {
        // Fetch mapping via HTTP
        debug!("Fetching Modbus mapping from {}", self.http_addr);
        let mapping = fetch_mapping(&self.http_addr, &self.key).await?;
        trace!("Mapping has {} entries", mapping.entries.len());

        // Connect to Modbus
        debug!("Connecting to Modbus at {}", self.addr);
        let mut ctx = tcp::connect(self.addr).await?;

        // Authenticate
        debug!("Authenticating...");
        self.authenticate(&mut ctx).await?;

        // Read DAQ data
        debug!("Reading DAQ data...");
        let values = self.read_daq_data(&mut ctx, &mapping).await?;

        Ok(DaqData { values })
    }

    /// Authenticate with the device by writing the key to registers 0x0101+.
    async fn authenticate(&self, ctx: &mut tokio_modbus::client::Context) -> Result<(), anyhow::Error> {
        let key_registers = encode_key(&self.key);
        trace!(
            "Writing {} key registers to 0x0100",
            key_registers.len()
        );

        // Write key to registers starting at address 0x0100 (register 0x0101)
        ctx.write_multiple_registers(0x0100, &key_registers).await??;

        debug!("Authentication complete");
        Ok(())
    }

    /// Read all DAQ data based on the mapping.
    async fn read_daq_data(
        &self,
        ctx: &mut tokio_modbus::client::Context,
        mapping: &ModbusMapping,
    ) -> Result<Vec<DaqValue>, anyhow::Error> {
        let mut values = Vec::with_capacity(mapping.entries.len());

        // Read registers in batches to optimize network efficiency
        // Each value uses 2 registers (4 bytes), max read is typically 125 registers
        const MAX_REGISTERS_PER_READ: u16 = 124; // Even number for complete values

        // Find the range of addresses we need to read
        if mapping.entries.is_empty() {
            return Ok(values);
        }

        let min_addr = mapping
            .entries
            .iter()
            .map(|e| e.address)
            .min()
            .unwrap_or(0x4000);
        let max_addr = mapping
            .entries
            .iter()
            .map(|e| e.address)
            .max()
            .unwrap_or(0x4000);

        // Read all needed registers
        let mut all_registers: Vec<u16> = Vec::new();
        let total_registers = (max_addr - min_addr + 2) as u16;

        let mut offset = 0u16;
        while offset < total_registers {
            let count = std::cmp::min(MAX_REGISTERS_PER_READ, total_registers - offset);
            let start_addr = min_addr as u16 + offset;

            trace!(
                "Reading {} registers from address 0x{:04X}",
                count,
                start_addr
            );

            let response = ctx.read_input_registers(start_addr, count).await??;
            all_registers.extend(response);

            offset += count;
        }

        // Decode each value from the registers
        for entry in &mapping.entries {
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

            // Each value spans 2 registers (4 bytes)
            let raw_bytes = [
                (all_registers[reg_offset] >> 8) as u8,
                (all_registers[reg_offset] & 0xFF) as u8,
                (all_registers[reg_offset + 1] >> 8) as u8,
                (all_registers[reg_offset + 1] & 0xFF) as u8,
            ];

            let mut value = decode_value(&raw_bytes, &entry.data_type);

            // If this entry has extended text available, read the full 64-byte string
            if let Some(ext_addr) = entry.extended_text_address {
                match self.read_extended_text(ctx, ext_addr).await {
                    Ok(ext_text) => {
                        trace!("Read extended text for id {} ({}): {}", entry.id, entry.name, ext_text);
                        value = serde_json::Value::String(ext_text);
                    }
                    Err(e) => {
                        log::warn!("Failed to read extended text for id {}: {}", entry.id, e);
                        // Keep the 4-char value we already decoded
                    }
                }
            }

            values.push(DaqValue {
                value,
                description: entry.to_daq_description(),
            });
        }

        debug!("Read {} DAQ values", values.len());
        Ok(values)
    }

    /// Read extended text (up to 64 characters) from the extended text register range.
    async fn read_extended_text(
        &self,
        ctx: &mut tokio_modbus::client::Context,
        address: u32,
    ) -> Result<String, anyhow::Error> {
        // Extended texts occupy 32 registers (64 bytes)
        const EXTENDED_TEXT_REGISTERS: u16 = 32;

        trace!("Reading {} registers from address 0x{:04X}", EXTENDED_TEXT_REGISTERS, address);
        
        let registers = ctx
            .read_input_registers(address as u16, EXTENDED_TEXT_REGISTERS)
            .await??;

        // Convert registers to bytes
        let mut bytes = Vec::with_capacity(64);
        for reg in registers {
            bytes.push((reg >> 8) as u8);
            bytes.push((reg & 0xFF) as u8);
        }

        // Find null terminator
        let mut text = String::new();
        for &byte in &bytes {
            if byte == 0 {
                break;
            }
            // ISO-8859-1 encoding: direct mapping to Unicode
            text.push(byte as char);
        }

        Ok(text)
    }
}

/// Parse the address string into socket address and HTTP address.
fn parse_address(addr: &str) -> (SocketAddr, String) {
    // Check if address already contains a port
    if addr.contains(':') {
        // Try to parse as socket address
        if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
            // Extract IP for HTTP (use port 80 for HTTP)
            let http_addr = socket_addr.ip().to_string();
            return (socket_addr, http_addr);
        }
    }

    // No port specified, default to Modbus port 502
    let socket_addr: SocketAddr = format!("{}:502", addr)
        .parse()
        .expect("Invalid IP address");
    let http_addr = addr.to_string();

    (socket_addr, http_addr)
}

/// Encode the authentication key for Modbus register writing.
///
/// The key is encoded as follows:
/// 1. Each character is converted to its ASCII hex code
/// 2. Two consecutive characters are packed into one 16-bit register
/// 3. A 0x0000 terminator is appended
fn encode_key(key: &str) -> Vec<u16> {
    let mut registers = Vec::with_capacity((key.len() / 2) + 1);

    let bytes = key.as_bytes();
    for chunk in bytes.chunks(2) {
        let high = chunk[0] as u16;
        let low = if chunk.len() > 1 { chunk[1] as u16 } else { 0 };
        registers.push((high << 8) | low);
    }

    // Add terminator
    registers.push(0x0000);

    registers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_key() {
        // Example from documentation: "22" -> 0x3232
        let key = "2221C5AE387A344F963EFA533881ACCE63AA";
        let encoded = encode_key(key);

        // First register: '2' '2' -> 0x32 0x32 -> 0x3232
        assert_eq!(encoded[0], 0x3232);
        // Second register: '2' '1' -> 0x32 0x31 -> 0x3231
        assert_eq!(encoded[1], 0x3231);
        // Third register: 'C' '5' -> 0x43 0x35 -> 0x4335
        assert_eq!(encoded[2], 0x4335);

        // Last register should be terminator
        assert_eq!(*encoded.last().unwrap(), 0x0000);

        // Key length 36 / 2 + 1 terminator = 19 registers
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
