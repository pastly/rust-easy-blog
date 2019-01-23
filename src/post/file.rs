use super::header::HeaderLine;
use super::PostParseError;
use std::io::BufRead;

#[derive(Debug)]
pub struct File {
    headers: Vec<HeaderLine>,
    text: String, // All text in file
    body: String, // Only after header and seperator
}
impl File {
    fn new() -> Self {
        Self {
            headers: vec![],
            text: String::new(),
            body: String::new(),
        }
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
                if line.starts_with('#') {
                    continue;
                }
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

#[cfg(test)]
mod tests {
    use super::File;
    use std::io::BufReader;

    #[test]
    fn no_headers() {
        let text = "\nHi there";
        let br = BufReader::new(text.as_bytes());
        let pf = File::new_from_buf(Box::new(br)).unwrap();
        assert_eq!(pf.headers.len(), 0);
        assert_eq!(pf.body, "Hi there");
        assert_eq!(pf.to_string(), text);
    }

    #[test]
    fn valid_header() {
        let text = "Aaaa: bbbb\n\nHi There";
        let br = BufReader::new(text.as_bytes());
        let pf = File::new_from_buf(Box::new(br)).unwrap();
        assert_eq!(pf.headers.len(), 1);
        for key in vec!["aaaa", "AAAA", "Aaaa", "aAaA"] {
            assert!(pf.has_header(key));
            assert_eq!(pf.get_header(key).unwrap(), "bbbb");
        }
    }

    #[test]
    fn missing_header() {
        let text = "";
        let br = BufReader::new(text.as_bytes());
        let pf = File::new_from_buf(Box::new(br)).unwrap();
        assert_eq!(pf.headers.len(), 0);
        assert!(!pf.has_header("aaaa"));
        assert_eq!(pf.get_header("aaaa"), None);
    }
}
