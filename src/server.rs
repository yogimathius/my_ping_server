use chrono::{DateTime, Utc};
use std::net::IpAddr;
use std::time::Duration;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use tokio::time::sleep;
use trust_dns_resolver::TokioAsyncResolver;
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

    pub async fn run(&self) {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        println!("Listening on {}", addr);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream).await;
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    if let Some(Ok(request_line)) = buf_reader.lines().next() {
        if request_line.len() > "ping".len() && request_line.contains("ping") {
            let host_name = request_line.split_whitespace().last().unwrap().to_string();
            send_icmp_echo_requests(host_name).await;
            return;
        }
        match request_line.as_str() {
            "ping" => respond_with_pong(stream),
            _ => respond_with_not_found(stream),
        }
    } else {
        respond_with_not_found(stream);
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

async fn send_icmp_echo_requests(hostname: String) {
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    println!("Resolving {}...", hostname);
    let response = resolver.lookup_ip(hostname).await.unwrap();
    let ip = response.iter().next().unwrap();

    match ip {
        IpAddr::V4(ipv4) => {
            println!("Resolved IP: {:?}", ipv4);
            for seq in 0..5 {
                println!("Sent ICMP echo request to {}", ipv4);
                let payload = [0; 8];
                let (_, duration) = surge_ping::ping(ip, &payload).await.unwrap();
                println!(
                    "Received ICMP echo reply from {} seq={} time={:?}",
                    ipv4, seq, duration
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            for _ in segments {
                for seq in 0..5 {
                    let payload = [0; 8];
                    println!("Sent TCP echo request to {}", ip);

                    let (_packet, duration) = surge_ping::ping(ip, &payload).await.unwrap();
                    println!(
                        "Received TCP echo reply from {} seq={:?} time={:?}",
                        ipv6, seq, duration
                    );
                }
            }
        }
    }
}
