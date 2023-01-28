use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

use crate::data::http::content_type::ContentType;

pub fn read_file(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    let content = fs::read(path)?;

    Ok(content)
}

pub fn load_file(file_name: &str, content_type: &ContentType) -> anyhow::Result<Vec<u8>> {
    let mut path = current_dir()?;

    match content_type {
        ContentType::TextHtml => path.push(format!("src/data/templates/{}", file_name)),
        ContentType::ImageGif => path.push(format!("src/data/resource/images/{}", file_name)),
    }

    read_file(path)
}
