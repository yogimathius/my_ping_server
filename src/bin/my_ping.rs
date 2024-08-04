use std::net::IpAddr;
use tokio::time::Duration;
use trust_dns_resolver::TokioAsyncResolver;

async fn send_icmp_echo_request(ip: IpAddr, seq: u16) {
    let payload = [0; 8];
    println!("Sent TCP echo request to {}", ip);

    let (_, duration) = surge_ping::ping(ip, &payload).await.unwrap();
    println!(
        "Received TCP echo reply from {} seq={:?} time={:?}",
        ip, seq, duration
    );
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    let hostname = std::env::args().nth(1).expect("Usage: my_ping <hostname>");
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
