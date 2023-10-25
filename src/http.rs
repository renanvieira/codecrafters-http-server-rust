use std::collections::HashMap;

use crate::request::Request;



pub fn parse_path(request: &Request) -> String {
    let path_split: Vec<String> = request.path.splitn(3, '/').map(|s| s.to_string()).collect();

    match path_split[1].as_ref() {
        "" => "/".to_string(),
        _ => path_split[1].to_string(),
    }
}

pub fn parse_request(raw_request: Vec<String>) -> Request {
    let status_line: Vec<&str> = raw_request[0].split(' ').collect();

    let method = status_line[0];
    let path = status_line[1];
    let http_version = status_line[2];

    let request_headers = parse_headers(&raw_request);

    Request::new(
        method.to_owned(),
        path.to_owned(),
        http_version.to_owned(),
        request_headers,
    )
}

pub fn parse_headers(http_request: &Vec<String>) -> HashMap<String, String> {
    let splited_headers: Vec<Vec<String>> = http_request
        .iter()
        .skip(1)
        .map(|h| h.split(' ').collect())
        .map(|h: String| h.splitn(2, ':').map(|s| s.to_string()).collect())
        .collect();

    splited_headers
        .iter()
        .map(|x| (x.get(0), x.get(1)))
        .filter(|(x, y)| x.is_some() && y.is_some())
        .map(|(x, y)| (x.unwrap(), y.unwrap()))
        .map(|(x, y)| (x.to_owned(), y.to_owned()))
        .collect()
}
