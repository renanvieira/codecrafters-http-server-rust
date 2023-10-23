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

    let headers = parse_headers(&http_request);
    let path_split: Vec<&str> = path.split('/').collect();
    let endpoint = path_split[1];

    let response_line = "HTTP/1.1 200 OK\r\n\r\n";

    match endpoint {
        "" => {
            stream.write_all(response_line.as_bytes())?;
        }
        "echo" => {
            stream.write_all(response_line.as_bytes())?;
            stream.write_all(path_split[2].as_bytes())?;
        }
        _ => (),
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
