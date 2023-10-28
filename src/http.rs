use std::collections::HashMap;
use std::str::FromStr;

use crate::request::Request;
use crate::router::HTTPMethod;

pub fn parse_request(headers: Vec<String>, body: Vec<u8>) -> Request {
    let status_line: Vec<&str> = headers[0].split(' ').collect();

    let method = status_line[0];
    let req_path = status_line[1];
    let http_version = status_line[2];

    let endpoint: &str;
    let path: Option<&str>;

    let (endpoint, path) = req_path
        .match_indices("/")
        .nth(1)
        .map(|(index, _)| {
            let (e, p) = req_path.split_at(index);
            (e, Some(p))
        })
        .unwrap_or_else(|| ("/", None));

    let request_headers = parse_headers(&headers);

    println!("{:#?}", headers);

    Request::new(
        HTTPMethod::from_str(&method).unwrap(),
        path.map(str::to_string),
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
