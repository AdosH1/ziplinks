use crate::data::http::request::Method;
use crate::data::http::response;
use crate::data::http::response::Status;
use crate::io::file::load_template;
use crate::lib::http::request;
use crate::data::link::Link;
use crate::io::app::generate::{self, generate_links, generate_link_referral_webpage, generate_link_opening_webpage, generate_unique_sub_url};

use std::collections::HashMap;
use std::sync::Mutex;

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

fn try_get_links_response(method: &Method, path: &String, mut links_hm : &Mutex<HashMap<String, Vec<Link>>>) -> String {

    let mut map = links_hm.lock().unwrap();
    //let p = format!("{}",path);

    if method.eq(&Method::GET) && map.contains_key(path) {
        let links = map.get(path).unwrap();
        return format_response(Status::ok, generate_link_opening_webpage(links));
    }
    
    try_create_response_from_file(response::Status::not_found, "404.html")
}

fn create_link_referral_response(body : String, mut links_hm : &Mutex<HashMap<String, Vec<Link>>>) -> String {
    
    let links = generate_links(body);
    let unique_hash = generate_unique_sub_url();

    let webpage = generate_link_referral_webpage(&unique_hash);

    let mut map = links_hm.lock().unwrap();
    let hash = format!("/{}", unique_hash);
    map.insert(hash, links);
    println!("Dictionary updated, elements: {:#?}", map);

    match webpage {
        Ok(page) => format_response(Status::ok, page),
        Err(e) => try_create_response_from_file(Status::internal_error, "404.html")
    }

}

pub fn triage_response(buffer: String, mut links_hm : &Mutex<HashMap<String, Vec<Link>>>) -> String {
    let (_method, _path, _body) = request::parse(buffer);
    println!("method: {:#?}", _method);
    println!("path: {:#?}", _path);
    println!("body: {:#?}", _body);
    if _method.is_none() || _path.is_none() {
        return try_create_response_from_file(response::Status::bad_request, "404.html");
    }
    let method = _method.unwrap();
    let path = _path.unwrap();
    let body = _body.unwrap();

    match (&method, path.as_str()) {
        (Method::GET, "/") => try_create_response_from_file(response::Status::ok, "index.html"),
        (Method::POST, "/generate") => create_link_referral_response(body, &links_hm),
        _ => try_get_links_response(&method, &path, &links_hm)
    }
}
