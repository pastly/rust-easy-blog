use super::header::HeaderLine;
use super::PostParseError;
use std::io::BufRead;
use std::time::SystemTime;

#[derive(Debug)]
pub struct File {
    headers: Vec<HeaderLine>,
    text: String, // All text in file
    body: String, // Only after header and seperator
    last_modified: Option<u64>,
}
impl File {
    fn new() -> Self {
        Self {
            headers: vec![],
            text: String::new(),
            body: String::new(),
            last_modified: None,
        }
    }

    pub fn new_from_buf(buf: Box<BufRead>, last_modified: Option<SystemTime>) -> Result<Self, PostParseError> {
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
                if line.is_empty() {
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
        if last_modified.is_some() {
            f.set_last_modified(last_modified.unwrap());
        }
        let err = f.has_required_headers();
        return if err.is_err() {
            Err(PostParseError::MissingHeaders(err.unwrap_err()))
        } else {
            Ok(f)
        };
    }

    fn set_last_modified(&mut self, last_modified: SystemTime) {
        self.last_modified = Some(last_modified
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs());
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

    pub fn has_required_headers(&self) -> Result<(), String> {
        let mut missing = vec![];
        let required_headers = ["title", "author"];
        for h in required_headers.iter() {
            if !self.has_header(h) {
                missing.push(*h);
            }
        }
        if missing.is_empty() {
            return Ok(());
        } else {
            return Err(missing.join(", "));
        }
    }

    pub fn has_header(&self, key: &str) -> bool {
        self.get_header(key).is_some()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_last_modified(&self) -> Option<u64> {
        self.last_modified
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
