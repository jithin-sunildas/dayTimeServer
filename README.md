# The Day Time Server is ready here!

Taking all the experience from the previous echo server, wrote another protocol in Rust. The day time server(RFC 867). Basically, it's a server that just listens for connections and sends the current date and time *once* to whoever connects, and a client to grab that time.
## What's Inside?

* A basic **TCP Daytime Server** that listens on a port and sends the current date and time to each connecting client. It handles multiple clients concurrently using threads.
* A simple **TCP Daytime Client** that connects to the server, receives the date and time, prints it, and disconnects.

## Getting Started

You'll need **Rust and Cargo** installed. If its not installed then the best way is to use "rustup". Follow the instructions from the official website.

1.  **Clone this repository:**
    ```bash
    git clone <URL_of_your_github_repo>
    cd <your_repo_folder_name>
    ```
    (Replace `<URL_of_your_github_repo>` and `<your_repo_folder_name>` with the actual details!)

2.  **Build the project:**
    Since this is a Cargo workspace with two binaries (server and client), you can build both with one command from the root directory:
    ```bash
    cargo build --workspace
    ```
    This will create the executable files in the `target/debug/` directory.

## How to Run

You'll need two separate terminal windows for this!

1.  **Start the Server:**
    In the first terminal, run the server executable. It will bind to `127.0.0.1:7878` and start listening.
    ```bash
    cargo run --package server
    ```
    (Or run the executable directly: `./target/debug/server`)
    You should see output like `Daytime server listening on 127.0.0.1:7878`. Keep this terminal open!

2.  **Start the Client:**
    In the second terminal, run the client executable. It will try to connect to the server you just started.
    ```bash
    cargo run --package client
    ```
    (Or run the executable directly: `./target/debug/client`)
    If the connection is successful, you'll see a message like `Successfully connected...` and then the date and time printed! The client will then exit.

## How it Works (Briefly)

* The **Server** uses `std::net::TcpListener` to `bind` to an address and port and `listen` for incoming connections.
* It uses `listener.incoming()` in a loop to `accept` new connections, spawning a new `std::thread` using `thread::spawn` for each.
* The `handle_client` function running in each thread uses the `chrono` crate to get the current time, formats it into a string, and uses `stream.write()` to send it to the client *once*. The connection is automatically closed when the function finishes.
* The **Client** uses `std::net::TcpStream::connect` to initiate a connection.
* It then uses `stream.read()` *once* to receive the data sent by the server, converts the bytes to a string, and prints it before exiting.

Served as a revision after the echo server project. Learned more about the error handling.
Feel free to clone and build from this though.
