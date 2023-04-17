use crate::{http::Request, http::Response, threadpool::ThreadPool};
use std::{io::prelude::*, net::TcpListener, sync::Arc};

/// HTTP Server struct.
pub struct Server {
    /// The server's internal threadpool.
    threadpool: ThreadPool,
    /// The server's TCP listener.
    listener: TcpListener,
    /// The function that handles HTTP requests.
    handler: Arc<Box<dyn Fn(Request) -> Response + Send + Sync + 'static>>,
}

impl Server {
    /// Creates a new HTTP Server.
    pub fn new(
        port: &str,
        threads: usize,
        handler: Box<dyn Fn(Request) -> Response + Send + Sync>,
    ) -> Server {
        let listener = TcpListener::bind(port).unwrap();
        let pool = ThreadPool::new(threads);
        Server {
            threadpool: pool,
            listener,
            handler: Arc::new(handler),
        }
    }

    /// Starts the HTTP server.
    /// It will run forever.
    pub fn start(&self) {
        println!(
            "Blitzkrieg Web Server is running on {}",
            self.listener.local_addr().unwrap()
        );
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let handler = Arc::clone(&self.handler);
            self.threadpool.execute(move || {
                let request = Request::from_tcp_stream(&mut stream);
                let response = handler(request);
                let response_bytes = response.into_bytes();
                stream.write_all(&response_bytes).unwrap();
            });
        }
    }
}
