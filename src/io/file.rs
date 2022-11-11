use anyhow::Result;
use std::env::current_dir;
use std::path::PathBuf;
use std::{fs, string};

pub fn read_file(path: PathBuf) -> anyhow::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn load_template(file_name: &str) -> anyhow::Result<String> {
    let mut path = current_dir()?;
    path.push(format!("src/data/templates/{}", file_name));
    read_file(path)
}
