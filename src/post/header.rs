use super::PostParseError;

#[derive(Debug)]
pub struct HeaderLine {
    text: String,
    pub key: String,
    pub value: String,
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
        if key.len() == 0 || value.len() == 0 {
            return Err(PostParseError::NotAHeader(text.to_string()));
        }
        Ok(Self{text: text.to_string(), key: key.to_string(), value: value.to_string()})
    }
}
impl ToString for HeaderLine {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn invalid() {
        for text in vec!["nocolon", "", ":", "a:", ":b"] {
            let h = HeaderLine::new(text);
            assert!(h.is_err());
        }
    }
}
