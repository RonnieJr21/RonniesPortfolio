use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
    handle_connection(_stream);
    }
}

enum Body {
    Text(String),
    Binary(Vec<u8>),
}
struct Response {
    status: String,
    body: Body,
    length: usize,
    content_type: String,
}

fn handle_connection(mut stream: TcpStream) {
    let res = request_handler(&mut stream);

    let response = format!("{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n", res.status, res.length, res.content_type);
    stream.write_all(response.as_bytes()).unwrap();
    println!("Response: {}", response);
    match res.body {
        Body::Text(text) => stream.write_all(text.as_bytes()).unwrap(),
        Body::Binary(binary) => stream.write_all(&binary).unwrap(),    }
}

fn request_handler(stream: &TcpStream) -> Response {

    let reader = BufReader::new(stream);
    let request_line = match reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return Response {
            status: "HTTP/1.1 400 Bad Request".to_string(),
            body: Body::Text("Bad Request".to_string()),
            length: "Bad Request".len(),
            content_type: "text/plain".to_string(),
        },
    };    let res = match request_line.as_str() {

        "GET /styles.css HTTP/1.1" => {
            let body = fs::read_to_string("html/styles.css").unwrap_or_else(|_| "File not found".to_string());
            Response {
                status: "HTTP/1.1 200 OK".to_string(),
                length: body.len(),
                body: Body::Text(body),
                content_type: "text/css".to_string(),

            }
        }
        "GET /pexels-creative-vix-9754.jpeg HTTP/1.1" => {
            let path = "html/pexels-creative-vix-9754.jpeg";
            eprintln!("Serving file: {}", path); // Debug log
            let body = fs::read(path).unwrap_or_else(|_| Vec::new());
            Response {
                status: "HTTP/1.1 200 OK".to_string(),
                length: body.len(),
                body: Body::Binary(body),
                content_type: "image/jpeg".to_string(),
            }
        }
        "GET / HTTP/1.1" => {
            let body = fs::read_to_string("html/Hello.html").unwrap_or_else(|_| "File not found".to_string());
            Response {
                status: "HTTP/1.1 200 OK".to_string(),
                length: body.len(),
                body: Body::Text(body),
                content_type: "text/html".to_string(),
            }
        }
        "GET /Projects HTTP/1.1" => {
            let body = fs::read_to_string("html/Projects.html").unwrap_or_else(|_| "File not found".to_string());
            Response {
                status: "HTTP/1.1 200 OK".to_string(),
                length: body.len(),
                body: Body::Text(body),
                content_type: "text/html".to_string(),
            }
        }
        "GET /Experience HTTP/1.1" => {
            let body = fs::read_to_string("html/Experience.html").unwrap_or_else(|_| "File not found".to_string());
            Response {
                status: "HTTP/1.1 200 OK".to_string(),
                length: body.len(),
                body: Body::Text(body),
                content_type: "text/html".to_string(),
            }
        }
        _ => {
            let body = fs::read_to_string("html/404.html").unwrap_or_else(|_| "File not found".to_string());
            Response {
                status: "HTTP/1.1 404 NOT FOUND".to_string(),
                length: body.len(),
                body: Body::Text(body),
                content_type: "text/html".to_string(),
            }
        }
    };
    res
}