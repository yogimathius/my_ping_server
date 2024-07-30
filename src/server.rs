use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use chrono::{DateTime, Utc};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn run(&self) {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        println!("Listening on {}", addr);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    match request_line.as_str() {
        "ping" => respond_with_pong(stream),
        _ => respond_with_not_found(stream),
    }
}

fn respond_with_pong(mut stream: TcpStream) {
    let utc = Utc::now().to_rfc2822();
    let epoch = DateTime::parse_from_rfc2822(&utc).unwrap().timestamp();
    let response = format!("Pong {}", epoch);
    stream.write_all(response.as_bytes()).unwrap();
}

fn respond_with_not_found(mut stream: TcpStream) {
    let response = "400 Bad Request\n";
    stream.write_all(response.as_bytes()).unwrap();
}
