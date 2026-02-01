use std::convert::TryInto;

use anyhow::anyhow;
use clap::Parser;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use lazy_static::lazy_static;
use tokio_stream::{self as stream};

use guntamatic_core::{DaqData, DataType};

#[derive(Parser)]
#[derive(Clone)]
pub struct Options {
    #[arg(env = "INFLUXDB_URL")]
    pub url: String,

    #[arg(env = "INFLUXDB_TOKEN")]
    pub token: String,

    #[arg(env = "INFLUXDB_BUCKET")]
    pub bucket: String,

    #[arg(env = "INFLUXDB_ORGANIZATION")]
    pub org: String,
}

lazy_static! {
    static ref WHITESPACE: regex::Regex = regex::Regex::new(r"\s+").unwrap();
}

pub async fn drain(
    opts: &Options,
    results_rc: flume::Receiver<DaqData>,
    source: &str,
) -> Result<(), anyhow::Error> {
    let client = Client::new(&opts.url, &opts.org, &opts.token);

    loop {
        let res = receive_and_write_data(&client, &opts.bucket, &results_rc, source).await;
        if let Err(err) = res {
            error!("{:?}", err);
        }
    }
}

async fn receive_and_write_data(
    client: &Client,
    bucket: &str,
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
        .write(bucket, stream::iter(points))
        .await
        .map_err(|err| anyhow!("error inserting into influxdb: {:?}", err))?;
    info!("wrote DAQ data to influxdb");

    Ok(())
}

fn daq_data_to_points(daq: DaqData, timestamp: i64) -> Result<Vec<DataPoint>, anyhow::Error> {
    let mut points: Vec<DataPoint> = vec![];
    for v in daq.values {
        let desc = v.description;
        let name = format!("{}_{}", desc.id, desc.name).to_lowercase();
        let name = WHITESPACE.replace_all(name.as_str(), "-");
        let builder = DataPoint::builder(name);
        let builder = match desc.typ {
            DataType::Boolean => builder.field("value", v.value.as_bool().unwrap_or(false)),
            DataType::Integer => builder.field("value", v.value.as_i64().unwrap_or(0)),
            DataType::Float => builder.field("value", v.value.as_f64().unwrap_or(0.0)),
            DataType::String => builder.field("value", v.value.as_str().unwrap_or("")),
        };
        points.push(builder.timestamp(timestamp).build()?);
    }
    Ok(points)
}
