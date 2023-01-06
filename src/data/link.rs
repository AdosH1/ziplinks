use std::fmt;
#[derive(Debug)]
pub struct Link {
    pub url : String
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Link: {}", self)
    }
}