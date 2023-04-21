use blitzkrieg::http::{Request, Response};
use blitzkrieg::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:3000", 4, Box::new(handler));
    server.start();
}

fn handler(_req: Request) -> Response {
    Response::new(200).body("Hello, world!")
}
