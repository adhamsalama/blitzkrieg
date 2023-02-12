use blitzkrieg::http::{Request, Response};
use blitzkrieg::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7818", Box::new(handler));
    server.start();
}

fn handler(request: Request) -> Response {
    Response::new(200).body("Hello, world!")
}
