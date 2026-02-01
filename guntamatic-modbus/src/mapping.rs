//! Parsing of the Modbus register mapping from HTML.

use log::{debug, trace};
use scraper::{Html, Selector};
use thiserror::Error;

use guntamatic_core::{DaqDescription, DataType, Unit};

/// Error type for mapping operations.
#[derive(Error, Debug)]
pub enum MappingError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse HTML: {0}")]
    Parse(String),

    #[error("Invalid data type: {0}")]
    InvalidDataType(String),

    #[error("Invalid unit: {0}")]
    InvalidUnit(String),
}

/// The complete Modbus register mapping.
#[derive(Debug, Clone)]
pub struct ModbusMapping {
    pub entries: Vec<ModbusMappingEntry>,
}

/// A single entry in the Modbus mapping.
#[derive(Debug, Clone)]
pub struct ModbusMappingEntry {
    /// The channel ID (matches DAQ ID).
    pub id: u32,
    /// The register number (e.g., 0x4001).
    pub register: u32,
    /// The address (register - 1, e.g., 0x4000).
    pub address: u32,
    /// The data type.
    pub data_type: DataType,
    /// The unit of measurement.
    pub unit: Option<Unit>,
    /// Size in bytes (4 for standard values).
    pub size: u8,
    /// The channel name.
    pub name: String,
    /// Extended text register address (0x5000+) for string types with >4 chars.
    pub extended_text_address: Option<u32>,
}

impl ModbusMappingEntry {
    /// Convert this mapping entry to a DaqDescription.
    pub fn to_daq_description(&self) -> DaqDescription {
        DaqDescription {
            id: self.id,
            name: self.name.clone(),
            typ: self.data_type,
            unit: self.unit,
        }
    }
}

/// Fetch the Modbus mapping from the device via HTTP.
///
/// # Arguments
///
/// * `addr` - The HTTP address of the device (IP or IP:port).
/// * `key` - The authentication key.
///
/// # Returns
///
/// The parsed mapping or an error.
pub async fn fetch_mapping(addr: &str, key: &str) -> Result<ModbusMapping, MappingError> {
    let url = format!("http://{}/mbmap.cgi?key={}", addr, key);
    debug!("Fetching mapping from {}", url);

    let response = reqwest::get(&url).await?;
    let html_content = response.text().await?;

    trace!("Received {} bytes of HTML", html_content.len());
    parse_mapping_html(&html_content)
}

/// Parse the mapping HTML content.
fn parse_mapping_html(html: &str) -> Result<ModbusMapping, MappingError> {
    let document = Html::parse_document(html);

    let td_selector =
        Selector::parse("td").map_err(|e| MappingError::Parse(format!("{:?}", e)))?;

    // Find all tables
    let table_selector =
        Selector::parse("table").map_err(|e| MappingError::Parse(format!("{:?}", e)))?;
    let tbody_tr_selector =
        Selector::parse("tbody tr").map_err(|e| MappingError::Parse(format!("{:?}", e)))?;

    let tables: Vec<_> = document.select(&table_selector).collect();
    
    if tables.is_empty() {
        return Err(MappingError::Parse("No tables found in HTML".to_string()));
    }

    // Parse first table (main DAQ data)
    let mut entries = Vec::new();
    let first_table = tables[0];

    for row in first_table.select(&tbody_tr_selector) {
        let cells: Vec<_> = row.select(&td_selector).collect();

        // Skip rows that don't have enough cells (header rows, etc.)
        if cells.len() < 7 {
            continue;
        }

        // Parse each column:
        // 0: Id, 1: Register, 2: Adresse, 3: Typ, 4: Einheit, 5: Größe, 6: Name

        let id_text = cells[0].text().collect::<String>().trim().to_string();
        let register_text = cells[1].text().collect::<String>().trim().to_string();
        let address_text = cells[2].text().collect::<String>().trim().to_string();
        let type_text = cells[3].text().collect::<String>().trim().to_string();
        let unit_text = cells[4].text().collect::<String>().trim().to_string();
        let size_text = cells[5].text().collect::<String>().trim().to_string();
        let name_text = cells[6].text().collect::<String>().trim().to_string();

        // Skip if ID is not numeric (likely a header or invalid row)
        let id: u32 = match id_text.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };

        // Parse register and address as hex
        let register = parse_hex(&register_text).unwrap_or(0);
        let address = parse_hex(&address_text).unwrap_or(0);

        // Parse data type
        let data_type = parse_data_type(&type_text)?;

        // Parse unit
        let unit = parse_unit(&unit_text);

        // Parse size
        let size: u8 = size_text.parse().unwrap_or(4);

        entries.push(ModbusMappingEntry {
            id,
            register,
            address,
            data_type,
            unit,
            size,
            name: name_text,
            extended_text_address: None,
        });
    }

    // Parse second table (extended texts) if it exists
    // Extended texts have the same IDs as standard entries but provide longer strings
    if tables.len() > 1 {
        let extended_table = tables[1];
        for row in extended_table.select(&tbody_tr_selector) {
            let cells: Vec<_> = row.select(&td_selector).collect();
            
            // Extended text table has: Id, Register, Adresse, Größe, Name, aktueller Wert
            if cells.len() < 5 {
                continue;
            }

            let id_text = cells[0].text().collect::<String>().trim().to_string();
            let address_text = cells[2].text().collect::<String>().trim().to_string();

            let id: u32 = match id_text.parse() {
                Ok(id) => id,
                Err(_) => continue,
            };

            let extended_address = parse_hex(&address_text).unwrap_or(0);

            // Find the matching entry by ID and add the extended text address
            if let Some(entry) = entries.iter_mut().find(|e| e.id == id) {
                entry.extended_text_address = Some(extended_address);
                trace!("Entry {} ({}) has extended text at 0x{:04X}", id, entry.name, extended_address);
            }
        }
        debug!("Linked extended text addresses from second table");
    }

    debug!("Parsed {} mapping entries", entries.len());
    Ok(ModbusMapping { entries })
}

