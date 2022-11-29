use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::Status;
use crate::io::file::load_template;
use crate::lib::http::request;
use crate::data::link::Link;
use crate::io::app::generate::{self, generate_links};

fn create_response(status: response::Status, content: String) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status.value(),
        content.len(),
        content
    )
}

fn try_create_response_from_file(status: Status, filename: &str) -> String {
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

fn try_create_response_from_string(status: Status, content : String) -> String {
    create_response(status, content)
}

pub fn triage_response(buffer: &[u8; 1024]) -> String {
    let (_method, _path, _body) = request::parse(&buffer);
    if _method.is_none() || _path.is_none() {
        return try_create_response_from_file(response::Status::bad_request, "404.html");
    }
    let method = _method.unwrap();
    let path = _path.unwrap();

    match (method, path.as_str()) {
        (Method::GET, "/") => try_create_response_from_file(response::Status::ok, "index.html"),
        _ => try_create_response_from_file(response::Status::not_found, "404.html"),
    }
}
