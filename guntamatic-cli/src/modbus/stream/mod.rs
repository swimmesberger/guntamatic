use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[derive(Clone)]
pub struct Options {
    /// The interval in which to poll the device for data [seconds]
    #[arg(
        long = "interval",
        short = 'i',
        env = "GUNTAMATIC_POLL_INTERVAL_SECONDS",
        default_value = "30",
        value_parser = super::super::parse_duration
    )]
    pub interval: Duration,

    /// The sink to write DAQ data to
    #[command(subcommand)]
    pub sink: Sink,
}

#[derive(Parser)]
#[derive(Clone)]
pub enum Sink {
    #[cfg(feature = "sink_influxdb")]
    #[command(
        name = "influxdb",
        about = "Push parsed DAQ data into the configured InfluxDB"
    )]
    InfluxDB(crate::sink::influxdb::Options),
}

pub async fn exec(
    _global_opts: &super::super::Options,
    modbus_opts: &super::Options,
    opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_core::DaqData;

    let sink = opts.sink.clone();
    let modbus_opts = modbus_opts.clone();
    let opts = opts.clone();

    let (tx, rc) = flume::unbounded::<DaqData>();
    let _listener = tokio::spawn(async move {
        loop {
            info!("retrieving DAQ data via Modbus...");
            let daq_data = guntamatic_modbus::load_and_parse_daq_data(
                modbus_opts.addr.as_str(),
                modbus_opts.key.as_str(),
            )
            .await;
            
            match daq_data {
                Err(err) => error!("error while retrieving DAQ data: {}", err),
                Ok(daq_data) => {
                    debug!(
                        "sending {:?} number of entries...",
                        daq_data.values.len()
                    );
                    let res = tx.send_async(daq_data).await;
                    if let Err(err) = res {
                        error!("error while forwarding DAQ data: {}", err);
                    }
                }
            };

            debug!("waiting {:?} seconds...", opts.interval);
            tokio::time::sleep(opts.interval).await;
        }
    });

    match &sink {
        #[cfg(feature = "sink_influxdb")]
        Sink::InfluxDB(influx_opts) => {
            crate::sink::influxdb::drain(influx_opts, rc, "modbus").await?;
        }
    };
    Ok(())
}
