use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FtmResult};
use std::str::Utf8Error;
use std::{char, str};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {}

fn get_next_word(text: &str) -> Vec<&str> {
    text.split(|c| c == ' ' || c == '\r').collect()
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut req = str::from_utf8(value)?;

        let mut splits = get_next_word(req);

        if splits.len() != 4 {
            return Err(ParseError::InvalidRequest);
        }

        if splits[2] != "HTTP1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        return Ok(Self {
            method: Method::from_str(splits[2]).ok_or,
            path: splits[1],
            query_string: splits[0],
        });
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidRequest
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FtmResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FtmResult {
        write!(f, "{}", self.message())
    }
}
