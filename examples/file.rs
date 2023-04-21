use std::collections::HashMap;

use blitzkrieg::{
    http::{BodyType, Request, Response},
    server::Server,
};

fn main() {
    let server = Server::new("127.0.0.1:4242", 4, Box::new(handler));
    server.start();
}

fn handler(req: Request) -> Response {
    let mut res = Response::new(200);
    let file = if let Some(BodyType::File(file)) = req.body {
        file
    } else {
        return res.body("Hello, world!");
    };
    std::fs::write(format!("file.{}", file.extension), &file.content.clone()).unwrap();
    res.body = Some(file.content);
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".into(),
        req.headers.get("Content-Type").unwrap().into(),
    );
    res.headers = Some(headers);
    res
}
