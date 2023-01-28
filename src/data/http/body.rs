use std::fmt;

pub struct Body {
    pub value: String,
}

impl Body {
    pub fn try_create(o: Option<String>) -> Option<Body> {
        match o {
            Some(s) => Some(Body { value: s }),
            None => None,
        }
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("body", &self.value).finish()
    }
}
