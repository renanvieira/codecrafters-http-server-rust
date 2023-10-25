use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, BufStream, ReadHalf};
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::http::{parse_path, parse_request};
use crate::response::not_found;
use crate::routes::{get_echo, get_index, get_user_agent};

use self::common::CRLF;
use self::response::Response;

pub mod common;
pub mod http;
pub mod request;
pub mod response;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Connection Established");

        tokio::spawn(async move {
            let conn_res = handle_connection(socket).await;

            match conn_res {
                Err(e) => {
                    eprintln!("Failure: {:?}", e);
                    return;
                }
                _ => {
                    println!("Connection Closed.")
                }
            }
        });
    }
}

async fn read_request_from_socket(
    reader: &mut BufStream<TcpStream>,
) -> Result<Vec<String>, anyhow::Error> {
    let mut http_request: Vec<String> = Vec::new();

    let mut http_request_lines: tokio::io::Lines<_> = reader.lines();

    loop {
        match http_request_lines.next_line().await? {
            Some(ref empty_line) if empty_line.is_empty() => break,
            Some(line) => {
                http_request.push(line)
            }
            None => break,
        }
    }

    Ok(http_request)
}

async fn handle_connection(stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut buf = BufStream::new(stream);
    let http_request = read_request_from_socket(&mut buf).await?;

    let request = parse_request(http_request);

    println!("Request: {} {}", request.method, request.path);
    let endpoint = parse_path(&request);

    let response = match endpoint.as_ref() {
        "/" => {
            println!("Route Matched '/': {} - {:#?}", endpoint, request.path);
            get_index(&request)
        }
        "echo" => {
            println!(
                "Route Matched '/echo': {} - {:#?} - ",
                endpoint, request.path
            );

            get_echo(&request)
        }
        "user-agent" => {
            println!(
                "Route Matched '/user-agent': {} - {:#?} - {:#?}",
                endpoint, request.path, request.headers,
            );

            get_user_agent(&request)
        }
        _ => {
            println!("Not Matching Found {} - {:#?}", endpoint, request.path);
            not_found()
        }
    };

    let response_bytes = build_response(&response)?;

    write_response(&mut buf, &response_bytes).await?;

    Ok(())
}

async fn write_response(
    buf: &mut BufStream<TcpStream>,
    response_bytes: &[u8],
) -> Result<(), anyhow::Error> {
    buf.write_all(&response_bytes).await?;
    buf.flush().await?;

    Ok(())
}

fn build_response(response: &Response) -> Result<Vec<u8>, anyhow::Error> {
    let status_line = format!("{}{}", response.build_status_line(), CRLF);
    let headers: String = response
        .headers
        .iter()
        .map(|(k, v)| format!("{}: {}{}", k, v, CRLF))
        .collect();

    let mut buf: String = String::new();

    buf.push_str(&status_line);
    buf.push_str(&headers);
    buf.push_str(&CRLF);
    buf.push_str(&response.content);

    Ok(buf.as_bytes().to_owned())
}
