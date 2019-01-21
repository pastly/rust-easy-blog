mod post {
    use std::fmt;
    use std::io::BufRead;

    #[derive(Debug)]
    pub enum PostParseError {
        IOError(std::io::Error),
        NotAHeader(String),
        //Misc(),
    }
    impl fmt::Display for PostParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let t = match self {
                &PostParseError::IOError(ref e) => {
                    format!("IOError: {}", e)
                }
                &PostParseError::NotAHeader(ref e) => {
                    format!("NotAHeader: {}", e)
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
                    f.headers.push(HeaderLine::new(line)?);
                } else {
                    body_lines.push(line);
                }
            }
            f.text = all_lines.join("\n");
            f.body = body_lines.join("\n");
            Ok(f)
        }
        pub fn get_header(&self, key: &str) -> Option<String> {
            let key = key.to_lowercase();
            for h in &self.headers {
                if h.key.to_lowercase() == key {
                    return Some(h.value.clone());
                }
            }
            None
        }

        pub fn has_header(&self, key: &str) -> bool {
            self.get_header(key).is_some()
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
        pub fn new(text: &str) -> Result<Self, PostParseError> {
            let colon_idx = text.find(':');
            if colon_idx.is_none() {
                return Err(PostParseError::NotAHeader(text.to_string()));
            }
            let colon_idx = colon_idx.unwrap();
            let key = &text[0..colon_idx].trim();
            let value = &text[colon_idx+1..].trim();
            Ok(Self{text: text.to_string(), key: key.to_string(), value: value.to_string()})
        }
    }
    impl ToString for HeaderLine {
        fn to_string(&self) -> String {
            self.text.clone()
        }
    }

    #[cfg(test)]
    mod headerline_tests {
        use super::HeaderLine;

        #[test]
        fn trimming() {
            let test = |h: HeaderLine| {
                assert_eq!(h.key, "aaaa");
                assert_eq!(h.value, "bbbb");
            };
            for text in vec!["aaaa:bbbb", "aaaa: bbbb", " aaaa : bbbb "] {
                let h = HeaderLine::new(text).unwrap();
                test(h);
            }
        }

        #[test]
        fn multiword() {
            let h = HeaderLine::new("Alpha Bet: Soup Four").unwrap();
            assert_eq!(h.key, "Alpha Bet");
            assert_eq!(h.value, "Soup Four");
        }
    }
}

use std::io::BufReader;

fn main() {
    let text = "Title: How I Met Your Mother\n#Date: Please\nAuthor: Jake 'n Josh\n\nHi\nthere bob\n\n\n    boyo";
    let br = BufReader::new(text.as_bytes());
    let pf = post::File::new_from_buf(Box::new(br));
    if pf.is_err() {
        println!("ERROR: {}", pf.unwrap_err());
        return;
    }
    let pf = pf.unwrap();
    assert!(pf.has_header("title"));
    println!("{}", pf.get_header("author").unwrap());
    println!("OK");
}
