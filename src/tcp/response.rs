use std::fmt::{Display, Formatter};

pub struct Response {
    pub status_code: u16,
    pub content: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = if 80 < self.content.len() { &format!("{} ...", &self.content[..80]) } else { &self.content };
        write!(f, "HTTP/1.1 {} {}", self.status_code, content)
    }
}

impl Response {
    pub fn into_bytes(self) -> Vec<u8> {
        let message = format!("{} {}\r\n\r\n{}", "HTTP/1.1", self.status_code, self.content);
        message.into_bytes()
    }
}
