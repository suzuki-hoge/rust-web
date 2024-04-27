use std::fmt::{Debug, Formatter};

pub struct Response {
    pub status_code: u16,
    pub content: String,
}

impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Response {{ HTTP/1.1 {} {} }}", self.status_code, self.content)
    }
}

impl Response {
    pub fn into_bytes(self) -> Vec<u8> {
        let message = format!("{} {}\r\n\r\n{}", "HTTP/1.1", self.status_code, self.content);
        message.into_bytes()
    }
}