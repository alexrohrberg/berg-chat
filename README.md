# berg-chat
Multithreaded P2P Chat in Rust

# Multithreaded P2P Chat in Rust

Welcome to the Multithreaded P2P Chat project in Rust! This project aims to provide a peer-to-peer chat application utilizing multiple threads and a command-line interface (CLI). With this chat application, users can communicate with each other in a distributed network environment.

## Prerequisites

Before running this application, ensure that you have the following prerequisites installed:

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)

## Installation

To install and run the Multithreaded P2P Chat, follow these steps:

1. Clone the repository to your local machine:

   ```shell
   git clone https://github.com/alexrohrberg/berg-chat.git
   ```

2. Navigate to the project's directory:

   ```shell
   cd your-repo
   ```

3. Build the project using `cargo`:

   ```shell
   cargo build
   ```

## Usage

This command will compile and run the application. Make sure you have an active internet connection to connect with other peers.
```shell
cargo run
```

By simply running this app, you have access to messsage recieving and sending capability.
The app will prompt you to enter a peers IP address and port.
This could be improved upon in later interations by adding a P2P server to handle the connection before transferring it to peers. The current design is merely a proof of concept. 
A client can recieve messages from multiple peers on seperate threads but support for replying to multiple clients in the same session has not yet been implemented. 

## Testing on Local Host

By default the app starts a server on localhost:8081 to listen to incoming messages.
To test this app with 2 peers, first start up the 1st peer.

```shell
cargo run
```

Then change the port in server.rs to 8080 (or some other port), save server.rs, switch terminal tabs and run cargo run once more to startup a second peer.

```shell
cargo run
```

Then, you can point the 2 peers at each other by entering the ip 127.0.0.1 and 8080/8081 respectively to point the peers at each other. 

## Features

The Multithreaded P2P Chat offers the following features:

- **Peer-to-Peer Communication:** Connect with other peers in a decentralized manner.
- **Multithreaded:** Utilize multiple threads to handle simultaneous connections and messages efficiently.
- **Command-Line Interface (CLI):** Interact with the chat application through a user-friendly command-line interface.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to modify and distribute this project in accordance with the terms specified in the license.

## Thank you!

Thank you for using the Multithreaded P2P Chat in Rust! I hope you have a great chatting experience!