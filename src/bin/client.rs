// client.rs
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    // connect to 4000
    let mut stream = TcpStream::connect("127.0.0.1:4000").unwrap();

    // send a message
    stream.write(b"Hello, world!").unwrap();

    // read the response
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("{}", str::from_utf8(&buffer).unwrap());
}
