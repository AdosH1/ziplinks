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

pub enum ContentType {
    image_gif,
    image_jpeg,
    image_png,
    text_html,
    text_plain,
}

impl ContentType {
    pub fn value(&self) -> &str {
        match &*self {
            ContentType::image_gif => "Content-Type: image/gif",
            ContentType::image_jpeg => "Content-Type: image/jpeg",
            ContentType::image_png => "Content-Type: image/png",
            ContentType::text_html => "Content-Type: text/html",
            ContentType::text_plain => "Content-Type: text/plain",
        }
    }
}