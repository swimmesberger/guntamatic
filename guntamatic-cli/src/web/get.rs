use clap::Parser;

#[derive(Parser, Clone)]
pub struct Options {}

pub async fn exec(
    _global_opts: &super::super::Options,
    web_opts: &super::Options,
    _opts: &Options,
) -> Result<(), anyhow::Error> {
    use guntamatic_core::DaqSource;
    use guntamatic_web::WebSource;

    let mut source = WebSource::connect(web_opts.addr.as_str(), web_opts.key.as_str()).await?;
    let daq_data = source.poll().await?;
    println!("{:#?}", daq_data);

    Ok(())
}
