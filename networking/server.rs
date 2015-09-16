use std::net::{TcpListener,TcpStream};
use std::thread;

fn main(){

let server =  TcpListener::bind("127.0.0.1:3000").unwrap();

fn handle_client(stream:&TcpStream) {
	println!("Accepted connection from {:?}",stream.local_addr().unwrap());
}
for stream in server.incoming() {
	match stream {
		Ok(stream) => {
			thread::spawn(move|| {
				handle_client(&stream)
			});
		},
		Err(e) => {println!("Connection Error");}
	}
}
drop(server);

}