use crate::data::http::request;
use crate::data::http::response;
use crate::data::http::response::Status;
use crate::io::file::load_template;

use super::request::triage_request;

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

    triage_request(&buffer);

    if buffer.starts_with(request::Request::home.value()) {
        try_create_response(response::Status::ok, "hello.html")
    } else {
        try_create_response(response::Status::not_found, "404.html")
    }
}
