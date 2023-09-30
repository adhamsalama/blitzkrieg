#[cfg(test)]
pub mod tests {
    use std::io::{BufReader, Read, Write};

    // Define a mock object that implements MyTcpStream
    struct MockTcpStream {
        read_buffer: Vec<u8>,
        write_buffer: Vec<u8>,
    }

    impl MockTcpStream {
        fn new() -> Self {
            Self {
                read_buffer: Vec::new(),
                write_buffer: Vec::new(),
            }
        }

        fn set_read_buffer(&mut self, buffer: Vec<u8>) {
            self.read_buffer = buffer;
        }

        // fn get_write_buffer(&self) -> &[u8] {
        //     &self.write_buffer
        // }
    }

    impl Read for MockTcpStream {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let bytes_read = std::cmp::min(buf.len(), self.read_buffer.len());
            buf[..bytes_read].copy_from_slice(&self.read_buffer[..bytes_read]);
            self.read_buffer.drain(..bytes_read);
            Ok(bytes_read)
        }
    }

    impl Write for MockTcpStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.write_buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    use crate::http::{self, FormdataText, HTTPMethod, Request};
    // Test parsing HTTP Requests from strings
    #[test]
    fn parse_http_string_works() {
        let req = Request::parse("GET /get-request-test HTTP/1.1\r\nHost: localhost:7878\r\nAccept-Encoding: gzip, deflate, br\r\nAccept: */*\r\nConnection: keep-alive".into(),
            vec![],
        ).unwrap();
        assert_eq!(req.method, HTTPMethod::GET);
        assert_eq!(req.path, "/get-request-test");
        assert_eq!(req.headers.get("Host").unwrap(), "localhost:7878");
    }
    #[test]
    fn parse_http_string_plain_body_works() {
        let req = Request::parse("POST / HTTP/1.1\r\nHost: localhost:7878\r\nUser-Agent: insomnia/2022.7.3\r\nContent-Type: text/plain\r\nAuthorization: token\r\nAccept: */*\r\nContent-Length: 15\r\n\r\n".into(), vec![115, 111, 109, 101, 32, 112, 108, 97, 105, 110, 32, 98, 111, 100, 121]).unwrap();
        assert_eq!(req.method, HTTPMethod::POST);
        assert_eq!(req.headers.get("Content-Type").unwrap(), "text/plain");
        match req.body.unwrap() {
            http::BodyType::Text(body) => assert_eq!(body, "some plain body"),
            http::BodyType::FormdataBody(_) => panic!("Bodytype shouldn't be formdata"),
            http::BodyType::File(_) => panic!("Bodytype shouldn't be file"),
        }
    }
    #[test]
    fn parse_tcp_stream_works() {
        let mut s = MockTcpStream::new();
        let body_string = "some plain body";
        let body: Vec<u8> = Vec::from(body_string);
        let r = "POST / HTTP/1.1\r\nHost: localhost:7878\r\nUser-Agent: insomnia/2022.7.3\r\nContent-Type: text/plain\r\nAuthorization: token\r\nAccept: */*\r\nContent-Length: 15\r\n\r\n";
        let v = [r.into(), body].concat();
        s.set_read_buffer(v.into());
        let mut buffer = BufReader::new(&mut s);
        let req = Request::from_tcp_stream(&mut buffer).unwrap();
        match req.body {
            Some(http::BodyType::Text(body)) => {
                assert_eq!(body, body_string)
            }
            _ => panic!("Bodytype shouldn't be formdata"),
        }
    }
    #[test]
    fn tcp_pipelining_works() {
        let mut s = MockTcpStream::new();
        let mut all_reqs = String::new();
        let n = 5;
        for i in 0..n {
            let req = format!("POST /{i} HTTP/1.1\r\nContent-Length: 0\r\n\r\n");
            all_reqs = format!("{}{}", all_reqs, req);
        }
        s.set_read_buffer(all_reqs.into());
        let mut buffer = BufReader::new(&mut s);
        for i in 0..n {
            let req = Request::from_tcp_stream(&mut buffer).unwrap();
            assert_eq!(req.path, format!("/{}", i));
        }
    }
    #[test]
    fn test_empty_body() {
        let req = Request::parse("POST / HTTP/1.1\r\nHost: localhost:7878\r\nUser-Agent: insomnia/2022.7.3\r\nContent-Type: text/plain\r\nAuthorization: token\r\nAccept: */*\r\nContent-Length: 15\r\n\r\n".into(), vec![]).unwrap();
        match req.body {
            None => assert!(true),
            _ => panic!("Body should be None"),
        }
    }
    #[test]
    fn parse_http_string_formdata_body_works() {
        let req = Request::parse("POST / HTTP/1.1\r\nHost: localhost:7878\r\nUser-Agent: insomnia/2022.7.3\r\nContent-Type: multipart/form-data; boundary=X-INSOMNIA-BOUNDARY\r\nAuthorization: dqweqw\r\nAccept: */*\r\nContent-Length: 175\r\n\r\n".into(), vec![45, 45, 88, 45, 73, 78, 83, 79, 77, 78, 73, 65, 45, 66, 79, 85, 78, 68, 65, 82, 89, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 68, 105, 115, 112, 111, 115, 105, 116, 105, 111, 110, 58, 32, 102, 111, 114, 109, 45, 100, 97, 116, 97, 59, 32, 110, 97, 109, 101, 61, 34, 110, 97, 109, 101, 34, 13, 10, 13, 10, 97, 100, 104, 111, 109, 13, 10, 45, 45, 88, 45, 73, 78, 83, 79, 77, 78, 73, 65, 45, 66, 79, 85, 78, 68, 65, 82, 89, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 68, 105, 115, 112, 111, 115, 105, 116, 105, 111, 110, 58, 32, 102, 111, 114, 109, 45, 100, 97, 116, 97, 59, 32, 110, 97, 109, 101, 61, 34, 97, 103, 101, 34, 13, 10, 13, 10, 50, 51, 13, 10, 45, 45, 88, 45, 73, 78, 83, 79, 77, 78, 73, 65, 45, 66, 79, 85, 78, 68, 65, 82, 89, 45, 45, 13, 10]
    ).unwrap();
        assert_eq!(req.method, HTTPMethod::POST);
        assert_eq!(
            req.headers.get("Content-Type").unwrap(),
            "multipart/form-data"
        );
        match req.body.unwrap() {
            http::BodyType::Text(_) => panic!("Bodytype shouldn't be text/plain"),
            http::BodyType::File(_) => panic!("Bodytype shouldn't be file"),
            http::BodyType::FormdataBody(body) => {
                let mut expected_body: Vec<FormdataText> = Vec::new();
                expected_body.push(FormdataText {
                    name: "name".into(),
                    value: "adhom".into(),
                });
                expected_body.push(FormdataText {
                    name: "age".into(),
                    value: "23".into(),
                });
                assert_eq!(body.fields.unwrap(), expected_body);
                assert_eq!(body.files, None);
            }
        }
    }
}
