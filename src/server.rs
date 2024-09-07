use std::net::TcpListener;

use crate::handlers::handle_connection;
pub struct Server {
    pub host: String,
    pub port: u16,
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
