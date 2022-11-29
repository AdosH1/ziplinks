use std::f32::consts::E;
use std::io::Error;
use anyhow::Result;

use crate::io::file;
use crate::data::link::Link;

pub fn generate_links(links : &Vec<Link>) -> anyhow::Result<String> {
    let mut template = file::load_template("links_base.html")?;
    template = template.replace('\n', "");
    template = template.replace('"', "'");

    let mut webpage = String::new();
    let first = &template[..100];
    let last = &template[101..];

    let mut links_str = String::new();

    for link in links {
        let t = format!("window.open('{}');", link.url);
        links_str.push_str(&t);
    }

    webpage.push_str(&first);
    webpage.push_str(&links_str);
    webpage.push_str(&last);

    Ok(webpage)
}