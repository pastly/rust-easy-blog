pub mod file;
mod header;

use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};

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

pub fn render_post_body(post: &file::File, parser: &str) -> String {
    let mut proc = Command::new(parser)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    {
        let mut stdin = proc.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(post.get_body().as_bytes()).expect("Failed to write to stdin");
    }
    let output = proc.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).unwrap()
}
