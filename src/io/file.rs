use anyhow::Result;
use std::env::current_dir;
use std::path::PathBuf;
use std::{fs, string};

use crate::data::http::response::ContentType;

pub fn read_file(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    let content = fs::read(path)?;
    
    Ok(content)
}

pub fn load_file(file_name: &str, content_type: &ContentType) -> anyhow::Result<Vec<u8>> {
    let mut path = current_dir()?;

    match content_type {
        ContentType::text_html => path.push(format!("src/data/templates/{}", file_name)),
        ContentType::text_plain => path.push(format!("src/data/templates/{}", file_name)),
        ContentType::image_gif => path.push(format!("src/data/resource/images/{}", file_name)),
        ContentType::image_jpeg => path.push(format!("src/data/resource/images/{}", file_name)),
        ContentType::image_png => path.push(format!("src/data/resource/images/{}", file_name)),
    }
    
    read_file(path)
}