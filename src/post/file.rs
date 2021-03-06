use super::header::HeaderLine;
use super::PostParseError;
use std::io::BufRead;
use std::time::SystemTime;

pub struct FileOpts {
    /// If true, then all required headers must be present and well-formed
    pub strict_headers: bool,
}

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

    pub fn new_from_buf(
        buf: Box<BufRead>,
        last_modified: Option<SystemTime>,
        opts_in: Option<FileOpts>,
    ) -> Result<Self, PostParseError> {
        // If no FileOpts was given, set the default options
        let opts = if opts_in.is_none() {
            FileOpts {
                strict_headers: true,
            }
        } else {
            opts_in.unwrap()
        };
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
                if line.is_empty() && !f.headers.is_empty() {
                    doing_headers = false;
                    continue;
                } else if line.is_empty() {
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
        let err = if opts.strict_headers {
            f.has_required_headers()
        } else {
            Ok(())
        };
        if err.is_err() {
            Err(PostParseError::MissingHeaders(err.unwrap_err()))
        } else {
            Ok(f)
        }
    }

    fn set_last_modified(&mut self, last_modified: SystemTime) {
        self.last_modified = Some(
            last_modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
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
        let required_headers = ["title", "author", "id", "date"];
        for h in required_headers.iter() {
            if !self.has_header(h) {
                missing.push(*h);
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing.join(", "))
        }
    }

    pub fn has_header(&self, key: &str) -> bool {
        self.get_header(key).is_some()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    //pub fn get_last_modified(&self) -> Option<u64> {
    //    self.last_modified
    //}

    fn hyphenated_title_for_filename(&self, len: usize) -> String {
        self.get_header("title")
            .unwrap()
            .to_lowercase()
            .split(' ')
            .collect::<Vec<&str>>()[0..len]
            .join("-")
    }

    pub fn get_long_rendered_filename(&self) -> String {
        let mut s = self.hyphenated_title_for_filename(3);
        s += "-";
        s += &self.get_header("id").unwrap();
        s += ".html";
        s
    }

    pub fn get_suggested_source_filename(&self) -> String {
        let mut s = self.hyphenated_title_for_filename(3);
        s += "-";
        s += &self.get_header("id").unwrap();
        s += ".reb";
        s
    }
}
impl ToString for File {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{File, FileOpts};
    use std::io::BufReader;

    #[test]
    fn first_line_blank() {
        let text = "\naaaa: bbbb\n\nHi there";
        let br = BufReader::new(text.as_bytes());
        let pf = File::new_from_buf(
            Box::new(br),
            None,
            Some(FileOpts {
                strict_headers: false,
            }),
        )
        .unwrap();
        assert_eq!(pf.headers.len(), 1);
        assert!(pf.has_header("aaaa"));
        assert_eq!(pf.get_header("aaaa").unwrap(), "bbbb");
        assert_eq!(pf.body, "Hi there");
        assert_eq!(pf.to_string(), text);
    }

    #[test]
    fn no_headers() {
        let text = "\nHi there";
        let br = BufReader::new(text.as_bytes());
        let err = File::new_from_buf(
            Box::new(br),
            None,
            Some(FileOpts {
                strict_headers: false,
            }),
        );
        assert!(err.is_err());
    }

    #[test]
    fn valid_header() {
        let text = "Aaaa: bbbb\n\nHi There";
        let br = BufReader::new(text.as_bytes());
        let pf = File::new_from_buf(
            Box::new(br),
            None,
            Some(FileOpts {
                strict_headers: false,
            }),
        )
        .unwrap();
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
        let pf = File::new_from_buf(
            Box::new(br),
            None,
            Some(FileOpts {
                strict_headers: false,
            }),
        )
        .unwrap();
        assert_eq!(pf.headers.len(), 0);
        assert!(!pf.has_header("aaaa"));
        assert_eq!(pf.get_header("aaaa"), None);
    }
}
