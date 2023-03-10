use blitzkrieg::http::{BodyType, Request, Response};
use blitzkrieg::server::Server;
use std::collections::HashMap;
use std::fs;
fn main() {
    let server = Server::new("127.0.0.1:3000", Box::new(handler));
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
