use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

use crate::http::parse_request;
use crate::response::not_found;
use crate::router::{get, post, Router};
use crate::routes::{get_echo, get_file, get_index, get_user_agent, post_file};

use self::common::{CRLF, TCP_BUFFER_SIZE};
use self::response::Response;

pub mod common;
pub mod http;
pub mod request;
pub mod response;
pub mod router;
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

async fn read_data_from_socket(
    reader: &mut BufStream<TcpStream>,
) -> Result<(Vec<String>, Vec<u8>), anyhow::Error> {
    let mut body: Vec<u8> = Vec::new();
    let mut http_headers: Vec<String> = Vec::new();

    let mut content: Vec<u8> = Vec::new();
    let mut buf: [u8; TCP_BUFFER_SIZE] = [0; TCP_BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buf).await?;

        content.extend(&buf[..bytes_read]);

        if bytes_read < TCP_BUFFER_SIZE {
            break;
        }
    }

    let delimiter = b"\r\n\r\n";

    if let Some(index) = content
        .windows(delimiter.len())
        .position(|w| w == delimiter)
    {
        let header_end = index + delimiter.len();

        let decoded_headers: Vec<String> = String::from_utf8_lossy(&content[..header_end])
            .to_owned()
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();

        http_headers.extend(decoded_headers);
        body.extend(&content[header_end..]);
    }

    Ok((http_headers, body))
}

async fn handle_connection(stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut buf = BufStream::new(stream);
    let (headers, body) = read_data_from_socket(&mut buf).await?;

    let request = parse_request(headers, body);

    println!("Request: {} {}", request.method, request.path.as_ref().unwrap_or(&String::new()));

    let router = Router::new()
        .route(get("/", get_index))
        .route(get("/files", get_file))
        .route(post("/files", post_file))
        .route(get("/echo", get_echo))
        .route(get("/user_agent", get_user_agent));

    let response: Response;

    if let Some(route_fn) = router.find(&request) {
        response = route_fn(&request);
    } else {
        response = not_found();
    }

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

    let mut byte_buf = buf.as_bytes().to_owned();
    byte_buf.extend(response.content.clone());

    Ok(byte_buf.to_owned())
}
