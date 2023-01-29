use crate::data::http::method::Method;
use crate::data::http::path::Path;
use std::collections::HashMap;
use std::fmt;

pub struct Header {
    pub method: Option<Method>,
    pub path: Option<Path>,
    pub protocol: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl Header {
    pub fn try_create(
        method: Option<Method>,
        path: Option<Path>,
        protocol: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Option<Header> {
        if method.is_some() && path.is_some() && protocol.is_some() {
            return Some(Header {
                method,
                path,
                protocol,
                headers,
            });
        }
        return None;
    }
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
