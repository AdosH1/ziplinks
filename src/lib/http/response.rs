use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::Status;
use crate::io::file::load_template;
use crate::lib::http::request;

fn create_response(status: response::Status, content: String) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status.value(),
        content.len(),
        content
    )
}

fn try_create_response(status: Status, filename: &str) -> String {
    let contents = load_template(filename);
    match contents {
        Ok(content) => create_response(status, content),
        Err(e) => {
            create_response(
                response::Status::internal_error,
                String::from("Internal error"),
            ) // todo
        }
    }
}

pub fn triage_response(buffer: &[u8; 1024]) -> String {
    let (_method, _path, _body) = request::parse(&buffer);
    if _method.is_none() || _path.is_none() {
        return try_create_response(response::Status::bad_request, "404.html");
    }
    let method = _method.unwrap();
    let path = _path.unwrap();

    match (method, path.as_str()) {
        (Method::GET, "/") => try_create_response(response::Status::ok, "index.html"),
        _ => try_create_response(response::Status::not_found, "404.html"),
    }
}