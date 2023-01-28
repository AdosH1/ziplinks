use std::fmt;
#[derive(Debug)]
pub struct Link {
    pub value: String,
}

impl Link {
    pub fn try_create(s: String) -> Option<Link> {
        match s.starts_with("http") {
            true => Some(Link { value: s }),
            false => None,
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Link: {}", self)
    }
}
