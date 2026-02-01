use std::time::Duration;

use clap::Parser;

// Prometheus sink is not yet implemented
// #[cfg(feature = "sink_prometheus")]
// mod sink_prometheus;

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
    // #[cfg(feature = "sink_prometheus")]
    // #[clap(
    //     name = "prometheus",
    //     about = "Serve measurements as prometheus metrics"
    // )]
    // Prometheus(sink_prometheus::Options),
    #[cfg(feature = "sink_influxdb")]
    #[command(name = "influxdb", about = "Push parsed DAQ data into the configured InfluxDB")]
    InfluxDB(crate::sink::influxdb::Options),
}

pub async fn exec(
    _global_opts: &super::super::Options,
    web_opts: &super::Options,
    opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_core::DaqSource;
    use guntamatic_web::WebSource;

    let sink = opts.sink.clone();
    let interval = opts.interval;

    // Connect once at startup
    info!("connecting to web API at {}...", web_opts.addr);
    let mut source = WebSource::connect(web_opts.addr.as_str(), web_opts.key.as_str()).await?;
    info!("connected to web API, starting polling loop");

    let (tx, rc) = flume::unbounded();
    let _listener = tokio::spawn(async move {
        loop {
            info!("retrieving DAQ data...");
            let daq_data = source.poll().await;
            match daq_data {
                Err(err) => error!("error while retrieving DAQ data: {}", err),
                Ok(daq_data) => {
                    debug!("sending {:?} number of entries seconds...", daq_data.values.len());
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
        // Future prometheus sink implementation
        // #[cfg(feature = "sink_prometheus")]
        // Sink::Prometheus(prom_opts) => {
        //     sink_prometheus::serve_metrics(prom_opts, rc).await?;
        // },
        #[cfg(feature = "sink_influxdb")]
        Sink::InfluxDB(influx_opts) => {
            crate::sink::influxdb::drain(influx_opts, rc, "web").await?;
        },
    };
    Ok(())
}
