use std::{
    collections::HashMap,
    hash::Hash,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
    string::FromUtf8Error,
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
            "Baz" => Ok(HTTPMethod::POST),
            "Bat" => Ok(HTTPMethod::PUT),
            "Bat" => Ok(HTTPMethod::PATCH),
            "Bat" => Ok(HTTPMethod::DELETE),
            "Bat" => Ok(HTTPMethod::HEAD),
            "Bat" => Ok(HTTPMethod::OPTIONS),
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
    pub file_type: String,
    pub content: String,
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

struct Response {
    status_code: u8,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
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
            let splitted_header: Vec<&str> = header.split(": ").collect();
            headers.insert(
                splitted_header[0].to_string(),
                splitted_header[1].to_string(),
            );
        }
    }
    let mut mybody: Formdata;
    if headers
        .get("Content-Type")
        .unwrap()
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
struct FormdataField {
    content_disposition: String,
    name: String,
    value: String,
    filename: Option<String>,
    content_type: Option<String>,
}
pub fn map_formdata_part(line: &str) -> Formdata {
    let binding = line.replace("\r\n\r\n", " ").replace("\r\n", "");
    let mut cleaned_line: Vec<&str> = binding.split("; ").collect();
    assert!(cleaned_line[0] == "Content-Disposition: form-data");
    let mut field_type = "";
    fn get_field_name(field: &Vec<&str>) -> String {
        return field[0][5..].to_string();
    }
    fn get_field_data(cleaned_line: &Vec<&str>) -> (String, String) {
        let cleaned_field = cleaned_line[1].replace("\"", "");
        let splitted_field: Vec<&str> = cleaned_field.split_whitespace().collect();
        let field_name = get_field_name(&splitted_field);
        let value = splitted_field[1];
        (field_name, value.to_string())
    }
    if cleaned_line.len() == 2 {
        field_type = "text";
        let (field_name, value) = get_field_data(&cleaned_line);
        // println!("Text field name {field_name}");
        // println!("Text field value {value}");
        let fdt = FormdataText {
            name: field_name,
            value: value.to_string(),
        };
        return Formdata::FormdataText(fdt);
    } else if cleaned_line.len() == 3 {
        field_type = "file";
        let field_name = cleaned_line[1][5..].to_string().replace("\"", "");
        let mut value: Vec<&str> = cleaned_line[2].split("Content-Type: ").collect();
        println!("Value 0 {:?}", value[0]);
        // println!("Value 1 {:?}", value[1]);

        let mut filetype = String::new();
        for char in value[1].chars() {
            if char != ' ' {
                filetype.push(char);
                value[1] = &value[1][1..];
            } else {
                value[1] = &value[1][1..];
                break;
            }
        }
        println!("File type {filetype}");
        println!("File value {:?}", value[1]);
        let fdf = FormdataFile {
            name: field_name,
            file_type: filetype,
            content: value[1].to_string(),
        };
        return Formdata::FormdataFile(fdf);
    } else {
        panic!("Unknown formdata field type!");
    }
}
pub fn parse_formdata(data: &Vec<u8>) -> FormdataBody {
    // Get separator value
    let data = String::from_utf8_lossy(&data);
    let data_splitted: Vec<&str> = data.split("\r\n").collect();
    let separator = data_splitted[0];
    let mut useful_data: Vec<&str> = data.split(separator).collect();
    useful_data.remove(0);
    useful_data.remove(useful_data.len() - 1);
    // let num_of_fields = useful_data.len() / 2;
    let mut formdatafields: Vec<FormdataText> = vec![];
    let mut formdatafiles: Vec<FormdataFile> = vec![];
    for line in useful_data {
        let output = map_formdata_part(line);
        match output {
            Formdata::FormdataText(text) => formdatafields.push(text),
            Formdata::FormdataFile(file) => formdatafiles.push(file),
        }
    }
    FormdataBody {
        fields: Some(formdatafields),
        files: Some(formdatafiles),
    }
}
