use blitzkrieg::ThreadPool;
mod http;
mod server;
use server::Server;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let server = Server {
        threadpool: pool,
        job: None,
        listener,
    };
    server.start();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     pool.execute(|| {
    //         let (mut stream, request) = handle_connection(stream);
    //         print_http_request(request);
    //         let response = "HTTP/1.1 200 OK\r\n\r\n".to_owned();
    //         stream.write_all(response.as_bytes()).unwrap();
    //     });
    // }
}
