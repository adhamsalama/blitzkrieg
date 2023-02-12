use std::str::FromStr;

#[derive(Clone)]
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
