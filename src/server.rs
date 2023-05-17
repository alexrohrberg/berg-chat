extern crate threads_pool;

use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread::sleep;
use threads_pool::*;

fn handle_connection(mut stream: TcpStream) {
    let mut idle_count = 0;
    loop {
        //wrap stream in a buf reader
        let mut reader = io::BufReader::new(&mut stream);
        //read bytes from reader into buffer
        let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
        //mark bytes read so buffer will not read them again
        reader.consume(received.len());
        match String::from_utf8(received) {
            Ok(string) => {
                if string.len() > 0 {
                    println!("\r Peer: {string}");
                    print!(">>> ");
                    io::stdout().flush().unwrap();
                } else {
                    if idle_count >= 60 {
                        break;
                    }
                    idle_count += 1;
                }
            },
            Err(_) => {
                println!("Message could not be parsed...");
                break;
            }
        }
        sleep(Duration::from_millis(500));
    }
    println!("Connection closed due to inactivity.");
}

pub fn start_server() {
    //open a new TCP socket on localhost
    let pool = ThreadPool::new(8);
    let source_addr = "127.0.0.1:8081";
    let listener = TcpListener::bind(source_addr).expect("Binding error, please try again");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    handle_connection(stream);
                }).expect("Pool execution failure");
            },
            Err(_) => { 
                println!("Connection failed");
            }
        }
    }
}