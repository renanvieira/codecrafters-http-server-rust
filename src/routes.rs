use std::fs;
use std::io::Empty;
use std::path::Path;

use crate::common::{StatusCode, EMPTY_CONTENT};
use crate::request::Request;
use crate::response::{Response, ResponseBuilder};

pub fn get_user_agent(request: &Request) -> Response {
    let content = request.headers.get("User-Agent");

    let response_builder = match content {
        Some(user_agent) => ResponseBuilder::new()
            .content(user_agent.to_owned())
            .status_code(StatusCode::OK),
        None => ResponseBuilder::new()
            .content(EMPTY_CONTENT.to_owned())
            .status_code(StatusCode::BadRequest),
    };

    response_builder.build()
}

pub fn get_echo(request: &Request) -> Response {
    let parsed_path: Vec<&str> = request.path.split("/echo").collect();

    let path_param = parsed_path[1];

    let content = match path_param {
        "" | "/" => EMPTY_CONTENT,
        _ => path_param.trim_start_matches('/'),
    };

    ResponseBuilder::new()
        .content(content.to_owned())
        .status_code(StatusCode::OK)
        .build()
}

pub fn get_index(request: &Request) -> Response {
    ResponseBuilder::new()
        .content(EMPTY_CONTENT.to_owned())
        .status_code(StatusCode::OK)
        .build()
}

pub fn get_file(request: &Request) -> Response {
    let directory = std::env::args().nth(2).expect("No Directory was given");
    let file = request
        .path
        .splitn(3, '/')
        .map(|s| s.to_owned())
        .nth(2)
        .expect("File is required");

    let full_path = format!("{}/{}", directory.trim_end_matches('/'), file);
    println!("Reading path: {}", full_path);

    let path: &Path = Path::new(&full_path);

    if path.exists() {
        let file_content: Vec<u8> = fs::read(path).expect("File should be readable");

        ResponseBuilder::new()
            .file(file_content)
            .status_code(StatusCode::OK)
            .build()
    } else {
        ResponseBuilder::new()
            .content(EMPTY_CONTENT.to_owned())
            .status_code(StatusCode::NotFound)
            .build()
    }
}
