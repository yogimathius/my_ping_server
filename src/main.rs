use chrono::{DateTime, Utc};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:42422").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", request_line);
    match request_line.as_str() {
        "GET /ping HTTP/1.1" => respond_with_pong(stream),
        _ => respond_with_not_found(stream),
    }
}

fn respond_with_pong(mut stream: TcpStream) {
    let utc = Utc::now().to_rfc2822();
    let epoch = DateTime::parse_from_rfc2822(&utc).unwrap().timestamp();

    let response = format!("HTTP/1.1 200 OK\r\n\r\nPong {}", epoch);
    stream.write_all(response.as_bytes()).unwrap();
}

fn respond_with_not_found(mut stream: TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found</h1></body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}
