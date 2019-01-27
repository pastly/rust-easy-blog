pub mod file;
mod header;

use std::fmt;

#[derive(Debug)]
pub enum PostParseError {
    IOError(std::io::Error),
    MissingHeaders(String),
    NotAHeader(String),
}
impl fmt::Display for PostParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = &match self {
            PostParseError::IOError(ref e) => format!("IOError: {}", e),
            PostParseError::MissingHeaders(ref e) => format!("MissingHeaders: {}", e),
            PostParseError::NotAHeader(ref e) => format!("NotAHeader: {}", e),
        };
        write!(f, "{}", t)
    }
}
impl std::convert::From<std::io::Error> for PostParseError {
    fn from(error: std::io::Error) -> Self {
        PostParseError::IOError(error)
    }
}
