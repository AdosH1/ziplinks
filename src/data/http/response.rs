pub enum Status {
    ok,
    internal_error,
    not_found,
}

impl Status {
    pub fn value(&self) -> &str {
        match &*self {
            ok => "HTTP/1.1 200 OK",
            internal_error => "HTTP/1.1 500 Internal Server Error",
            not_found => "HTTP/1.1 404 Not Found",
        }
    }
}
