use std::time::Duration;

use clap::Parser;

#[derive(Parser, Clone)]
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

#[derive(Parser, Clone)]
pub enum Sink {
    #[cfg(feature = "sink_influxdb")]
    #[command(name = "influxdb", about = "Push parsed DAQ data into the configured InfluxDB")]
    InfluxDB(crate::sink::influxdb::Options),

    #[cfg(feature = "sink_influxdb3")]
    #[command(
        name = "influxdb3",
        about = "Push parsed DAQ data into the configured InfluxDB 3 database"
    )]
    InfluxDB3(crate::sink::influxdb3::Options),
}

pub async fn exec(
    _global_opts: &super::super::Options,
    modbus_opts: &super::Options,
    opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_core::DaqSource;
    use guntamatic_modbus::ModbusSource;

    let sink = opts.sink.clone();
    let interval = opts.interval;

    // Connect once at startup
    info!("connecting to Modbus at {}...", modbus_opts.addr);
    let mut source =
        ModbusSource::connect(modbus_opts.addr.as_str(), modbus_opts.key.as_str()).await?;
    info!("connected to Modbus, starting polling loop");

    let (tx, rc) = flume::unbounded();
    let _listener = tokio::spawn(async move {
        loop {
            info!("retrieving DAQ data via Modbus...");
            let daq_data = source.poll().await;

            match daq_data {
                Err(err) => {
                    error!("error while retrieving DAQ data: {}", err);
                    // Attempt to reconnect on error
                    warn!("attempting to reconnect...");
                    if let Err(reconnect_err) = source.reconnect().await {
                        error!("reconnection failed: {}", reconnect_err);
                    } else {
                        info!("reconnected successfully");
                    }
                },
                Ok(daq_data) => {
                    debug!("sending {:?} number of entries...", daq_data.values.len());
                    let res = tx.send_async(daq_data).await;
                    if let Err(err) = res {
                        error!("error while forwarding DAQ data: {}", err);
                    }
                },
            };

            debug!("waiting {:?} seconds...", interval);
            tokio::time::sleep(interval).await;
        }
    });

    match &sink {
        #[cfg(feature = "sink_influxdb")]
        Sink::InfluxDB(influx_opts) => {
            crate::sink::influxdb::drain(influx_opts, rc, "modbus").await?;
        },
        #[cfg(feature = "sink_influxdb3")]
        Sink::InfluxDB3(influx_opts) => {
            crate::sink::influxdb3::drain(influx_opts, rc, "modbus").await?;
        },
    };
    Ok(())
}
