mod part02;
use std::io::prelude::*;
use std::io;

fn read_vec() -> Vec<i32> {
let mut vec : Vec<i32> = Vec::<i32>::new();
let stdin = io::stdin();
println!("Enter some numbers. One per line \nEnd with Ctrl-D(*nix) or Ctrl-Z(win32)\n");

for line in stdin.lock().lines() {
	let line = line.unwrap();
	match line.trim().parse::<i32>() {
		Ok(num) => { vec.push(num); },
		Err(e) => {println!("{}",e);break;},
	}
}
println!("\nSo i got a list of {} numbers here !\n",vec.len());
vec
}

use part02::{SomethingOrNothing,Something,Nothing,vec_min};


pub fn main() {
	let vec = read_vec();
	let min = vec_min(vec);
	min.print();
}

trait Print {

}

impl<T:Print> SomethingOrNothing<T> {
	fn print2(self) {
		unimplemented!()
	}
}