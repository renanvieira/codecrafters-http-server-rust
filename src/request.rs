use std::collections::HashMap;


pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(
        method: String,
        path: String,
        http_version: String,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            path,
            http_version,
            headers,
        }
    }
}
