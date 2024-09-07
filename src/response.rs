use crate::request::Request;
use chrono::{DateTime, Utc};

pub struct Response {
    pub status: String,
    pub body: String,
}

impl From<Request> for Response {
    fn from(request: Request) -> Self {
        match request.command.as_str() {
            "ping" => {
                let utc = Utc::now().to_rfc2822();
                let epoch = DateTime::parse_from_rfc2822(&utc).unwrap().timestamp();
                Response {
                    status: "200 OK".to_string(),
                    body: format!("Pong {}", epoch),
                }
            }
            _ => Response {
                status: "400 Bad Request".to_string(),
                body: "Bad Request".to_string(),
            },
        }
    }
}
