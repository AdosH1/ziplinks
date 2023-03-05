#[derive(Debug)]
pub struct Path {
    pub value: String,
}

impl Path {
    pub fn try_create(o: Option<String>) -> Option<Path> {
        o.map(|s| {Path {value: s}})
    }
}
