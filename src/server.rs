use crate::{
    http::parse_http_string, http::parse_tcp_stream, http::Request, http::Response,
    threadpool::ThreadPool,
};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
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
                let response_bytes = build_http_response_string(response);
                stream.write_all(&response_bytes).unwrap();
            });
        }
    }
}

pub fn build_http_request(mut stream: TcpStream) -> (TcpStream, Request) {
    let (request, body) = parse_tcp_stream(&mut stream);
    let request = parse_http_string((request, body));
    return (stream, request);
}

pub fn build_http_response_string(response: Response) -> Vec<u8> {
    let mut res = String::from("HTTP/1.1 ");
    let status_code = response.status_code.to_string();
    res = format!("{}{}", res, status_code);
    res.push_str("\r\n");
    for (key, value) in response.headers.unwrap_or_default() {
        res = format!("{}{}: {}\r\n", res, key, value);
    }
    res.push_str("\r\n");
    let mut res = res.as_bytes().to_owned();
    res.append(&mut response.body.unwrap_or_default());
    res
}