/// Parse a hex string (with or without 0x prefix).
fn parse_hex(s: &str) -> Option<u32> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(s, 16).ok()
}

/// Parse the data type string.
fn parse_data_type(s: &str) -> Result<DataType, MappingError> {
    match s.to_lowercase().as_str() {
        "float" => Ok(DataType::Float),
        "int" | "integer" => Ok(DataType::Integer),
        "bool" | "boolean" => Ok(DataType::Boolean),
        "string" => Ok(DataType::String),
        other => Err(MappingError::InvalidDataType(other.to_string())),
    }
}

/// Parse the unit string to a Unit enum.
fn parse_unit(s: &str) -> Option<Unit> {
    match s {
        "°C" => Some(Unit::DegreeCelsius),
        "%" => Some(Unit::Percent),
        "d" => Some(Unit::Days),
        "h" => Some(Unit::Hours),
        "m3" => Some(Unit::CubicMeter),
        "" | " " => None,
        _ => Some(Unit::None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_hex("0x4001"), Some(0x4001));
        assert_eq!(parse_hex("4001"), Some(0x4001));
        assert_eq!(parse_hex("0X4000"), Some(0x4000));
    }

    #[test]
    fn test_parse_data_type() {
        assert_eq!(parse_data_type("float").unwrap(), DataType::Float);
        assert_eq!(parse_data_type("int").unwrap(), DataType::Integer);
        assert_eq!(parse_data_type("bool").unwrap(), DataType::Boolean);
        assert_eq!(parse_data_type("string").unwrap(), DataType::String);
    }

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse_unit("°C"), Some(Unit::DegreeCelsius));
        assert_eq!(parse_unit("%"), Some(Unit::Percent));
        assert_eq!(parse_unit("d"), Some(Unit::Days));
        assert_eq!(parse_unit("h"), Some(Unit::Hours));
        assert_eq!(parse_unit("m3"), Some(Unit::CubicMeter));
        assert_eq!(parse_unit(""), None);
    }

    #[test]
    fn test_parse_mapping_html() {
        let html = r#"
        <html>
        <body>
            <table>
                <thead>
                    <tr><th>Id</th><th>Register</th><th>Adresse</th><th>Typ</th><th>Einheit</th><th>Größe</th><th>Name</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td>0</td>
                        <td>0x4001</td>
                        <td>0x4000</td>
                        <td>string</td>
                        <td></td>
                        <td>4</td>
                        <td>Betrieb</td>
                    </tr>
                    <tr>
                        <td>1</td>
                        <td>0x4003</td>
                        <td>0x4002</td>
                        <td>float</td>
                        <td>°C</td>
                        <td>4</td>
                        <td>Aussentemperatur</td>
                    </tr>
                </tbody>
            </table>
        </body>
        </html>
        "#;

        let mapping = parse_mapping_html(html).unwrap();
        assert_eq!(mapping.entries.len(), 2);

        assert_eq!(mapping.entries[0].id, 0);
        assert_eq!(mapping.entries[0].register, 0x4001);
        assert_eq!(mapping.entries[0].address, 0x4000);
        assert_eq!(mapping.entries[0].data_type, DataType::String);
        assert_eq!(mapping.entries[0].name, "Betrieb");

        assert_eq!(mapping.entries[1].id, 1);
        assert_eq!(mapping.entries[1].data_type, DataType::Float);
        assert_eq!(mapping.entries[1].unit, Some(Unit::DegreeCelsius));
        assert_eq!(mapping.entries[1].name, "Aussentemperatur");
    }
}
