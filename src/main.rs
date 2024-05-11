use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use cookie::Cookie;

fn main() {    
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

fn handle_client(mut stream: TcpStream) {
    stream.read(&mut [0; 1024]).unwrap();
    
    let mut cookie = Cookie::new("Francesco", "twitch");

    stream.write(b"HTTP/1.1 200 OK\r\n");
    stream.write(b"Set-Cookie: ");
    stream.write(cookie.to_string().as_bytes());
    stream.write(b"\r\n");
    stream.write(b"\r\n");
    stream.write(b"Hello, World!").unwrap();
    stream.flush().unwrap();
}
