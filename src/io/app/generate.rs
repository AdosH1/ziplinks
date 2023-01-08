use std::f32::consts::E;
use std::io::Error;
use anyhow::Result;
use random_string::generate;

use crate::io::file;
use crate::data::link::Link;
use crate::lib::html::generate::open_links;

pub fn generate_link_referral_webpage(sub_url : &String) -> anyhow::Result<String> {

    let mut webpage = String::new();
    webpage.push_str("
    <!DOCTYPE html>
    <html lang='en'>
    <head>
        <meta charset='utf-8'>
        <title>ziplinks</title>
        <link rel='stylesheet' href='https://fonts.googleapis.com/icon?family=Material+Icons'>
        <link rel='stylesheet' href='https://code.getmdl.io/1.3.0/material.indigo-pink.min.css'>
        <script defer src='https://code.getmdl.io/1.3.0/material.min.js'></script>
    </head>
    <body>
    <style>
      .demo-layout-waterfall .mdl-layout__header-row .mdl-navigation__link:last-of-type  {
        padding-right: 0;
      }
    </style>
      
      <div class='demo-layout-waterfall mdl-layout mdl-js-layout'>
        <header class='mdl-layout__header mdl-layout__header--waterfall' style='background-color: #E1C699;'>
          <!-- Top row, always visible -->
          <div class='mdl-layout__header-row'>
            <!-- Title -->
            <span class='mdl-layout-title'>ziplinks</span>
            <div class='mdl-layout-spacer'></div>
          </div>
        </header>
        <div class='mdl-layout__drawer'>
          <span class='mdl-layout-title'>ziplinks</span>
          <nav class='mdl-navigation'>
            <a class='mdl-navigation__link' href='/about'>About</a>
            <a class='mdl-navigation__link' href='/'>Generate</a>
          </nav>
        </div>
        <main class='mdl-layout__content' style='background-color: #F5F5DC;'>
            <div class='page-content'><!-- Your content goes here -->");

    // Add body things here
    let body = format!("
    <center>
    <br/>
    <p>Please find your links here: <a href='http://localhost:7878/{}'>http://localhost:7878/{}</a></p>
    </center>", &sub_url, &sub_url);
    webpage.push_str(&body);

    webpage.push_str("
             </div>
        </main>
    </div>
    </body>
    </html>");

    Ok(webpage)
}

pub fn generate_link_opening_webpage(links : &Vec<Link>) -> String {

    let mut webpage = String::new();
    webpage.push_str("
    <!DOCTYPE html>
    <html lang='en'>
    <head>
        <meta charset='utf-8'>
        <title>ziplinks</title>
        <link rel='stylesheet' href='https://fonts.googleapis.com/icon?family=Material+Icons'>
        <link rel='stylesheet' href='https://code.getmdl.io/1.3.0/material.indigo-pink.min.css'>
        <script defer src='https://code.getmdl.io/1.3.0/material.min.js'></script>");
    // <--- Add head stuff here --->
    webpage.push_str(&open_links(links).1);
    
    webpage.push_str("</head>
    <body>
    <style>
      .demo-layout-waterfall .mdl-layout__header-row .mdl-navigation__link:last-of-type  {
        padding-right: 0;
      }
    </style>
      
      <div class='demo-layout-waterfall mdl-layout mdl-js-layout'>
        <header class='mdl-layout__header mdl-layout__header--waterfall' style='background-color: #E1C699;'>
          <!-- Top row, always visible -->
          <div class='mdl-layout__header-row'>
            <!-- Title -->
            <span class='mdl-layout-title'>ziplinks</span>
            <div class='mdl-layout-spacer'></div>
          </div>
        </header>
        <div class='mdl-layout__drawer'>
          <span class='mdl-layout-title'>ziplinks</span>
          <nav class='mdl-navigation'>
            <a class='mdl-navigation__link' href='/about'>About</a>
            <a class='mdl-navigation__link' href='/'>Generate</a>
          </nav>
        </div>
        <main class='mdl-layout__content' style='background-color: #F5F5DC;'>
            <div class='page-content'><!-- Your content goes here -->");
    
    // Add body things here
    let body = format!("
    <br/>
    <center>
        <h1>Enjoy!</h1>
        <img src='/src/data/resource/marauder-starcraft.gif' alt='A marauder from the game Starcraft 2 dancing.'/>
    </center>");
    webpage.push_str(&body);

    webpage.push_str("
        </div>
    </main>
    </div>
    </body>
    </html>");

    webpage
}

pub fn generate_unique_sub_url() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let sub_url = generate(6, charset);

    // todo : check if its unique against database (when we have one), if not - regen
    // worth figuring our when collision gets bad, increase length or better way of avoiding collisions

    format!("{}", sub_url)
}

pub fn generate_links(body : String) -> Vec<Link> {
    let b : String = body.chars().skip(6).collect();
    let v: Vec<&str> = b.split("\r\n").collect();
    println!("Vec: {:#?}", &v);
    let mut links = Vec::new();

    for link in v.iter() {
        let l = link.to_owned().to_owned();
        println!("Link: {:#?}", &l);
        links.push(Link {url : l});
    }
    links
}