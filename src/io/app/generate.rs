use std::f32::consts::E;
use std::io::Error;
use anyhow::Result;
use random_string::generate;

use crate::io::file;
use crate::data::link::Link;
use crate::lib::html::generate::open_links;

pub fn generate_link_webpage(sub_url : String,links : &Vec<Link>) -> anyhow::Result<String> {

    let mut webpage = String::new();
    webpage.push_str("
    <!DOCTYPE html>
    <html lang='en'>
        <head>
            <meta charset='utf-8'>");
    // Add things in the head here
            webpage.push_str(&open_links(links).1);
            webpage.push_str("
        </head>
    <body>");
    // Add body things here
    let body = format!("Here is your unique sub-url: http://localhost:7878/{}", sub_url);
    webpage.push_str(&body);
    webpage.push_str("
    </body>
    </html>");

    Ok(webpage)
}

pub fn generate_unique_sub_url() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let sub_url = generate(6, charset);

    // todo : check if its unique against database (when we have one), if not - regen
    // worth figuring our when collision gets bad, increase length or better way of avoiding collisions

    sub_url
}

pub fn generate_links(body : String) -> Vec<Link> {
    let b : String = body.chars().skip(6).collect();
    let v: Vec<&str> = b.split("%0D%0A").collect();
    println!("Vec: {:#?}", &v);
    let mut links = Vec::new();

    for link in v.iter() {
        let l = link.to_owned().to_owned();
        println!("Link: {:#?}", &l);
        links.push(Link {url : l});
    }
    links
}