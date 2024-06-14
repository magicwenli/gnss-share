use crate::config::Config;
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(name = "gnss-share")]
#[command(author = "Zeeshan Ali <zeeshanak@gnome.org>\nMagicwenli <yxnian@outlook.com>")]
#[command(about = "Utility to share your GNSS device on local network.")]
#[command(
    long_about = "This utility can boardcast GNSS data from a serial device to a TCP/Unix socket. And it can also receive GNSS data from TCP/Unix socket and send it back to a serial device."
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// GNSS device node.
    pub device: String,

    /// GNSS device baudrate.
    #[arg(short, long, default_value = "115200")]
    pub baudrate: u32,

    /// TCP service IP or net iface. Default is binding all iface.
    #[arg(short, long)]
    pub interface: Option<String>,

    /// TCP service port.
    #[arg(short, long, default_value = "10110")]
    pub port: u16,

    /// Disable TCP service.
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub no_tcp: bool,

    /// Unix socket service path. Default is disable.
    #[arg(short, long)]
    pub socket_path: Option<String>,

    /// Daemonize the process
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub daemonize: bool,
}

pub fn config_from_cmdline() -> Config {
    let matches = Cli::parse();

    Config {
        dev_path: matches.device,
        port: matches.port,
        net_iface: matches.interface,
        no_tcp: matches.no_tcp,
        socket_path: matches.socket_path,
        baudrate: matches.baudrate,
        daemonize: matches.daemonize,
    }
}
