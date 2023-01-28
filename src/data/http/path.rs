#[derive(Debug)]
pub struct Path {
    pub value: String,
}

impl Path {
    pub fn try_create(o: Option<String>) -> Option<Path> {
        if let Some(s) = o {
            Some(Path { value: s })
        } else {
            None
        }
    }
}
