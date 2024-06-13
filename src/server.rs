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

use crate::config::Config;
use mio::{net, Events, Interest, Poll, Token};
use mio_serial::SerialPortBuilderExt;
use std::{
    io::{self, Read, Write},
    os::fd::AsRawFd,
    rc::Rc,
};
const SERIAL_TOKEN: Token = Token(0);
const TCP_TOKEN: Token = Token(1);

pub struct Server {
    config: Rc<Config>,
}

impl Server {
    pub fn new(config: Rc<Config>) -> io::Result<Self> {
        Ok(Server { config })
    }

    fn add_client(
        &self,
        poller: &Poll,
        clients: &mut Vec<net::TcpStream>,
        mut client: net::TcpStream,
    ) {
        let client_fd = client.as_raw_fd() as usize;
        poller
            .registry()
            .register(&mut client, Token(client_fd), Interest::READABLE)
            .unwrap();
        clients.push(client);
    }

    fn remove_client(&self, poller: &Poll, clients: &mut Vec<net::TcpStream>, client_fd: usize) {
        let client_index = clients
            .iter()
            .position(|c| c.as_raw_fd() as usize == client_fd)
            .unwrap();
        let mut client = clients.swap_remove(client_index);
        poller.registry().deregister(&mut client).unwrap();
    }

    pub fn run(&mut self) {
        let mut poller = Poll::new().expect("unable to create poller");
        let mut events = Events::with_capacity(500);
        let mut clients: Vec<net::TcpStream> = Vec::new();
        let mut serial_server = mio_serial::new(self.config.dev_path.clone(), self.config.baudrate)
            .stop_bits(mio_serial::StopBits::One)
            .parity(mio_serial::Parity::None)
            .data_bits(mio_serial::DataBits::Eight)
            .flow_control(mio_serial::FlowControl::None)
            .open_native_async()
            .expect("unable to open serial port");
        let mut tcp_server = net::TcpListener::bind(std::net::SocketAddr::new(
            self.config.get_ip().parse().unwrap(),
            self.config.port,
        ))
        .expect("unable to bind TCP listener");
        poller
            .registry()
            .register(&mut serial_server, SERIAL_TOKEN, Interest::READABLE)
            .unwrap();

        poller
            .registry()
            .register(&mut tcp_server, TCP_TOKEN, Interest::READABLE)
            .unwrap();

        loop {
            poller.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERIAL_TOKEN => {
                        let mut buffer = vec![0; 1024];
                        let bytes_read = serial_server.read(&mut buffer).unwrap();
                        if bytes_read == 0 {
                            continue;
                        }

                        for mut client in clients.iter() {
                            client.write_all(buffer.as_slice()).unwrap();
                        }
                    }
                    TCP_TOKEN => {
                        let (client, addr) = tcp_server.accept().unwrap();
                        println!("Accepted connection from: {}", addr);
                        self.add_client(&poller, &mut clients, client);
                    }
                    _ => {
                        let client_fd = event.token().0;
                        if event.is_error() || event.is_read_closed() || event.is_write_closed() {
                            self.remove_client(&poller, &mut clients, client_fd);
                        } else {
                            let client = clients
                                .iter_mut()
                                .find(|c| c.as_raw_fd() as usize == client_fd)
                                .unwrap();
                            let mut buffer = vec![0; 1024];
                            let bytes_read = client.read(&mut buffer).unwrap();
                            if bytes_read == 0 {
                                self.remove_client(&poller, &mut clients, client_fd);
                            } else {
                                serial_server.write_all(&buffer).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
