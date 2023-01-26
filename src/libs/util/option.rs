pub fn deref<'a>(o: Option<&&'a str>) -> Option<&'a str> {
    match o {
        Some(s) => Some(*s),
        None => None,
    }
}

pub fn str_to_string(o: Option<&str>) -> Option<String> {
    match o {
        Some(s) => Some(String::from(s)),
        None => None,
    }
}
