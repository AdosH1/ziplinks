use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Link: {}", self)
    }
}
