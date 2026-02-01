//! Decoding of raw Modbus register bytes into typed values.

use guntamatic_core::DataType;
use serde_json::Value;

/// Decode raw bytes into a serde_json::Value based on the data type.
///
/// # Arguments
///
/// * `bytes` - 4 bytes of raw register data (big-endian)
/// * `data_type` - The expected data type
///
/// # Returns
///
/// A serde_json::Value containing the decoded value.
pub fn decode_value(bytes: &[u8; 4], data_type: &DataType) -> Value {
    match data_type {
        DataType::Float => decode_float(bytes),
        DataType::Integer => decode_int(bytes),
        DataType::Boolean => decode_bool(bytes),
        DataType::String => decode_string(bytes),
    }
}

/// Decode a 32-bit IEEE-754 float from big-endian bytes.
fn decode_float(bytes: &[u8; 4]) -> Value {
    let value = f32::from_be_bytes(*bytes);
    // Convert to f64 for JSON, round to 2 decimal places for cleaner output
    let rounded = (value as f64 * 100.0).round() / 100.0;
    Value::Number(
        serde_json::Number::from_f64(rounded).unwrap_or_else(|| serde_json::Number::from(0)),
    )
}

/// Decode a signed 32-bit integer from big-endian bytes.
fn decode_int(bytes: &[u8; 4]) -> Value {
    let value = i32::from_be_bytes(*bytes);
    Value::Number(serde_json::Number::from(value))
}

/// Decode a boolean from the LSB of the 4-byte value.
fn decode_bool(bytes: &[u8; 4]) -> Value {
    // Boolean is stored in the LSB
    let value = bytes[3] & 0x01;
    Value::Bool(value != 0)
}

/// Decode a string (up to 4 characters, ISO-8859-1, null-terminated).
fn decode_string(bytes: &[u8; 4]) -> Value {
    let mut s = String::with_capacity(4);
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        // ISO-8859-1 is a direct mapping to Unicode for bytes 0-255
        s.push(byte as char);
    }
    Value::String(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_float_positive() {
        // 0x42 5D 33 33 = 55.3
        let bytes = [0x42, 0x5D, 0x33, 0x33];
        let value = decode_float(&bytes);
        assert_eq!(value.as_f64().unwrap(), 55.3);
    }

    #[test]
    fn test_decode_float_negative() {
        // 0xC1 A0 00 00 = -20.0
        let bytes = [0xC1, 0xA0, 0x00, 0x00];
        let value = decode_float(&bytes);
        assert_eq!(value.as_f64().unwrap(), -20.0);
    }

    #[test]
    fn test_decode_float_zero() {
        // 0x00 00 00 00 = 0.0
        let bytes = [0x00, 0x00, 0x00, 0x00];
        let value = decode_float(&bytes);
        assert_eq!(value.as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_decode_int_positive() {
        // 0x00 01 02 03 = 66051
        let bytes = [0x00, 0x01, 0x02, 0x03];
        let value = decode_int(&bytes);
        assert_eq!(value.as_i64().unwrap(), 66051);
    }

    #[test]
    fn test_decode_int_negative() {
        // 0xFF FF FF F6 = -10
        let bytes = [0xFF, 0xFF, 0xFF, 0xF6];
        let value = decode_int(&bytes);
        assert_eq!(value.as_i64().unwrap(), -10);
    }

    #[test]
    fn test_decode_bool_true() {
        let bytes = [0x00, 0x00, 0x00, 0x01];
        let value = decode_bool(&bytes);
        assert_eq!(value.as_bool().unwrap(), true);
    }

    #[test]
    fn test_decode_bool_false() {
        let bytes = [0x00, 0x00, 0x00, 0x00];
        let value = decode_bool(&bytes);
        assert_eq!(value.as_bool().unwrap(), false);
    }

    #[test]
    fn test_decode_string() {
        // "AUS" + null terminator
        let bytes = [0x41, 0x55, 0x53, 0x00];
        let value = decode_string(&bytes);
        assert_eq!(value.as_str().unwrap(), "AUS");
    }

    #[test]
    fn test_decode_string_full() {
        // "REGE" (no null terminator, full 4 chars)
        let bytes = [0x52, 0x45, 0x47, 0x45];
        let value = decode_string(&bytes);
        assert_eq!(value.as_str().unwrap(), "REGE");
    }
}
