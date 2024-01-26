use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display, Formatter};
use std::str::{self, Utf8Error};
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
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
