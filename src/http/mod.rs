use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

mod errors;
mod methods;

use errors::*;
use methods::*;

#[derive(Clone)]
pub struct FormdataFile {
    pub name: String,
    pub file_name: String,
    pub content: Vec<u8>,
}
#[derive(Clone)]
pub struct FormdataText {
    pub name: String,
    pub value: String,
}

pub enum Formdata {
    FormdataText(FormdataText),
    FormdataFile(FormdataFile),
}

#[derive(Clone)]
pub struct FormdataBody {
    pub fields: Option<Vec<FormdataText>>,
    pub files: Option<Vec<FormdataFile>>,
}

#[derive(Clone)]
pub enum BodyType {
    Text(String),
    FormdataBody(FormdataBody),
}

#[derive(Clone)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<BodyType>,
}

impl Request {
    pub fn new(method: HTTPMethod, path: &str, headers: HashMap<String, String>) -> Self {
        Self {
            method,
            path: path.to_string(),
            headers,
            body: None,
        }
    }

    /// Set body request
    pub fn body(&mut self, body: Vec<u8>) -> &Self {
        self.body = Some(
            match self
                .headers
                .get("Content-Type")
                .unwrap_or(&"".to_string())
                .contains("multipart/form-data")
            {
                true => {
                    self.headers.insert(
                        "Content-Type".to_string(),
                        "multipart/form-data".to_string(),
                    );
                    let formdatabody = parse_formdata(&body);

                    BodyType::FormdataBody(formdatabody)
                }

                false => {
                    let chars = std::str::from_utf8(body.as_slice()).unwrap().to_string();
                    BodyType::Text(chars)
                }
            },
        );

        self
    }
}

impl FromStr for Request {
    type Err = RequestError;

    /// Parsing http request
    fn from_str(request: &str) -> Result<Self, Self::Err> {
        let request_lines: Vec<&str> = request.split("\r\n").collect();
        let mut first_line_iter = request_lines[0].split_whitespace();
        let method = first_line_iter.next().unwrap();
        let uri = first_line_iter.next().unwrap();
        let mut headers: HashMap<String, String> = HashMap::new();
        for header in request_lines.iter().skip(1) {
            if header.len() > 0 {
                match header.find(": ") {
                    Some(split_index) => {
                        headers.insert(
                            header[..split_index].to_string(),
                            header[split_index + 2..].to_string(),
                        );
                    }
                    None => return Err(RequestError::MissingChar(':')),
                }
            }
        }
        Ok(Self {
            path: uri.to_string(),
            body: None,
            method: HTTPMethod::from_str(method).unwrap(),
            headers,
        })
    }
}

pub struct Response {
    pub status_code: u16,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

pub fn parse_tcp_stream(stream: &mut TcpStream) -> (String, Vec<u8>) {
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
            let sizeplit = l.split(":");
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.                         // let body: String = buffer.lines().map(|result| result.unwrap()).collect();
    return (request, buffer);
}

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
            while !(data[i] == 13 && data[i + 1] == 10 && data[i + 2] == 45 && data[i + 3] == 45) {
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
    FormdataBody {
        fields: Some(form_fields),
        files: Some(form_files),
    }
}
