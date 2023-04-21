use std::{collections::HashMap, str::FromStr};
mod parser;

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

// Error type for HTTPMethod

impl FromStr for HTTPMethod {
    type Err = String;
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
            _ => Err("Unknown HTTP method".to_string()),
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
        res.push_str(&self.status_code.to_string());
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
