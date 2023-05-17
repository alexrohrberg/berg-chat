use std::io::{self, Write};
use std::thread;
use std::net::TcpStream;
use berg_chat::server;

fn main() {
    println!("Welcome to BergChat 1.0\nA Simple Multithreaded P2P Chat App");
    //process incoming messages on a non blocking thread
    let handle = thread::spawn(move || {
        server::start_server();
    });
    loop {
        println!("Please enter the IP you wish to chat with. (e.g. 127.0.0.1)");

        let mut dest_ip = String::new();

        io::stdin()
            .read_line(&mut dest_ip)
            .expect("Failed to read line");

        let dest_ip: String = match dest_ip.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };

        println!("Please enter the port you wish to chat with. (e.g. 8080)");

        let mut dest_port = String::new();

        io::stdin()
            .read_line(&mut dest_port)
            .expect("Failed to read line");

        let dest_port: String = match dest_port.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };

        println!("Attempting to connect to {dest_ip}:{dest_port}");
        let dest_addr = format!("{dest_ip}:{dest_port}");
        if let Ok(mut stream) = TcpStream::connect(dest_addr) {
            println!("Connected to the peer server!");
            println!("Please enter a new message below");
            loop {
                print!(">>> ");
                io::stdout().flush().unwrap();
                let mut msg = String::new();

                io::stdin()
                    .read_line(&mut msg)
                    .expect("Failed to read line");

                let msg: String = match msg.trim().parse() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                match stream.write(msg.as_bytes()) {
                    Ok(_) => {},
                    Err(_) => {
                        println!("Connection to peer was lost.");
                        break;
                    }
                }
            }
        } else {
            println!("Couldn't connect to peer server...");
            continue;
        }
        break;
    }
    handle.join().unwrap();
}