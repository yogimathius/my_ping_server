use chrono::{DateTime, Utc};
use std::net::IpAddr;
use std::time::Duration;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};
use tokio::time::sleep;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn handle_connection(mut stream: TcpStream) {
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
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
}

fn respond_with_not_found(mut stream: TcpStream) {
    let response = "400 Bad Request\n";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
}

async fn send_icmp_echo_requests(hostname: String) {
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let response = resolver.lookup_ip(hostname).await.unwrap();
    let ip = response.iter().next().unwrap();

    match ip {
        IpAddr::V4(ipv4) => {
            for seq in 0..5 {
                send_icmp_echo_request(std::net::IpAddr::V4(ipv4), seq).await;
            }
        }
        IpAddr::V6(ipv6) => {
            for seq in 0..5 {
                send_icmp_echo_request(std::net::IpAddr::V6(ipv6), seq).await;
            }
        }
    }
}

async fn send_icmp_echo_request(ip: IpAddr, seq: u16) {
    let payload = [0; 8];
    println!("Sent TCP echo request to {}", ip);

    let (_, duration) = surge_ping::ping(ip, &payload).await.unwrap();
    println!(
        "Received TCP echo reply from {} seq={:?} time={:?}",
        ip, seq, duration
    );
    sleep(Duration::from_secs(1)).await;
}
