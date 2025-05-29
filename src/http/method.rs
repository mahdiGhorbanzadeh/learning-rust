use super::ParseError;

pub enum Method {
    GET,
    POST,
    PUT,
}

impl Method {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}
