use super::{BodyType, FormdataBody, FormdataFile, FormdataText, HTTPMethod, Request};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    str::FromStr,
};

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
        let default = "".to_string();
        let content_type = headers.get("Content-Type").unwrap_or(&default);
        if content_type.contains("multipart/form-data") {
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
            let body = std::str::from_utf8(body.as_slice()).unwrap().to_string();
            let body = match body.len() {
                0 => None,
                _ => Some(BodyType::Text(body)),
            };
            return Request {
                path: uri.to_string(),
                body,
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
