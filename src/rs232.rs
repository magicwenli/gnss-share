/* vim: set et ts=4 sw=4: */
/* rs232.rs
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

use crate::config::Config;
use crate::gps::GPS;
use serial;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

pub struct RS232 {
    reader: BufReader<serial::SystemPort>,
}

impl RS232 {
    pub fn new(config: Rc<Config>) -> io::Result<Self> {
        RS232::new_for_path(config.dev_path, &config)
    }

    fn new_for_path(path: &Path, config: &Config) -> io::Result<Self> {
        let mut port = serial::open(path.as_os_str())?;
        RS232::configure(&mut port as &mut dyn serial::SerialPort, config)?;

        Ok(RS232 {
            reader: BufReader::new(port),
        })
    }

    fn configure(port: &mut dyn serial::SerialPort, config: &Config) -> serial::Result<()> {
        let baudrate = config.get_baudrate();
        let settings = serial::PortSettings {
            baud_rate: baudrate,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };

        port.configure(&settings)?;

        port.set_timeout(Duration::from_millis(3_000))?;

        Ok(())
    }

    #[allow(dead_code)]
    fn verify(&mut self) -> bool {
        let mut buffer = String::new();

        for _ in 1..3 {
            if let Ok(_) = self.read_line(&mut buffer) {
                if buffer.len() >= 15
                    && buffer.chars().nth(0) == Some('$')
                    && buffer.chars().nth(6) == Some(',')
                {
                    return true;
                }

                buffer.clear();
            } else {
                println!("Failed to read from serial port");
            }
        }

        false
    }
}

impl GPS for RS232 {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
        self.reader.read_line(buffer)
    }
}
