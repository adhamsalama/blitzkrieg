use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::{
    http::parse_http_string,
    http::parse_tcp_stream,
    http::BodyType,
    http::Request,
    http::{parse_formdata, Response},
    ThreadPool,
};

pub struct Server {
    pub threadpool: ThreadPool,
    pub listener: TcpListener,
    pub handler: Arc<Box<dyn Fn(Request) -> Response + Send + Sync + 'static>>,
}
impl Server {
    pub fn new(port: &str, handler: Box<dyn Fn(Request) -> Response + Send + Sync>) -> Server {
        let listener = TcpListener::bind(port).unwrap();
        let pool = ThreadPool::new(4);
        Server {
            threadpool: pool,
            listener,
            handler: Arc::new(handler),
        }
    }
    pub fn start(&self) {
        println!("Web Server is running...");
        for stream in self.listener.incoming() {
            let handler = Arc::clone(&self.handler);
            self.threadpool.execute(move || {
                let (mut stream, request) = build_http_request(stream.unwrap());
                let response = handler(request);
                let line = format!(
                    "HTTP/1.1 {}\r\n\r\n{}",
                    response.status_code,
                    response.body.unwrap_or_else(|| "".to_string())
                );
                stream.write_all(line.as_bytes()).unwrap();
            });
        }
    }
}

pub fn build_http_request(mut stream: TcpStream) -> (TcpStream, Request) {
    let (request, body) = parse_tcp_stream(&mut stream);
    let request = parse_http_string((request, body));
    return (stream, request);
}

pub fn print_http_request(request: Request) {
    println!("Request path {}", request.path);
    println!("Request headers {:?}", request.headers);
    println!(
        "Request content type {:?}",
        request.headers.get("Content-Type").unwrap()
    );
    match request.body.unwrap() {
        BodyType::FormdataBody(body) => {
            let formdatafields = body.fields;
            let formdatafiles = body.files;
            for field in formdatafields.unwrap_or_else(|| vec![]) {
                println!("Field name {}, value {}", field.name, field.value);
            }
            for field in formdatafiles.unwrap_or_else(|| vec![]) {
                println!("File name {}, content {}", field.name, field.content);
            }
        }
        BodyType::Text(text) => println!("Raw text {:?}", text), // BodyType::Text(text) => println!("Body is text {text}"),
    }
}
