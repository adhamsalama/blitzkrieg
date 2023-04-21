use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}
impl FromStr for HTTPMethod {
    type Err = ();
    /// Creates an HTTP method enum value from a str.
    fn from_str(input: &str) -> Result<HTTPMethod, Self::Err> {
        match input {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "DELETE" => Ok(HTTPMethod::DELETE),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FormdataText {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct FormdataFile {
    pub name: String,
    pub file_name: String,
    pub content: Vec<u8>,
}
pub enum Formdata {
    FormdataText(FormdataText),
    FormdataFile(FormdataFile),
}

#[derive(Debug, PartialEq)]
/// Formdata body struct.
pub struct FormdataBody {
    /// Formdata text fields.
    pub fields: Option<Vec<FormdataText>>,
    /// Formdata files.
    pub files: Option<Vec<FormdataFile>>,
}

#[derive(Debug)]
/// HTTP Request body type.
pub enum BodyType {
    Text(String),
    FormdataBody(FormdataBody),
}
#[derive(Debug)]
/// HTTP Request struct.
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<BodyType>,
}

impl Request {
    /// Constructs an HTTP Request from a TCP Stream.
    pub fn from_tcp_stream<T: Read + Write>(stream: &mut T) -> Request {
        let mut reader = BufReader::new(stream);
        let mut request = String::new();
        loop {
            let r = reader.read_line(&mut request).unwrap();
            if r < 3 {
                //detect empty line
                break;
            }
        }
        let mut size = 0;
        let linesplit = request.split("\n");
        for l in linesplit {
            if l.starts_with("Content-Length") {
                size = l
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
            }
        }
        let mut buffer = vec![0; size]; //New Vector with size of Content
        reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.
        let request = Request::parse(request, buffer);
        request
    }
    /// Parses an HTTP Request from a String and its body from a vector of bytes.
    pub fn parse(request: String, body: Vec<u8>) -> Request {
        let request_lines: Vec<&str> = request.split("\r\n").collect();
        let mut first_line_iter = request_lines[0].split_whitespace();
        let method = first_line_iter.next().unwrap();
        let uri = first_line_iter.next().unwrap();
        let mut headers: HashMap<String, String> = HashMap::new();
        for header in request_lines.iter().skip(1) {
            if header.len() > 0 {
                let split_index = header.find(": ").expect("Header doesn't have a ': '");
                headers.insert(
                    header[..split_index].to_string(),
                    header[split_index + 2..].to_string(),
                );
            }
        }
        if headers
            .get("Content-Type")
            .unwrap_or(&"".to_string())
            .contains("multipart/form-data")
        {
            headers.insert(
                "Content-Type".to_string(),
                "multipart/form-data".to_string(),
            );
            let formdatabody = Request::parse_formdata(&body);

            return Request {
                path: uri.to_string(),
                body: Some(BodyType::FormdataBody(formdatabody)),
                method: HTTPMethod::from_str(method).unwrap(),
                headers,
            };
        } else {
            let chars = std::str::from_utf8(body.as_slice()).unwrap().to_string();
            return Request {
                path: uri.to_string(),
                body: Some(BodyType::Text(chars)),
                method: HTTPMethod::from_str(method).unwrap(),
                headers,
            };
        }
    }
    /// Parses and returns a Formdata body.
    pub fn parse_formdata(data: &Vec<u8>) -> FormdataBody {
        // Get separator value
        let n = data.len();
        let mut i = 0;
        let mut form_files: Vec<FormdataFile> = vec![];
        let mut form_fields: Vec<FormdataText> = vec![];
        while i < n - 1 {
            let mut line = String::new();
            while !(data[i] == 13 && data[i + 1] == 10) {
                line.push(data[i] as char);
                i += 1;
            }
            i += 4;
            if line == "" || line.starts_with("-") {
                i += 1;
                continue;
            }
            if line.contains("form-data") && line.contains("filename") {
                let splitted = line.replace("\"", "");
                let splitted: Vec<&str> = splitted.split("; ").collect();
                let name = &splitted[1][5..];

                let filename = &splitted[2][9..];
                // Start parsing file
                // Ignore \r\n
                i += 4;
                // Ignore content-type
                while !(data[i] == 13 && data[i + 1] == 10) {
                    i += 1;
                }
                // Ignore \r\n
                i += 4;
                let mut file: Vec<u8> = vec![];
                while !(data[i] == 13
                    && data[i + 1] == 10
                    && data[i + 2] == 45
                    && data[i + 3] == 45)
                {
                    file.push(data[i]);
                    i += 1;
                }

                form_files.push(FormdataFile {
                    name: name.into(),
                    file_name: filename.into(),
                    content: file,
                })
            } else if line.contains("form-data") {
                let mut value = String::new();
                // i += 4;
                while !(data[i] == 13 && data[i + 1] == 10) {
                    value.push(data[i] as char);
                    i += 1;
                }
                let splitted = line.replace("\"", "");
                let splitted: Vec<&str> = splitted.split("; ").collect();
                let name = &splitted[1][5..];
                form_fields.push(FormdataText {
                    name: name.into(),
                    value: value.into(),
                })
            } else {
                // Shouldn't reach here if line doesn't start with "--"
                if !line.contains("--") {
                    panic!("Error in parsing form data");
                }
            }
            i += 1;
        }
        let form_fields = match form_fields.len() {
            0 => None,
            _ => Some(form_fields),
        };
        let form_files = match form_files.len() {
            0 => None,
            _ => Some(form_files),
        };
        FormdataBody {
            fields: form_fields,
            files: form_files,
        }
    }
}

/// HTTP Response struct.
pub struct Response {
    pub status_code: u16,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    /// Creates an HTTP Response.
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            headers: None,
            body: None,
        }
    }

    /// Set reponse headers.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set reponse body.
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.as_bytes().to_vec());
        self
    }

    /// Turns an HTTP Response into bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut res = String::from("HTTP/1.1 ");
        let status_code = self.status_code.to_string();
        res = format!("{}{}", res, status_code);
        res.push_str("\r\n");
        let q = self.headers.unwrap_or_default();
        for (key, value) in q {
            res = format!("{}{}: {}\r\n", res, key, value);
        }
        res.push_str("Server: Blitzkrieg\r\n");
        res.push_str("\r\n");
        let mut res = res.as_bytes().to_owned();
        res.append(&mut self.body.unwrap());
        res
    }
}
