mod data;
mod io;
mod libs;

use data::link::Link;
use data::settings::Settings;
use io::app::response::triage_response;
use libs::server::threadpool::ThreadPool;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::Mutex;
use urlencoding::decode;

const MAX_BUFFER_SIZE: usize = 32768;

fn main() {
    let config = Settings::new();
    let settings = match config {
        Ok(s) => s,
        Err(e) => {
            println!("An error occured loading config: {}", e);
            return;
        }
    };
    
    let addr = format!("0.0.0.0:{}", settings.server.port);
    let pool = ThreadPool::new(settings.server.num_threads);
    let links_hm: Mutex<HashMap<String, Vec<Link>>> = Mutex::new(HashMap::new());

    let listener_result = TcpListener::bind(&addr);
    match listener_result {
        Ok(listener) => {
            println!("Listening on {}...", addr);
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let cn = handle_connection(stream, settings.clone(), &links_hm);
                        pool.execute(move || cn);
                    }
                    Err(stream_error) => {
                        println!("{}", stream_error);
                    }
                }
            }
        }
        Err(listener_error) => {
            println!("{}", listener_error);
        }
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, settings: Settings, links_hm: &Mutex<HashMap<String, Vec<Link>>>) {
    let mut buffer = [0; MAX_BUFFER_SIZE];
    let _ = stream.read(&mut buffer);

    let b = buffer.to_vec();
    let s = str::from_utf8(&b).unwrap().to_string();
    let decoded = decode(&s).unwrap().into_owned();

    let (headers, body) = triage_response(decoded, settings, &links_hm);
    stream.write(headers.as_bytes()).unwrap();
    stream.write(&body).unwrap();
    stream.flush().unwrap();
}
