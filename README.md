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
    let server = Server::new("127.0.0.1:3000", Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    Response::new(200).body("Hello, world!")
}
```

### [Multipart/form-data](examples/multi_part.rs)

```rust
use blitzkrieg::http::{BodyType, Request, Response};
use blitzkrieg::server::Server;
use std::collections::HashMap;
use std::fs;
fn main() {
    let server = Server::new("127.0.0.1:7818", Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Authorization".into(), "Some token".to_string());
    match request.body.unwrap() {
        BodyType::FormdataBody(formdata_body) => {
            println!("Request content-type is multipart/form-data");
            for field in formdata_body.fields.unwrap_or_default() {
                println!("Name {}, value {}", field.name, field.value);
            }
            for file in formdata_body.files.unwrap_or_default() {
                println!("Name {}, filename {}", file.name, file.file_name);
                // Save file to disk
                fs::write(file.file_name, file.content);
            }
        }
        BodyType::Text(text_body) => {
            println!("Request content-type is text");
            println!("{text_body}");
        }
    }
    Response::new(200).body("Hello, world!")
}
```
