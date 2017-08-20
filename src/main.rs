# listens for tcpipv6
pub sturct AddrParseError(_);
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
	// ...
}

let listener = TcpListener::bind("127.0.0.1:80").unwrap();

for stream in listener.incoming() {
	handle_client(stream?};
}

