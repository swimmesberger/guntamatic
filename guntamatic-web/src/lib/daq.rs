use log::{debug, trace};
use serde::Deserialize;

use guntamatic_core::{DaqData, DaqDescription, DaqSource, DaqValue};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
struct RawData {
    data: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
struct DaqDescriptionList {
    list: Vec<DaqDescription>,
}

/// A persistent HTTP source that caches DAQ descriptions.
///
/// This is more efficient than `load_and_parse_daq_data` for repeated polling, as it:
/// - Caches the DAQ descriptions (fetched once at connection time)
/// - Uses a shared reqwest client with connection pooling
/// - Only fetches data values on each poll
///
/// # Example
///
/// ```ignore
/// use guntamatic_web::WebSource;
/// use guntamatic_core::DaqSource;
///
/// let mut source = WebSource::connect("192.168.1.100", "your-key").await?;
/// loop {
///     let data = source.poll().await?;
///     println!("{:?}", data);
///     tokio::time::sleep(std::time::Duration::from_secs(30)).await;
/// }
/// ```
pub struct WebSource {
    /// HTTP client with connection pooling.
    client: reqwest::Client,
    /// The device address.
    addr: String,
    /// The authentication key.
    key: String,
    /// Cached DAQ descriptions.
    descriptions: Vec<DaqDescription>,
}

impl WebSource {
    /// Connect to a Guntamatic device and prepare for polling.
    ///
    /// This method fetches the DAQ descriptions once and caches them
    /// for use in subsequent polls.
    ///
    /// # Arguments
    ///
    /// * `addr` - The HTTP address of the device (IP or IP:port).
    /// * `key` - The authentication key for the device.
    pub async fn connect(addr: &str, key: &str) -> Result<Self, anyhow::Error> {
        let client = reqwest::Client::new();

        // Fetch descriptions once
        let desc_url = format!("http://{}/ext/daqdesc.cgi?key={}", addr, key);
        debug!("Fetching DAQ descriptions from {}", desc_url);
        let data_description: DaqDescriptionList =
            client.get(&desc_url).send().await?.json().await?;
        debug!("Cached {} DAQ descriptions", data_description.list.len());

        Ok(Self {
            client,
            addr: addr.to_string(),
            key: key.to_string(),
            descriptions: data_description.list,
        })
    }

    /// Refresh the cached descriptions from the device.
    ///
    /// Call this if you suspect the DAQ configuration has changed.
    pub async fn refresh_descriptions(&mut self) -> Result<(), anyhow::Error> {
        let desc_url = format!("http://{}/ext/daqdesc.cgi?key={}", self.addr, self.key);
        debug!("Refreshing DAQ descriptions from {}", desc_url);
        let data_description: DaqDescriptionList =
            self.client.get(&desc_url).send().await?.json().await?;
        self.descriptions = data_description.list;
        debug!("Refreshed {} DAQ descriptions", self.descriptions.len());
        Ok(())
    }
}

impl DaqSource for WebSource {
    async fn poll(&mut self) -> Result<DaqData, anyhow::Error> {
        let data_url = format!("http://{}/ext/daqdata.cgi?key={}", self.addr, self.key);
        trace!("Polling data from {}", data_url);

        let raw_data: RawData = self.client.get(&data_url).send().await?.json().await?;

        let values = self
            .descriptions
            .iter()
            .cloned()
            .zip(raw_data.data)
            .map(|(desc, value)| DaqValue { description: desc, value })
            .collect();

        debug!("Polled {} DAQ values via HTTP", self.descriptions.len());
        Ok(DaqData { values })
    }

    fn source_name(&self) -> &'static str {
        "web"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use guntamatic_core::{DataType, Unit};
    type Result = std::result::Result<(), anyhow::Error>;

    #[test]
    pub fn test_parse_description() -> Result {
        let s = r#"[
            {"id":3,"name":"Kesseltemperatur","type":"float","unit":"°C"},
            {"id":10,"name":"Puffer T5","type":"float","unit":"°C"}
        ]"#;
        let desc: DaqDescriptionList = serde_json::de::from_str(s)?;
        assert_eq!(
            desc,
            DaqDescriptionList {
                list: vec![
                    DaqDescription {
                        id: 3,
                        name: "Kesseltemperatur".to_string(),
                        typ: DataType::Float,
                        unit: Some(Unit::DegreeCelsius),
                    },
                    DaqDescription {
                        id: 10,
                        name: "Puffer T5".to_string(),
                        typ: DataType::Float,
                        unit: Some(Unit::DegreeCelsius),
                    },
                ],
            }
        );
        Ok(())
    }

    #[test]
    pub fn test_parse_raw_data() -> Result {
        use serde_json::Value::*;
        use std::str::FromStr;

        let s = r#"[
            1, 10.23, "hello world!", false
        ]"#;
        let raw_data: RawData = serde_json::de::from_str(s)?;
        assert_eq!(
            raw_data,
            RawData {
                data: vec![
                    Number(serde_json::Number::from_str("1").unwrap()),
                    Number(serde_json::Number::from_f64(10.23).unwrap()),
                    String("hello world!".to_string()),
                    Bool(false),
                ],
            }
        );
        Ok(())
    }
}
