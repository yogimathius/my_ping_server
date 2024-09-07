use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.stream.write_all(buf)
    }
}
