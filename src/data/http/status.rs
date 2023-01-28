pub enum Status {
    Ok,            //200
    BadRequest,    //400
    NotFound,      //404
    InternalError, //500
}

impl Status {
    pub fn value(&self) -> &str {
        match &*self {
            Status::Ok => "HTTP/1.1 200 OK",
            Status::InternalError => "HTTP/1.1 500 Internal Server Error",
            Status::BadRequest => "HTTP/1.1 400 Bad Request",
            Status::NotFound => "HTTP/1.1 404 Not Found",
        }
    }
}
