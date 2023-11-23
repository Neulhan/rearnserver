use crate::http::header;

use super::header::Header;
use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    header: Header<'buf>,
    method: Method,
    body: &'buf str,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // "GET /search?name=abc&sort=1 HTTP/1.1"
        let request = str::from_utf8(buf)?;

        let mut start_line = "";
        let mut header_line = "";
        let mut body = "";

        if let Some(i) = request.find("\r\n") {
            start_line = &request[..i];
            let request = &request[i..];

            if let Some(j) = request.find("\r\n\r\n") {
                header_line = &request[..j];
                body = &request[j + 4..];

                if let Some(k) = body.find("\0") {
                    body = &body[..k];
                }
            }
        }

        let mut first_line_split = start_line.split(" ");

        let method = first_line_split.next().ok_or(ParseError::InvalidRequest)?;
        let mut path = first_line_split.next().ok_or(ParseError::InvalidRequest)?;
        let protocol = first_line_split.next().ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let header = Header::from(header_line);

        Ok(Self {
            path,
            query_string,
            header,
            method,
            body,
        })
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
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
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

impl Error for ParseError {}
