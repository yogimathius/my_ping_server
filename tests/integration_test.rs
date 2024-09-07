use my_ping_server::server::Server;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::test]
async fn test_server_run() {
    let server = Server::new("127.0.0.1", 8080);

    // Start the server in a background task
    tokio::spawn(async move {
        server.run().await;
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    stream.write_all(b"ping").await.unwrap();

    let mut buffer = [0; 4];
    let n = stream.read(&mut buffer).await.unwrap();

    // Check the response
    assert_eq!(&buffer[..n], b"pong");
}
