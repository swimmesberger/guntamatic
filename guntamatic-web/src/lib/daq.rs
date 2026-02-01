use log::trace;
use serde::Deserialize;

use guntamatic_core::{DaqData, DaqDescription, DaqValue, DataType, Unit};

#[derive(Debug, Clone, PartialEq)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct RawData {
    pub data: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct DaqDescriptionList {
    pub list: Vec<DaqDescription>,
}

pub async fn load_and_parse_daq_data(addr: &str, key: &str) -> Result<DaqData, http_types::Error> {
    let desc_url = format!("http://{}/ext/daqdesc.cgi?key={}", addr, key);
    trace!("desc url: {}", desc_url);
    let data_description: DaqDescriptionList = reqwest::get(desc_url)
        .await?
        .json()
        .await?;
    trace!("desc list:\n{:?}", data_description);

    let data_url = format!("http://{}/ext/daqdata.cgi?key={}", addr, key);
    trace!("data url: {}", data_url);
    let raw_data: RawData = reqwest::get(data_url)
        .await?
        .json()
        .await?;
    trace!("raw data:\n{:?}", raw_data);

    let values = data_description.list.into_iter()
        .zip(raw_data.data.into_iter())
        .map(|(desc, value)| {
            DaqValue {
                description: desc,
                value,
            }
        })
        .collect::<Vec<_>>();
    let daq = DaqData {
        values,
    };
    Ok(daq)
}


#[cfg(test)]
mod test {
    use super::*;
    type Result = std::result::Result<(), anyhow::Error>;

    #[test]
    pub fn test_parse_description() -> Result {
        let s = r#"[
            {"id":3,"name":"Kesseltemperatur","type":"float","unit":"°C"},
            {"id":10,"name":"Puffer T5","type":"float","unit":"°C"}
        ]"#;
        let desc: DaqDescriptionList = serde_json::de::from_str(s)?;
        assert_eq!(desc, DaqDescriptionList {
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
        });
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
        assert_eq!(raw_data, RawData {
            data: vec![
                Number(serde_json::Number::from_str("1").unwrap()),
                Number(serde_json::Number::from_f64(10.23).unwrap()),
                String("hello world!".to_string()),
                Bool(false),
            ],
        });
        Ok(())
    }
}