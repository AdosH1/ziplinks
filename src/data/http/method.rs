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
        o.map(|m| match m.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNKNOWN,
        })
    }
}
