use std::collections::HashMap;
use std::str::FromStr;

use crate::request::Request;
use crate::router::HTTPMethod;

pub fn parse_request(headers: Vec<String>, body: Vec<u8>) -> Request {
    let status_line: Vec<&str> = headers[0].split(' ').collect();

    let method = status_line[0];
    let req_path = status_line[1];
    let http_version = status_line[2];

    let split_index = req_path.match_indices("/").map(|(idx, _)| idx).nth(1);
    let path: Option<String>;
    let endpoint: String;

    if let Some(idx) = split_index {
        let split: (_, _) = req_path.split_at(idx);

        endpoint = split.0.to_string();
        path = Some(split.1.to_string());
    } else {
        endpoint = req_path.to_string();
        path = None;
    }

    let request_headers = parse_headers(&headers);
    Request::new(
        HTTPMethod::from_str(&method).unwrap(),
        path,
        endpoint.to_owned(),
        http_version.to_owned(),
        request_headers,
        Some(body),
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
