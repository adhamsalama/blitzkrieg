use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

enum HTTPMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

struct Request {
    method: HTTPMethods,
    path: String,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    cookies: HashMap<String, String>,
}

struct Response {
    status_code: u8,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
}

fn parser(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    /*
        Request = [
        "GET / HTTP/1.1",
        "Host: localhost:7878",
        "Accept-Encoding: gzip, deflate, br",
        "Accept: ",
        "Connection: keep-alive",
        "User-Agent: HTTPie/3.2.1",
        ]
    */
    let first_line: Vec<&str> = http_request[0].split(" ").collect();
    let method = first_line[0].clone();
    let path = first_line[1].clone();
}
