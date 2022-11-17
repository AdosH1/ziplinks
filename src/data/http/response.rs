pub enum Status {
    ok,             //200
    internal_error, //500
    bad_request,    //400
    not_found,      //404
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
