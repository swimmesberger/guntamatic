pub mod stream;
pub mod get;

use std::net::IpAddr;


use clap::Parser;

#[derive(Parser)]
#[derive(Clone)]
pub struct Options {
    /// The IP address of the local network device to bind to. ex.: 127.0.0.1
    #[arg(
        long = "iface",
        name = "if",
        env = "GUNTAMATIC_INTERFACE",
        global = true,
        value_parser = parse_ip_addr,
    )]
    pub iface_addr: Option<IpAddr>,

    /// The address/IP of the Guntamatic device to stream data from
    #[arg(env = "GUNTAMATIC_ADDRESS")]
    pub addr: String,

    /// The key to authenticate with against the device
    #[arg(env = "GUNTAMATIC_TOKEN")]
    pub key: String,


    #[command(subcommand)]
    pub cmd: SubCmds,
}

#[derive(Parser)]
#[derive(Clone)]
pub enum SubCmds {
    #[command(
        name = "stream",
        about = "Stream DAQ data to one of various sinks"
    )]
    Stream(stream::Options),
    #[command(
        name = "get",
        about = "Get DAQ data and print it to stdout"
    )]
    Get(get::Options),
}

fn parse_ip_addr(addr: &str) -> Result<IpAddr, std::net::AddrParseError> {
    addr.parse()
}
