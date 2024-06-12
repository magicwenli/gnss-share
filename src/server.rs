use crate::config::Config;
use mio::{event, net, Events, Interest, Poll, Token};
use mio_serial::{SerialPortBuilderExt, SerialStream};
use std::{
    io::{self, Read, Write},
    os::fd::AsRawFd,
    rc::Rc,
};
const SERIAL_TOKEN: Token = Token(0);
const TCP_TOKEN: Token = Token(1);

pub struct Server {
    config: Rc<Config>,
    poller: Poll,
    events: Events,
    serial_server: SerialStream,
    tcp_server: net::TcpListener,
    clients: Vec<net::TcpStream>,
}

impl Server {
    pub fn new(config: Rc<Config>) -> io::Result<Self> {
        let poller = Poll::new()?;
        let mut events = Events::with_capacity(500);
        let mut serial_rx = mio_serial::new(&config.dev_path, config.baudrate)
            .stop_bits(mio_serial::StopBits::One)
            .parity(mio_serial::Parity::None)
            .data_bits(mio_serial::DataBits::Eight)
            .flow_control(mio_serial::FlowControl::None)
            .open_native_async()?;
        poller
            .registry()
            .register(&mut serial_rx, SERIAL_TOKEN, Interest::READABLE)
            .unwrap();

        let mut tcp_rx = net::TcpListener::bind(std::net::SocketAddr::new(
            config.get_ip().parse().unwrap(),
            config.port,
        ))
        .expect("unable to bind TCP listener");
        poller
            .registry()
            .register(&mut tcp_rx, TCP_TOKEN, Interest::READABLE)
            .unwrap();

        let clients = Vec::new();

        Ok(Server {
            config,
            poller,
            events,
            clients,
            serial_server: serial_rx,
            tcp_server: tcp_rx,
        })
    }

    fn add_client(
        self,
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

    fn remove_client(
        self,
        poller: &Poll,
        clients: &mut Vec<net::TcpStream>,
        mut client: net::TcpStream,
    ) {
        let client_fd = client.as_raw_fd() as usize;
        poller.registry().deregister(&mut client).unwrap();
        clients.retain(|c| c.as_raw_fd() as usize != client_fd);
        client.shutdown(std::net::Shutdown::Both).unwrap();
    }

    pub fn run(&mut self) {
        loop {
            self.poller.poll(&mut self.events, None).unwrap();

            for event in self.events.iter() {
                match event.token() {
                    SERIAL_TOKEN => {
                        let mut buffer = vec![0; 1024];
                        let bytes_read = self.serial_server.read(&mut buffer).unwrap();
                        if bytes_read == 0 {
                            continue;
                        }

                        for client in self.clients.into_iter() {
                            client.write_all(buffer.as_slice()).unwrap();
                        }
                    }
                    TCP_TOKEN => {
                        let (client, addr) = self.tcp_server.accept().unwrap();
                        println!("Accepted connection from: {}", addr);
                        self.add_client(&self.poller, &mut self.clients, client);
                    }
                    _ => {
                        let client = self
                            .clients
                            .into_iter()
                            .find(|c| c.as_raw_fd() as usize == event.token().0)
                            .unwrap();
                        if event.is_error() || event.is_read_closed() || event.is_write_closed() {
                            self.remove_client(&self.poller, &mut self.clients, client);
                        } else {
                            let mut buffer = vec![0; 1024];
                            let bytes_read = client.read(&mut buffer).unwrap();
                            if bytes_read == 0 {
                                self.remove_client(&self.poller, &mut self.clients, client);
                            } else {
                                let data = String::from_utf8(buffer).unwrap();
                                println!("Received data from client: {}", data);

                                self.serial_server.write_all(&buffer).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
