use std::io::prelude::*;
use std::net::TcpStream;
fn main() {

let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
let _ = stream.write(&[1]);
let _ = stream.read(&mut [0;128]);

}
