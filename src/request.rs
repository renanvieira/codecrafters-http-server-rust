use std::collections::HashMap;

use crate::router::HTTPMethod;

pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub endpoint: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub payload: Option<Vec<u8>>,
}

impl Request {
    pub fn new(
        method: HTTPMethod,
        path: String,
        endpoint: String,
        http_version: String,
        headers: HashMap<String, String>,
        payload: Option<Vec<u8>>,
    ) -> Self {
        Self {
            method,
            path,
            endpoint,
            http_version,
            headers,
            payload,
        }
    }
}
