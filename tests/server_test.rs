#[cfg(test)]
mod tests {
    use my_ping_server::server::Server;

    #[test]
    fn test_server_new() {
        let server = Server::new("127.0.0.1", 8080);
        assert_eq!(server.host, "127.0.0.1");
        assert_eq!(server.port, 8080);
    }
}
