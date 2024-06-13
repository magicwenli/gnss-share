use core::fmt;
use mio::{event::Source, net};
#[cfg(target_family = "unix")]
use std::os::unix::io::AsRawFd;
#[cfg(not(target_family = "unix"))]
use std::os::windows::io::AsRawSocket as AsRawFd;
use std::{
    io::{Read, Result, Write},
    net::SocketAddr,
};

pub trait SocketPrint {
    fn peer_addr(&self) -> Result<SocketAddr>;
    fn local_addr(&self) -> Result<SocketAddr>;
}

pub trait Stream: AsRawFd + Source + Write + Read + SocketPrint {}

impl<T: AsRawFd + Source + Write + Read + SocketPrint> Stream for T {}

impl fmt::Display for Box<dyn Stream> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(target_family = "unix")]
        let fd = self.as_raw_fd();
        #[cfg(not(target_family = "unix"))]
        let fd = self.as_raw_socket();

        let peer_addr = match self.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => return write!(f, "fd: {}", fd),
        };
        let local_addr = match self.local_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => return write!(f, "fd: {}", fd),
        };
        write!(f, "remote:{} local:{} fd:{}", peer_addr, local_addr, fd)
    }
}

impl SocketPrint for net::TcpStream {
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.peer_addr()
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        self.local_addr()
    }
}

#[cfg(target_family = "unix")]
impl SocketPrint for net::UnixStream {
    fn peer_addr(&self) -> Result<SocketAddr> {
        Ok("unix".parse().unwrap())
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        Ok("unix".parse().unwrap())
    }
}
