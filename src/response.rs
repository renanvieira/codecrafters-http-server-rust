use std::collections::HashMap;

use crate::common::{StatusCode, CRLF, EMPTY_CONTENT};


pub struct Response {
    pub content: String,
    pub content_type: String,
    pub content_length: usize,
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(
        content: String,
        content_length: usize,
        status_code: StatusCode,
        content_type: String,
        headers: Option<HashMap<String, String>>,
    ) -> Self {
        let _headers = match headers {
            Some(hm) => hm,
            None => HashMap::new(),
        };

        Self {
            content,
            content_length,
            status_code,
            headers: _headers,
            content_type,
        }
    }

    pub fn build_default_headers(&self) -> Result<HashMap<String, String>, anyhow::Error> {
        let mut default_headers = HashMap::new();

        default_headers.insert("Content-Type".to_owned(), self.content_type.to_owned());
        default_headers.insert("Content-Length".to_owned(), self.content_length.to_string());

        Ok(default_headers)
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }

    pub fn build_status_line(&self) -> String {
        format!("HTTP/1.1 {}", self.status_code)
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    pub content: String,
    pub content_type: String,
    pub status_code: Option<StatusCode>,
    pub headers: HashMap<String, String>,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            content: EMPTY_CONTENT.to_owned(),
            content_type: "text/plain".to_owned(),
            status_code: None,
            headers: HashMap::new(),
        }
    }

    pub fn content(mut self, content: String) -> ResponseBuilder {
        self.content = content;

        self
    }

    pub fn content_type(mut self, content_type: String) -> ResponseBuilder {
        self.content_type = content_type;

        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> ResponseBuilder {
        self.status_code = Some(status_code);

        self
    }

    pub fn add_header(mut self, header_name: String, value: String) -> ResponseBuilder {
        self.headers.insert(header_name, value);

        self
    }

    pub fn build(mut self) -> Response {
        let content_length_with_crlf = format!("{}{}", &self.content, CRLF);
        let content_length = content_length_with_crlf.as_bytes().len();

        let status_code = self
            .status_code
            .clone()
            .expect("Status Code must be present");
        let content_type = self.content_type.clone();

        self.headers
            .insert("Content-Length".to_owned(), content_length.to_string());
        self.headers.insert("Content-Type".to_owned(), content_type);

        Response::new(
            self.content,
            content_length,
            status_code,
            self.content_type,
            Some(self.headers),
        )
    }
}


pub fn not_found() -> Response {
    ResponseBuilder::new()
        .content(EMPTY_CONTENT.to_owned())
        .status_code(StatusCode::NotFound)
        .build()
}
