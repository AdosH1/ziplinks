#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    UNKNOWN,
}

impl Method {
    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Method::GET, Method::GET) => true,
            (Method::POST, Method::POST) => true,
            (Method::UNKNOWN, Method::UNKNOWN) => true,
            _ => false,
        }
    }

    pub fn try_create(o: Option<String>) -> Option<Method> {
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
}
