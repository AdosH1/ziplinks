use std::str::Bytes;

pub enum Request {
    home,
}

impl Request {
    pub fn value(&self) -> &[u8; 16] {
        match *self {
            Request::home => b"GET / HTTP/1.1\r\n",
        }
    }
}
