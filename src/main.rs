use blitzkrieg::ThreadPool;
mod http;
mod server;
use http::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7878", Box::new(user_fn));
    server.start();
}

fn user_fn(req: &Request) {
    println!("My Handler!!!");
}
