/* vim: set et ts=4 sw=4: */
/* main.rs
 *
 * Copyright (C) 2024 Magicwenli.
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

mod cmdline_config;
mod config;
mod server;
mod stream;

use config::Config;
use server::Server;
use signal_hook;
use signal_hook::consts as signals;
use std::io;
use std::sync::mpsc;
use std::thread;

use std::rc::Rc;

enum DoneReason {
    Signal(i32),
    Success,
}

/// Stolen directly from crate chan-signal.
fn notify(signals: &[i32], s: mpsc::Sender<DoneReason>) -> Result<(), io::Error> {
    let mut signals = signal_hook::iterator::Signals::new(signals)?;
    thread::spawn(move || {
        for signal in signals.forever() {
            if s.send(DoneReason::Signal(signal)).is_err() {
                break;
            }
        }
    });
    Ok(())
}

fn main() {
    let config = cmdline_config::config_from_cmdline();

    let (sdone, rdone) = mpsc::channel();
    notify(&[signals::SIGINT, signals::SIGTERM], sdone.clone()).unwrap();

    thread::spawn(move || run(sdone, Rc::new(config)));

    match rdone.recv().unwrap() {
        DoneReason::Signal(signals::SIGINT) => {
            println!("Interrupt from keyboard. Exitting..");
        }

        DoneReason::Signal(signals::SIGTERM) => {
            println!("Kill signal received. Exitting..");
        }

        DoneReason::Signal(_) => (),

        DoneReason::Success => {
            println!("Program completed normally.");
        }
    };
}

fn run(sdone: mpsc::Sender<DoneReason>, config: Rc<Config>) {
    let mut server = Server::new(config.clone()).unwrap();
    server.run();
    sdone.send(DoneReason::Success).unwrap();
}
