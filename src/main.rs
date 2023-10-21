use std::io::{prelude::*};
use std::io::{Error, Write, BufReader};
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

    let http_request : Vec<_> = buf_reader.lines().map(|x| x.unwrap()).take_while(|x|!x.is_empty()).collect();

    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;

    Ok(())
}
