use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::{ContentType, Status};
use crate::data::link::Link;
use crate::io::app::generate::{
    generate_bad_request, generate_homepage, generate_internal_error, generate_link_opening_page,
    generate_link_page, generate_not_found, generate_sub_url,
};
use crate::io::file::load_file;
use crate::libs::http::request;
use crate::libs::parse::links;

use std::collections::HashMap;
use std::sync::Mutex;

fn insert_headers(
    status: Status,
    content_type: ContentType,
    content: Vec<u8>,
) -> (String, Vec<u8>) {
    (
        format!(
            "{}\r\n{}\r\nContent-Length: {}\r\n\r\n",
            status.value(),
            content_type.value(),
            content.len()
        ),
        content,
    )
}

fn internal_error() -> (String, Vec<u8>) {
    let internal_error = generate_internal_error();
    match internal_error {
        Ok(page) => insert_headers(
            response::Status::InternalError,
            response::ContentType::TextHtml,
            page,
        ),
        _ => insert_headers(
            response::Status::InternalError,
            response::ContentType::TextHtml,
            String::from("Internal error").as_bytes().to_vec(),
        ),
    }
}

fn not_found_error() -> (String, Vec<u8>) {
    let not_found_page = generate_not_found();
    format_header(Status::NotFound, not_found_page, ContentType::TextHtml)
}

fn try_get_file(status: Status, filename: &str, content_type: ContentType) -> (String, Vec<u8>) {
    let contents = load_file(filename, &content_type);
    format_header(status, contents, content_type)
}

fn format_header(
    status: Status,
    contents: anyhow::Result<Vec<u8>>,
    content_type: ContentType,
) -> (String, Vec<u8>) {
    match contents {
        Ok(content) => insert_headers(status, content_type, content),
        _ => internal_error(),
    }
}

fn try_retrieve_links(
    method: Method,
    path: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let map = links_hm.lock().unwrap();

    if method.eq(&Method::GET) && map.contains_key(&path) {
        let links = map.get(&path).unwrap();
        let contents = generate_link_opening_page(links);

        match contents {
            Ok(body) => {
                let (headers, body) = insert_headers(Status::Ok, ContentType::TextHtml, body);
                return (headers, body);
            }
            Err(e) => {
                println!("Error occurred: {:#?}", e);
                return internal_error();
            }
        }
    }

    not_found_error()
}

fn try_create_links(
    body: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let links = links::parse_body_to_links(body);
    let unique_hash = generate_sub_url();

    let webpage = generate_link_page(&unique_hash);

    let mut map = links_hm.lock().unwrap();
    let hash = format!("/{}", unique_hash);
    map.insert(hash, links);

    match webpage {
        Ok(page) => insert_headers(Status::Ok, ContentType::TextHtml, page),
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            not_found_error()
        }
    }
}

pub fn triage_response(
    buffer: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let (_method, _path, _body) = request::parse(buffer);

    if _method.is_none() || _path.is_none() {
        return format_header(
            Status::BadRequest,
            generate_bad_request(),
            ContentType::TextHtml,
        );
    }
    let method = _method.unwrap();
    let path = _path.unwrap();
    let body = _body.unwrap();

    match (&method, path.as_str()) {
        (Method::GET, "/") => {
            let home = generate_homepage();
            format_header(Status::Ok, home, ContentType::TextHtml)
        }
        (Method::GET, "/resource/images/marauder") => try_get_file(
            response::Status::Ok,
            "marauder-starcraft.gif",
            ContentType::ImageGif,
        ),
        (Method::POST, "/generate") => try_create_links(body, &links_hm),
        _ => {
            let (headers, body) = try_retrieve_links(method, path, links_hm);
            (headers, body)
        }
    }
}
