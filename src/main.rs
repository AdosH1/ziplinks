mod data;
mod io;
mod lib;
use lib::http::response::triage_response;
use lib::server::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() {
    let pool = ThreadPool::new(1);

    let listener_result = TcpListener::bind("127.0.0.1:7878");
    match listener_result {
        Ok(listener) => {
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        pool.execute(|| {
                            handle_connection(stream);
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

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer);

    let b = buffer.to_vec();
    let s = str::from_utf8(&b).unwrap();

    let response = triage_response(s.to_string());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
