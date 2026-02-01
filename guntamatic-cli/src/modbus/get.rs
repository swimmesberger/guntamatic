use clap::Parser;

#[derive(Parser, Clone)]
pub struct Options {}

pub async fn exec(
    _global_opts: &super::super::Options,
    modbus_opts: &super::Options,
    _opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_core::DaqSource;
    use guntamatic_modbus::ModbusSource;

    let mut source =
        ModbusSource::connect(modbus_opts.addr.as_str(), modbus_opts.key.as_str()).await?;

    let daq_data = source.poll().await?;
    println!("{:#?}", daq_data);

    Ok(())
}
