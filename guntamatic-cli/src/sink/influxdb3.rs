use std::convert::TryInto;
use std::sync::LazyLock;

use anyhow::anyhow;
use clap::Parser;
use influxdb3_client::{Client, ClientConfig, Point, Precision};

use guntamatic_core::{DaqData, DataType};

#[derive(Parser, Clone)]
pub struct Options {
    #[arg(env = "INFLUXDB3_URL")]
    pub url: String,

    /// Authentication token. May be left empty for an unauthenticated
    /// InfluxDB 3 Core instance.
    #[arg(env = "INFLUXDB3_TOKEN", default_value = "")]
    pub token: String,

    /// Target InfluxDB 3 database (the v3 equivalent of a v2 bucket).
    #[arg(env = "INFLUXDB3_DATABASE")]
    pub database: String,
}

static WHITESPACE: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"\s+").unwrap());

pub async fn drain(
    opts: &Options,
    results_rc: flume::Receiver<DaqData>,
    source: &str,
) -> Result<(), anyhow::Error> {
    let config = ClientConfig::builder()
        .host(&opts.url)
        .token_opt((!opts.token.is_empty()).then(|| opts.token.clone()))
        .database(&opts.database)
        .build()
        .map_err(|err| anyhow!("error building influxdb3 client config: {:?}", err))?;
    let client = Client::new(config)
        .await
        .map_err(|err| anyhow!("error creating influxdb3 client: {:?}", err))?;

    loop {
        let res = receive_and_write_data(&client, &results_rc, source).await;
        if let Err(err) = res {
            error!("{:?}", err);
        }
    }
}

async fn receive_and_write_data(
    client: &Client,
    results_rc: &flume::Receiver<DaqData>,
    source: &str,
) -> Result<(), anyhow::Error> {
    use std::time::SystemTime;

    let data = results_rc
        .recv_async()
        .await
        .map_err(|err| anyhow!("error receiving DAQ data: {}", err))?;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    info!("DAQ data received ({}): at {}", source, now.as_millis());

    let points = daq_data_to_points(data, now.as_nanos().try_into().unwrap_or(i64::MAX))?;
    client
        .write(points)
        .precision(Precision::Nanosecond)
        .await
        .map_err(|err| anyhow!("error inserting into influxdb3: {:?}", err))?;
    info!("wrote DAQ data to influxdb3");

    Ok(())
}

fn daq_data_to_points(daq: DaqData, timestamp: i64) -> Result<Vec<Point>, anyhow::Error> {
    let mut points: Vec<Point> = vec![];
    for v in daq.values {
        let desc = v.description;
        let name = format!("{}_{}", desc.id, desc.name).to_lowercase();
        let name = WHITESPACE.replace_all(name.as_str(), "-");
        let point = Point::new(name.as_ref());
        let point = match desc.typ {
            DataType::Boolean => point.field("value", v.value.as_bool().unwrap_or(false)),
            DataType::Integer => point.field("value", v.value.as_i64().unwrap_or(0)),
            DataType::Float => point.field("value", v.value.as_f64().unwrap_or(0.0)),
            DataType::String => point.field("value", v.value.as_str().unwrap_or("")),
        };
        points.push(point.timestamp_nanos(timestamp));
    }
    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;
    use guntamatic_core::{DaqDescription, DaqValue};

    fn value(id: u32, name: &str, typ: DataType, value: serde_json::Value) -> DaqValue {
        DaqValue {
            value,
            description: DaqDescription { id, name: name.to_string(), typ, unit: None },
        }
    }

    #[test]
    fn converts_each_data_type_to_line_protocol() {
        let daq = DaqData {
            values: vec![
                value(1, "Boiler On", DataType::Boolean, serde_json::json!(true)),
                value(2, "Runtime", DataType::Integer, serde_json::json!(42)),
                value(3, "Temp", DataType::Float, serde_json::json!(21.5)),
                value(4, "State", DataType::String, serde_json::json!("heating")),
            ],
        };

        let points = daq_data_to_points(daq, 1_700_000_000_000_000_000).unwrap();
        assert_eq!(points.len(), 4);

        let lp: Vec<String> = points
            .iter()
            .map(|p| p.to_line_protocol(Precision::Nanosecond).unwrap())
            .collect();

        // Measurement name is "{id}_{name}" lowercased with whitespace -> "-".
        assert!(lp[0].starts_with("1_boiler-on value=true"));
        assert!(lp[1].starts_with("2_runtime value=42i"));
        assert!(lp[2].starts_with("3_temp value=21.5"));
        // String fields must be quoted line protocol.
        assert!(lp[3].starts_with("4_state value=\"heating\""));
        // Timestamp is preserved.
        assert!(lp[0].trim_end().ends_with(" 1700000000000000000"));
    }
}
