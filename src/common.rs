use std::fmt::Display;

pub const CRLF: &str = "\r\n";
pub const EMPTY_CONTENT: &str = "";
pub const TCP_BUFFER_SIZE: usize = 4096;

#[derive(Clone)]
pub enum StatusCode {
    OK,
    NotFound,
    BadRequest,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::OK => write!(f, "200 OK"),
            StatusCode::NotFound => write!(f, "404 Not Found"),
            StatusCode::BadRequest => write!(f, "400 Bad Request"),
        }
    }
}


