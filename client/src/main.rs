use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Succesfully connected to the server 127.0.0.1:7878");
            stream
        }
        Err(_e) => {
            eprintln!("Error connecting to server 127.0.0.1:7878");
            eprintln!("Make sure server is running!");
            process::exit(1);
        }
    };

    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let dayTime = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!(">{}", dayTime);
        }
        Err(_e) => {
            eprintln!("Error geting day and time from server stream.");
            process::exit(1);
        }
    };
}
