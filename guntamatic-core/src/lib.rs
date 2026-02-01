//! Core types shared between guntamatic-web and guntamatic-modbus.

use std::fmt::Display;
use async_trait::async_trait;
use serde::Deserialize;

/// A trait for DAQ data sources that can be polled for data.
///
/// This abstraction allows different data sources (Modbus, HTTP) to be used
/// interchangeably in polling loops, while managing their connections efficiently.
///
/// # Example
///
/// ```ignore
/// use guntamatic_core::DaqSource;
///
/// async fn poll_loop<S: DaqSource>(source: &mut S) {
///     loop {
///         match source.poll().await {
///             Ok(data) => println!("Got {} values", data.values.len()),
///             Err(e) => eprintln!("Error: {}", e),
///         }
///         tokio::time::sleep(std::time::Duration::from_secs(30)).await;
///     }
/// }
/// ```
#[async_trait]
pub trait DaqSource: Send {
    /// Poll for current DAQ data.
    ///
    /// This method should efficiently fetch the current values, reusing any
    /// existing connections or cached metadata where possible.
    async fn poll(&mut self) -> Result<DaqData, anyhow::Error>;

    /// Returns the name of this source type (e.g., "modbus", "web").
    fn source_name(&self) -> &'static str;
}

/// Container for all DAQ (Data Acquisition) values.
#[derive(Debug, Clone, PartialEq)]
pub struct DaqData {
    pub values: Vec<DaqValue>,
}

/// A single DAQ value with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct DaqValue {
    pub value: serde_json::Value,
    pub description: DaqDescription,
}

/// Metadata describing a DAQ channel.
#[derive(Debug, Clone, PartialEq)]
#[derive(Deserialize)]
pub struct DaqDescription {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub typ: DataType,
    pub unit: Option<Unit>,
}

/// The data type of a DAQ value.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DataType {
    Float,
    Integer,
    Boolean,
    String,
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float" => Self::Float,
            "int" | "integer" => Self::Integer,
            "bool" | "boolean" => Self::Boolean,
            "string" => Self::String,
            v => {
                return Err(Error::unknown_variant(
                    v,
                    &["float", "int", "integer", "bool", "boolean", "string"],
                ))
            }
        })
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::Integer => write!(f, "int"),
            Self::Boolean => write!(f, "bool"),
            Self::String => write!(f, "string"),
        }
    }
}

/// The unit of measurement for a DAQ value.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Unit {
    DegreeCelsius,
    Percent,
    Days,
    Hours,
    CubicMeter,
    None,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::DegreeCelsius => "°C",
            Self::Percent => "%",
            Self::Days => "d",
            Self::Hours => "h",
            Self::CubicMeter => "m3",
            Self::None => "",
        };
        write!(f, "{}", str)
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "°C" => Self::DegreeCelsius,
            "%" => Self::Percent,
            "d" => Self::Days,
            "h" => Self::Hours,
            "m3" => Self::CubicMeter,
            "" | " " => Self::None,
            v => {
                return Err(Error::unknown_variant(v, &["°C", "%", "d", "h", "m3", ""]))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatype_deserialize() {
        let float: DataType = serde_json::from_str(r#""float""#).unwrap();
        assert_eq!(float, DataType::Float);

        let int: DataType = serde_json::from_str(r#""int""#).unwrap();
        assert_eq!(int, DataType::Integer);

        let integer: DataType = serde_json::from_str(r#""integer""#).unwrap();
        assert_eq!(integer, DataType::Integer);

        let bool_val: DataType = serde_json::from_str(r#""bool""#).unwrap();
        assert_eq!(bool_val, DataType::Boolean);

        let string: DataType = serde_json::from_str(r#""string""#).unwrap();
        assert_eq!(string, DataType::String);
    }

    #[test]
    fn test_unit_deserialize() {
        let celsius: Unit = serde_json::from_str(r#""°C""#).unwrap();
        assert_eq!(celsius, Unit::DegreeCelsius);

        let percent: Unit = serde_json::from_str(r#""%""#).unwrap();
        assert_eq!(percent, Unit::Percent);
    }
}
