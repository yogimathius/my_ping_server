use bytes::Bytes;

pub(crate) struct Request {
    pub command: String,
    pub hostname: Option<String>,
}

impl From<Bytes> for Request {
    fn from(bytes: Bytes) -> Self {
        // Convert Bytes to String
        let request_line = String::from_utf8(bytes.to_vec()).unwrap();

        // Split the request line into parts
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        // Extract the command and hostname
        let command = parts[0].to_string();
        let hostname = if parts.len() > 1 {
            Some(parts[1].to_string())
        } else {
            None
        };

        // Return the Request struct
        Request { command, hostname }
    }
}
