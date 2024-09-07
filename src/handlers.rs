use bytes::Bytes;
use std::io::Read;
use std::net::IpAddr;
use std::time::Duration;
use std::{io::Write, net::TcpStream};
use tokio::time::sleep;
use trust_dns_resolver::TokioAsyncResolver;

use crate::connection::Connection;
use crate::request::Request;
use crate::response::Response;

pub async fn handle_connection(mut stream: TcpStream) {
    let mut connection = Connection::new(stream);

    let mut buf = vec![0; 1024];
    let n = connection.read(&mut buf).unwrap();
    let bytes = Bytes::copy_from_slice(&buf[..n]);
    let request: Request = bytes.into();
    if let Some(hostname) = request.hostname {
        send_icmp_echo_requests(hostname).await;
    } else {
        let response: Response = request.into();
        respond_with_pong(&mut connection, response);
    }
}

fn respond_with_pong(connection: &mut Connection, response: Response) {
    let full_response = format!("{}\n{}", response.status, response.body);
    if let Err(e) = connection.write(full_response.as_bytes()) {
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
