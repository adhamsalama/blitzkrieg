# Blitzkrieg

<img src="https://cdn-icons-png.flaticon.com/512/3857/3857541.png" style="width: 20%; height: 50%" alt="Blitzkrieg airplane logo">

An HTTP web server written from scratch in Rust (WIP).

## Documentation

https://docs.rs/blitzkrieg/latest/blitzkrieg/

## Installation

`cargo add blitzkrieg`

## Usage

### [Hello world](examples/hello_world.rs)

```rust
use blitzkrieg::http::{Request, Response};
use blitzkrieg::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:3000", 4, Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    Response::new(200).body("Hello, world!")
}
```

### [File](examples/file.rs)

```rust
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
```
