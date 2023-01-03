use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::Status;
use crate::io::file::load_template;
use crate::lib::http::request;
use crate::data::link::Link;
use crate::io::app::generate::{self, generate_links, generate_link_webpage, generate_unique_sub_url};

fn format_response(status: response::Status, content: String) -> String {
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
        Ok(content) => format_response(status, content),
        Err(e) => {
            format_response(
                response::Status::internal_error,
                String::from("Internal error"),
            ) // todo
        }
    }
}

fn create_link_response(body : String) -> String {
    
    let links = generate_links(body);
    let unique_hash = generate_unique_sub_url();
    let webpage = generate_link_webpage(unique_hash, &links);

    match webpage {
        Ok(page) => format_response(Status::ok, page),
        Err(e) => try_create_response_from_file(Status::internal_error, "404.html")
    }

}

pub fn triage_response(buffer: String) -> String {
    let (_method, _path, _body) = request::parse(buffer);
    if _method.is_none() || _path.is_none() {
        return try_create_response_from_file(response::Status::bad_request, "404.html");
    }
    let method = _method.unwrap();
    let path = _path.unwrap();
    let body = _body.unwrap();
    println!("Body: {:#?}", &body);

    match (method, path.as_str()) {
        (Method::GET, "/") => try_create_response_from_file(response::Status::ok, "index.html"),
        (Method::POST, "/generate") => create_link_response(body),
        _ => try_create_response_from_file(response::Status::not_found, "404.html"),
    }
}
