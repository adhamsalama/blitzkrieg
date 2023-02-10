# Blitzkrieg

<img src="https://cdn-icons-png.flaticon.com/512/3857/3857541.png" style="width: 20%; height: 50%" alt="Blitzkrieg airplane logo">

An HTTP web server written from scratch in Rust (WIP).

## Documentation
https://docs.rs/blitzkrieg/latest/blitzkrieg/

## Installation
```cargo add blitzkrieg```

## Usage
### Hello world example
```rust
use blitzkrieg::http::{Request, Response};
use blitzkrieg::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7878", Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    Response {
        status_code: 200,
        headers: None,
        body: Some(String::from("Hello, world!").as_bytes().to_vec()),
    }
}
```

### Multipart/form-data
```rust
use blitzkrieg::http::{BodyType, Request, Response};
use blitzkrieg::server::Server;
use std::collections::HashMap;
use std::fs;
fn main() {
    let server = Server::new("127.0.0.1:7878", Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Authorization".into(), "Some token".to_string());
    match request.body {
        Some(body) => match body {
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
        },
        None => {}
    }
    Response {
        status_code: 200,
        headers: None,
        body: Some(String::from("Hello, world!").as_bytes().to_vec()),
    }
}

```