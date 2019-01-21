mod post {
    use std::fmt;
    use std::io::BufRead;

    #[derive(Debug)]
    pub enum PostParseError {
        IOError(std::io::Error),
        //Misc(),
    }
    impl fmt::Display for PostParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let t = match self {
                &PostParseError::IOError(ref e) => {
                    format!("IOError: {}", e)
                }
                //&PostParseError::Misc() => {
                //    String::from("Miscelaneous post parsing error")
                //}
            };
            write!(f, "{}", t)
        }
    }
    impl std::convert::From<std::io::Error> for PostParseError {
        fn from(error: std::io::Error) -> Self {
            PostParseError::IOError(error)
        }
    }

    #[derive(Debug)]
    pub struct File {
        headers: Vec<HeaderLine>,
        text: String, // All text in file
        body: String, // Only after header and seperator
    }
    impl File {
        fn new() -> Self {
            Self{headers: vec![], text: String::new(), body: String::new()}
        }
        pub fn new_from_buf(buf: Box<BufRead>) -> Result<Self, PostParseError> {
            let mut f = Self::new();
            let mut all_lines = vec![];
            let mut body_lines = vec![];
            let mut doing_headers: bool = true;
            for line in buf.lines() {
                let line = line?;
                all_lines.push(line.clone());
                if doing_headers {
                    let line = line.trim();
                    if line.starts_with('#') { continue; }
                    if line.len() == 0 {
                        doing_headers = false;
                        continue;
                    }
                    f.headers.push(HeaderLine::new(line));
                } else {
                    body_lines.push(line);
                }
            }
            f.text = all_lines.join("\n");
            f.body = body_lines.join("\n");
            Ok(f)
        }
    }
    impl ToString for File {
        fn to_string(&self) -> String {
            self.text.clone()
        }
    }

    #[derive(Debug)]
    struct HeaderLine {
        text: String,
        key: String,
        value: String,
    }
    impl HeaderLine {
        pub fn new(text: &str) -> Self {
            Self{text: text.to_string(), key: String::new(), value: String::new()}
        }
    }
    impl ToString for HeaderLine {
        fn to_string(&self) -> String {
            self.text.clone()
        }
    }
}

use std::io::BufReader;

fn main() {
    let text = "Title: How I Met Your Mother\n#Date: Please\nAuthor: Jake 'n Josh\n\nHi\nthere bob\n\n\n    boyo";
    let br = BufReader::new(text.as_bytes());
    let pf = post::File::new_from_buf(Box::new(br));
    println!("{}", pf.unwrap().to_string());
}
