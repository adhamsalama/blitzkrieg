use std::{collections::HashMap, fs};

use blitzkrieg::ThreadPool;
mod http;
mod server;
use http::{Request, Response};
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7878", Box::new(user_fn));
    server.start();
}

fn user_fn(req: Request) -> Response {
    println!("My Handler!!!");
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Authorization".into(), "asd".into());
    headers.insert("other-header".into(), "dsa".into());
    match req.body {
        Some(body) => match body {
            http::BodyType::FormdataBody(b) => {
                println!("GOT FORMDATA");
                println!("printing fields");
                for field in b.fields.unwrap_or_default() {
                    println!("Name {}, value {}", field.name, field.value);
                }
                for file in b.files.unwrap_or_default() {
                    println!("Name {}, filename {}", file.name, file.file_name);
                    fs::write(file.file_name, file.content);
                }
            }
            http::BodyType::Text(b) => {}
        },
        None => {}
    }
    Response {
        status_code: 200,
        headers: Some(headers),
        cookies: None,
        body: Some("Some String...".to_string()),
    }
}
