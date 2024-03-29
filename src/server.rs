use crate::{http::Request, http::Response, threadpool::ThreadPool};
use std::{
    io::{prelude::*, BufReader},
    net::TcpListener,
    sync::Arc,
};

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
            self.threadpool.execute(move || loop {
                let mut cloned_stream = stream.try_clone().unwrap();
                let mut reader = BufReader::new(&mut cloned_stream);
                loop {
                    let request = Request::from_tcp_stream(&mut reader);
                    // stream
                    //     .set_read_timeout(Some(Duration::from_secs(5)))
                    //     .unwrap();
                    // stream
                    //     .set_write_timeout(Some(Duration::from_secs(5)))
                    //     .unwrap();
                    match request {
                        Ok(request) => {
                            println!("{} {}", request.method, request.path);
                            let response = handler(request);
                            match stream.write_all(&response.into_bytes()) {
                                Ok(_) => match stream.flush() {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!("Error in flushing response. {}", err);
                                    }
                                },
                                Err(err) => {
                                    println!("Error in writing response. {}", err);
                                }
                            }
                        }
                        Err(error) => {
                            if error == "Finished".to_string() {
                                break;
                            }
                            println!("Error in request. {error}");
                            if error == "Resource temporarily unavailable (os error 11)".to_string()
                                || error == "Connection reset by peer (os error 104)".to_string()
                            {
                                break;
                            }
                            let error_response = Response::new(500).body(&error);
                            match stream.write_all(&error_response.into_bytes()) {
                                Ok(_) => {
                                    break;
                                }
                                Err(err) => {
                                    println!("Error in sending generic response. {}", err);
                                    break;
                                }
                            }
                        }
                    }
                }
                break;
            });
        }
    }
}
