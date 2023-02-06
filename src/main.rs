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
    Response {
        status_code: 404,
        headers: None,
        cookies: None,
        body: Some("Some String...".to_string()),
    }
}
