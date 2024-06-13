use mio::{
    event::Source,
    net::{TcpStream, UnixStream},
    Registry,
};
use std::{
    io::{Read, Write},
    os::unix::io::{AsRawFd, RawFd},
};

pub enum Stream {
    Tcp(TcpStream),
    Unix(UnixStream),
}

impl AsRawFd for Stream {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.as_raw_fd(),
            Stream::Unix(unix_stream) => unix_stream.as_raw_fd(),
        }
    }
}

impl Source for Stream {
    fn register(
        &mut self,
        registry: &Registry,
        token: mio::Token,
        interests: mio::Interest,
    ) -> std::io::Result<()> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.register(registry, token, interests),
            Stream::Unix(unix_stream) => unix_stream.register(registry, token, interests),
        }
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: mio::Token,
        interests: mio::Interest,
    ) -> std::io::Result<()> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.reregister(registry, token, interests),
            Stream::Unix(unix_stream) => unix_stream.reregister(registry, token, interests),
        }
    }

    fn deregister(&mut self, registry: &Registry) -> std::io::Result<()> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.deregister(registry),
            Stream::Unix(unix_stream) => unix_stream.deregister(registry),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.write(buf),
            Stream::Unix(unix_stream) => unix_stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.flush(),
            Stream::Unix(unix_stream) => unix_stream.flush(),
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(tcp_stream) => tcp_stream.read(buf),
            Stream::Unix(unix_stream) => unix_stream.read(buf),
        }
    }
}
