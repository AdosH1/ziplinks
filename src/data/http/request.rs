use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    UNKNOWN,
}

impl Method {
    pub 
    fn eq(&self, other: &Self) -> bool {

        match (self, other) {
            (Method::GET, Method::GET) => true,
            (Method::POST, Method::POST) => true,
            (Method::UNKNOWN, Method::UNKNOWN) => true,
            _ => false
        }
    }
}

pub fn get_method(s: &str) -> Method {
    match s {
        "GET" => Method::GET,
        "POST" => Method::POST,
        _ => Method::UNKNOWN,
    }
}

pub fn get_method_option(o: Option<String>) -> Option<Method> {
    if let Some(s) = o {
        match s.as_str() {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            _ => Some(Method::UNKNOWN),
        }
    } else {
        None
    }
}

pub struct Header {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("protocol", &self.protocol)
            .field("headers", &self.headers)
            .finish()
    }
}

pub struct Body {
    pub body: Option<String>,
}
impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("body", &self.body).finish()
    }
}
