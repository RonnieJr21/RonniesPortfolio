use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7800").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}



fn handle_connection(mut stream: TcpStream){

    let buf_reader: BufReader<&TcpStream> = BufReader::new(& stream);
    let http_req : Vec<String> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let req_line:Vec<_> = http_req.get(0).unwrap().split(" ").collect();
    get_handler(req_line.get(0).unwrap(), req_line.get(1).unwrap(), stream);
}
//  format!("{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n", res.status, res.length, res.content_type)
fn get_handler(method:&str, route:&str, mut stream: TcpStream){
    match (method, route) {
        ("GET","/") => {Res::send_Hello(stream);}
        ("GET", "/Projects") =>{Res::send_Projects(stream);}
        ("GET", "/Experience") =>{Res::send_Experience(stream);}
        ("GET", "/styles.css") =>{Res::send_CSS(stream);}
        //("GET", "/Experience") =>{Res::send_Experience(stream);}
        (_,_)=>{ Res::send_404(stream);}
    }
}
fn post_handler(){}

struct Res {
    status:String,
    body:String,
    length:String,
    content_type: String,

}

impl Res {
    fn send_Hello(mut stream: TcpStream) {
        let file =fs::read_to_string("html/Hello.html").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

     fn send_Projects(mut stream: TcpStream) {
        let file =fs::read_to_string("html/Projects.html").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

     fn send_Experience(mut stream: TcpStream) {
        let file =fs::read_to_string("html/Experience.html").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

    fn send_CSS(mut stream: TcpStream) {
        let file =fs::read_to_string("html/styles.css").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/css\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

    fn send_BG(mut stream: TcpStream) {
        let file =fs::read_to_string("html/black-bg-png").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: image/png\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

     fn send_404(mut stream: TcpStream) {
        let file =fs::read_to_string("html/404.html").unwrap();
           let _ = stream.write_all(format!("HTTP/1.1 404 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", file.len().to_string(), file).as_bytes());
    }

}

