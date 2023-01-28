pub enum ContentType {
    ImageGif,
    TextHtml,
}

impl ContentType {
    pub fn value(&self) -> &str {
        match &*self {
            ContentType::ImageGif => "Content-Type: image/gif",
            ContentType::TextHtml => "Content-Type: text/html",
        }
    }
}
