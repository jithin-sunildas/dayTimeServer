use chrono::Local;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{io, thread};

fn handle_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().expect("Failed to read client address!");
    println!("client {} connected.", client_addr);
    let now = Local::now();
    let now_string = now.format("%a, %d %b %Y %H:%M:%S %z").to_string();

    //writing the string to the stream.
    match stream.write(now_string.as_bytes()) {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            eprintln!("Failed to write day time to client: {}", client_addr)
        }
    };

    println!("connection handler finished with client: {}", client_addr);
}

fn main() -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:7878");
    println!("Day-Time server listning on 127.0.0.1:7878");

    for stream in listner?.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error {} accepting connection", e);
            }
        }
    }
    Ok(())
}
