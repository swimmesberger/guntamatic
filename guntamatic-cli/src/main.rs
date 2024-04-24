use clap::Parser;

#[macro_use] extern crate log;

use std::time::Duration;

mod web;

#[derive(Parser)]
#[command(
    name = "guntamatic",
    version = "0.2.1",
    author = "swimmes <wimmesberger@gmail.com>",
    about = "CLI tool to connect to and extract data from Guntamatic Devices",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Options {
    /// Controls the log level. ex.: -v,  -vv or -vvv
    #[arg(
        short = 'v',
        long = "verbose",
        action = clap::ArgAction::Count
    )]
    verbose: u8,

    #[command(subcommand)]
    cmd: SubCmds,
}
#[derive(Parser)]
pub enum SubCmds {
    #[command(
        name = "web",
        about = "Accessing devices using web/HTTP APIs"
    )]
    Web(web::Options),
}


fn parse_duration(secs_str: &str) -> Result<Duration, std::num::ParseIntError> {
    use std::str::FromStr;
    let secs = u64::from_str(secs_str)?;
    Ok(Duration::from_secs(secs))
}

type AResult<T> = Result<T, anyhow::Error>;

//noinspection RsUnreachableCode
#[tokio::main]
async fn main() -> AResult<()> {
    let options = Options::parse();

    // initialize logger
    let log_level = match options.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log_level)
        .init();
    
    // set ctrl-c handler
    let (exit_tx, exit_rc) = flume::unbounded::<(bool, i32)>();
    let ctrl_c_tx = exit_tx.clone();
    ctrlc::set_handler(move || {
        let _ = ctrl_c_tx.try_send((true, 0));
    })?;

    // spawn actual streamer
    let task = tokio::spawn(async move {
        let result = execute(options).await;
        let rc = if let Err(e) = result {
            error!("{}", e);
            -1
        } else {
            0
        };
        let _ = exit_tx.try_send((false, rc));
    });


    let (ctrl_c, rc) = exit_rc.recv_async().await?;
    if ctrl_c {
        info!("received Ctrl-C, quitting.");
        task.abort();
    } else {
        debug!("task completed, quitting.")
    }

    std::process::exit(rc);

    #[allow(unreachable_code)]
    Ok(())
}

async fn execute(options: Options) -> Result<(), anyhow::Error> {
    use anyhow::anyhow;

    match &options.cmd {
        SubCmds::Web(web_opts) => {
            match &web_opts.cmd {
                web::SubCmds::Stream(stream_opts) => {
                    web::stream::exec(&options, &web_opts, stream_opts)
                        .await
                        .map_err(|err| anyhow!("error while streaming DAQ data: {}", err))?;
                },
                web::SubCmds::Get(get_opts) => {
                    web::get::exec(&options, &web_opts, get_opts)
                        .await?;
                }
            }
        },
    }
    Ok(())
}
