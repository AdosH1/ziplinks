mod data;
mod io;
mod lib;
use lib::http::response::triage_response;
use lib::server::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use data::link::Link;
use urlencoding::decode;

use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let pool = ThreadPool::new(1);
    let mut links_hm : Mutex<HashMap<String, Vec<Link>>> = Mutex::new(HashMap::new());

    let listener_result = TcpListener::bind("127.0.0.1:7878");
    match listener_result {
        Ok(listener) => {
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let cn = handle_connection(stream, &links_hm);
                        pool.execute(move || {
                            cn
                        });
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

fn handle_connection(mut stream: TcpStream, mut links_hm : &Mutex<HashMap<String, Vec<Link>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer);

    let b = buffer.to_vec();
    let s = str::from_utf8(&b).unwrap().to_string();
    let decoded = decode(&s).unwrap().into_owned();

    let response = triage_response(decoded, &links_hm);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
