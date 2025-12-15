use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display,Debug,Formatter};
use std::str;
#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<&'buff str>,
    //method: super::method::Method,
    method: Method,
}
/*

impl Request {

    /*
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
    */
}
*/

impl<'buf>TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        /*
        match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }


        match str::from_utf8(buf).or(ParseError::InvalidEncoding) {
            Ok(request) => {},
            Err(e) => return Err(e),

        }
        */

        let request = str::from_utf8(buf)?;

        /*
        match get_next_word(request) {
            Some((method, request)) => {},
            None => Return Err(ParseError:;InvalidRequest)

        }
        */

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method  = method.parse()?;
        let mut query_string = None;
        /*
        match path.find("?") {
            Some(i) => {
                query_string = Some(&path[i+1..]);
                path = &path[..i];
            },
            None() => {}
        }

        let q = path.find("?");
        if q is_some() {
            let i = q.unwrap();
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }
        */

        if let Some(i) = path.find("?") {
            query_string = Some(&path[i+1..]);
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
    /*
    let mut iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {},
            None => break;
        }
    }
    */

    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\n' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None

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

/*
impl Debug for ParseError {
    fn debug(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
*/

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::InvalidRequest => "Invalid Request".to_string(),
            Self::InvalidEncoding => "Invalid Encoding".to_string(),
            Self::InvalidProtocol => "Invalid Protocol".to_string(),
            Self::InvalidMethod => "Invalid Method".to_string(),
        }
    } 
}
