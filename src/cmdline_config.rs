/* vim: set et ts=4 sw=4: */
/* cmdline_config.rs
 *
 * Copyright (C) 2017 Pelagicore AB.
 * Copyright (C) 2017 Zeeshan Ali.
 *
 * GPSShare is free software; you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * GPSShare is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License along
 * with GPSShare; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author: Zeeshan Ali <zeeshanak@gnome.org>
 */

use clap::Parser;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "gps-share")]
#[command(author = "Zeeshan Ali <zeeshanak@gnome.org>\nMagicwenli <yxnian@outlook.com>")]
#[command(about = "Utility to share your GNSS device on local network.")]
pub struct Cli {
    /// GPS device node
    pub device: String,

    /// Baudrate to use for communication with GPS device
    #[arg(short, long, default_value = "115200")]
    pub baudrate: u32,

    /// Port to run TCP service on
    #[arg(short, long, default_value = "10110")]
    pub port: u16,

    /// Bind specific network interface
    #[arg(short, long)]
    pub interface: Option<String>,

    /// Don't share over TCP
    #[arg(short, long, default_value = "false")]
    pub no_tcp: bool,

    /// Path to place the socket service
    #[arg(short, long)]
    pub socket_path: Option<String>,
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
    }
}
