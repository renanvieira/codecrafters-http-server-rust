use std::fs;
use std::io::Write;
use std::path::Path;

use std::fs::OpenOptions;

use crate::common::{StatusCode, EMPTY_CONTENT};
use crate::request::Request;
use crate::response::{Response, ResponseBuilder};

pub fn get_user_agent(request: &Request) -> Response {
    let content = request.headers.get("User-Agent");

    println!("Content: {:#?}", content);

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

    let content = match request.path.as_ref() {
        None => EMPTY_CONTENT,
        Some(p) => p.trim_start_matches('/'),
    };

    ResponseBuilder::new()
        .content(content.to_owned())
        .status_code(StatusCode::OK)
        .build()
}

pub fn get_index(_request: &Request) -> Response {
    ResponseBuilder::new()
        .content(EMPTY_CONTENT.to_owned())
        .status_code(StatusCode::OK)
        .build()
}

pub fn get_file(request: &Request) -> Response {
    let directory = std::env::args().nth(2).expect("No Directory was given");

    let full_path = format!(
        "{}{}",
        directory.trim_end_matches('/'),
        request.path.as_ref().unwrap()
    );
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

pub fn post_file(request: &Request) -> Response {
    let directory = std::env::args().nth(2).expect("No Directory was given");
    let file_path = &format!(
        "{}{}",
        directory.trim_end_matches('/'),
        request.path.as_ref().unwrap()
    );
    let full_path = Path::new(file_path);

    let mut open_options = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(full_path);

    let resp_status_code: StatusCode;

    match open_options.as_mut() {
        Ok(file) => {
            if let Some(body) = &request.payload {
                match file.write_all(body) {
                    Err(_) => panic!("Failed to read file {}", file_path),
                    _ => println!("Wrote to {:?}", full_path),
                }
            }

            resp_status_code = StatusCode::Created;
        }
        Err(e) => {
            println!("Error: {}", e);
            resp_status_code = StatusCode::BadRequest;
        }
    };

    ResponseBuilder::new().status_code(resp_status_code).build()
}
