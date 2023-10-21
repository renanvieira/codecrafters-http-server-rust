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

    if path == "/" {
        stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    } else {
        stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
    }
    Ok(())
}
