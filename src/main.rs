use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Stablished!");
        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect();

    let status_line: Vec<&str> = http_request[0].split(' ').collect();

    let method = status_line[0];
    let path = status_line[1];
    let http_version = status_line[2];

    println!("Request: {} {}", method, path);
    let request_headers = parse_headers(&http_request);
    let path_split: Vec<String> = path.splitn(3,'/').map(|s|s.to_string()).collect();

    let endpoint = path_split[1].to_owned();
    let empty_content = String::new();
    let content = path_split.get(2).unwrap_or(&empty_content);
    let content_bytearray = content.as_bytes();
    let mut response_headers: HashMap<&str, String> = HashMap::new();

    response_headers.insert("Content-Type", "text/plain".to_owned());
    response_headers.insert("Content-Length",content_bytearray.len().to_string());
    

    let ok_response_line = "HTTP/1.1 200 OK\r\n";
    let not_found_response_line = "HTTP/1.1 404 NotFound\r\n";
    let headers_line: Vec<String> = response_headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect();

    match endpoint.as_ref() {
        "" => {
            println!("Route Matched '/': {} - {:#?}", endpoint, path_split);
            stream.write_all(ok_response_line.as_bytes())?;
            stream.write_all("\r\n\r\n".as_bytes())?;
        }
        "echo" => {
            println!("Route Matched '/echo': {} - {:#?} - ", endpoint, path_split);
            stream.write_all(ok_response_line.as_bytes())?;
            stream.write_all(headers_line.join("\r\n").as_bytes())?;
            stream.write_all("\r\n\r\n".as_bytes())?;
            stream.write_all(content_bytearray)?;
        }
        _ => {
            println!("Not Matching Found {} - {:#?}", endpoint, path_split);

            stream.write_all(not_found_response_line.as_bytes())?;
            stream.write_all("\r\n".as_bytes())?;
        }
    };

    Ok(())
}

fn parse_headers(http_request: &Vec<String>) -> HashMap<String, String> {
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
