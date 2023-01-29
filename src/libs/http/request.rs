use crate::data::http::body::Body;
use crate::data::http::header::Header;
use crate::data::http::method::Method;
use crate::data::http::path::Path;
use crate::libs::util::option;
use std::collections::HashMap;
use std::str;

fn remove_colon(o: Option<String>) -> Option<String> {
    match o {
        Some(s) => {
            let mut string = s.to_string();
            if string.ends_with(":") {
                string.pop();
            }
            Some(string)
        }
        None => None,
    }
}

fn get_value(o: &Option<Vec<&str>>, index: usize) -> Option<String> {
    if let Some(m) = o {
        option::str_to_string(option::deref(m.get(index)))
    } else {
        None
    }
}

fn get_raw<'a>(v: &Vec<&'a str>, index: usize) -> Option<Vec<&'a str>> {
    v.get(index).and_then(|i| Some(i.split(" ").collect()))
}

fn parse_header(raw_header: &str) -> Option<Header> {
    let s: Vec<&str> = raw_header.split("\r\n").collect();

    let raw_method: Option<Vec<&str>> = get_raw(&s, 0);
    let method = Method::try_create(get_value(&raw_method, 0));
    let path = Path::try_create(get_value(&raw_method, 1));
    let protocol = get_value(&raw_method, 2);

    let mut headers = HashMap::new();

    let mut i = 1;
    loop {
        let header = get_raw(&s, i);
        if header.is_none() {
            break;
        }

        let k = get_value(&header, 0);
        let key = remove_colon(k);
        let value = get_value(&header, 1);
        if key.is_some() && value.is_some() {
            headers.insert(key.unwrap(), value.unwrap());
        }
        i += 1;
    }

    Header::try_create(method, path, protocol, Some(headers))
}

fn get_body(raw_body: &str) -> Option<Body> {
    let body_str = Some(raw_body.trim_end_matches(char::from(0)));
    let body = option::str_to_string(body_str);
    Body::try_create(body)
}

pub fn parse(buffer: String) -> (Option<Method>, Option<Path>, Option<Body>) {
    let split: Vec<&str> = buffer.as_str().split("\r\n\r\n").collect();

    let header = match split.get(0) {
        Some(raw_header) => parse_header(raw_header),
        None => None,
    };

    let body = match split.get(1) {
        Some(raw_body) => get_body(raw_body),
        None => None,
    };

    match (header, body) {
        (Some(head), bod) => (head.method, head.path, bod),
        _ => (None, None, None),
    }
}
