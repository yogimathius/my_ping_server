use my_ping_server::server::Server;

fn main() {
    let server = Server::new("127.0.0.1", 42422);

    server.run();
}
