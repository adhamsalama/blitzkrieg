use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

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
pub struct FormdataText {
    pub name: String,
    pub value: String,
}

pub struct FormdataFile {
    pub name: String,
    pub file_name: String,
    pub content: Vec<u8>,
}
pub enum Formdata {
    FormdataText(FormdataText),
    FormdataFile(FormdataFile),
}
pub struct FormdataBody {
    pub fields: Option<Vec<FormdataText>>,
    pub files: Option<Vec<FormdataFile>>,
}
pub enum BodyType {
    Text(String),
    FormdataBody(FormdataBody),
}
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    // queries: HashMap<String, String>,
    // cookies: HashMap<String, String>,
    pub body: Option<BodyType>,
}

pub struct Response {
    pub status_code: u16,
    pub headers: Option<HashMap<String, String>>,
    pub cookies: Option<HashMap<String, String>>,
    pub body: Option<String>,
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

pub fn parse_http_string((request, body): (String, Vec<u8>)) -> Request {
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
        let formdatabody = parse_formdata(&body);

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
