use std::fmt;

pub struct Body {
    pub value: String,
}

impl Body {
    pub fn try_create(o: Option<String>) -> Option<Body> {
        o.map(|s| {Body {value: s}})
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("body", &self.value).finish()
    }
}
