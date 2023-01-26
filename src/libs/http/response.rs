use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::{ContentType, Status};
use crate::data::link::Link;
use crate::io::app::generate::{
    generate_link_opening_webpage, generate_link_referral_webpage, generate_links,
    generate_unique_sub_url,
};
use crate::io::file::load_file;
use crate::libs::http::request;

use std::collections::HashMap;
use std::sync::Mutex;

fn format_response(
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

fn try_create_response_from_file(
    status: Status,
    filename: &str,
    content_type: ContentType,
) -> (String, Vec<u8>) {
    let contents = load_file(filename, &content_type);
    match contents {
        Ok(content) => format_response(status, content_type, content),
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            format_response(
                response::Status::InternalError,
                response::ContentType::TextHtml,
                String::from("Internal error").as_bytes().to_vec(),
            ) // todo
        }
    }
}

fn try_get_links_response(
    method: Method,
    path: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let map = links_hm.lock().unwrap();

    if method.eq(&Method::GET) && map.contains_key(&path) {
        let links = map.get(&path).unwrap();
        let content = generate_link_opening_webpage(links);
        let (headers, body) = format_response(
            Status::Ok,
            ContentType::TextHtml,
            content.as_bytes().to_vec(),
        );
        return (headers, body.to_vec());
    }

    let (headers, body) = try_create_response_from_file(
        response::Status::NotFound,
        "404.html",
        ContentType::TextHtml,
    );
    return (headers, body.to_vec());
}

fn create_link_referral_response(
    body: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let links = generate_links(body);
    let unique_hash = generate_unique_sub_url();

    let webpage = generate_link_referral_webpage(&unique_hash);

    let mut map = links_hm.lock().unwrap();
    let hash = format!("/{}", unique_hash);
    map.insert(hash, links);

    match webpage {
        Ok(page) => format_response(Status::Ok, ContentType::TextHtml, page.as_bytes().to_vec()),
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            try_create_response_from_file(Status::InternalError, "404.html", ContentType::TextHtml)
        }
    }
}

pub fn triage_response(
    buffer: String,
    links_hm: &Mutex<HashMap<String, Vec<Link>>>,
) -> (String, Vec<u8>) {
    let (_method, _path, _body) = request::parse(buffer);
    if _method.is_none() || _path.is_none() {
        return try_create_response_from_file(
            response::Status::BadRequest,
            "404.html",
            ContentType::TextHtml,
        );
    }
    let method = _method.unwrap();
    let path = _path.unwrap();
    let body = _body.unwrap();

    match (&method, path.as_str()) {
        (Method::GET, "/") => {
            try_create_response_from_file(response::Status::Ok, "index.html", ContentType::TextHtml)
        }
        (Method::GET, "/resource/images/marauder") => try_create_response_from_file(
            response::Status::Ok,
            "marauder-starcraft.gif",
            ContentType::ImageGif,
        ),
        (Method::POST, "/generate") => create_link_referral_response(body, &links_hm),
        _ => {
            let (headers, body) = try_get_links_response(method, path, links_hm);
            (headers, body)
        }
    }
}
