use clap::Parser;
use anyhow::anyhow;

#[derive(Parser)]
#[derive(Clone)]
pub struct Options {}

pub async fn exec(
    _global_opts: &super::super::Options,
    modbus_opts: &super::Options,
    _opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_modbus as gmodbus;

    let daq_data = gmodbus::load_and_parse_daq_data(
        modbus_opts.addr.as_str(),
        modbus_opts.key.as_str(),
    )
    .await
    .map_err(|err| anyhow!("{}", err))?;

    println!("{:#?}", daq_data);

    Ok(())
}
