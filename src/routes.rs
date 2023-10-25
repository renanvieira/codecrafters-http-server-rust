use crate::common::{EMPTY_CONTENT, StatusCode};
use crate::request::Request;
use crate::response::{ResponseBuilder, Response};



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



