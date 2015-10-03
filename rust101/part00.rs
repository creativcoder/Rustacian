// Finding Minimum from a list of numbers using idiomatic rust code

enum NumberOrNothing {
	Number(i32),
	Nothing,
}

use self::NumberOrNothing::{Number,Nothing};

fn vec_min(vec:Vec<i32>) -> NumberOrNothing {
	let mut min = Nothing;
	for el in vec {

		match min {
			Nothing => {min = Number(el);},
			Number(n) => {let new_min = min_i32(n,el);min = Number(new_min);},
		}

	}
	min
}

fn min_i32(a:i32,b:i32) -> i32 {
	if a > b {b} else {a}
}

fn read_vec(rang:u32) -> Vec<i32> {
	match rang {
		0 => {vec![]},
		_ => {vec![87,4,7,6,5,9,453]},
	}
}


pub fn main(){
	let vec_list = read_vec(0);
	let min = vec_min(vec_list);
	match min {
		Number(n) => {println!("The minimum of the list is: {}",n);},
		Nothing => {println!("No minimum Element, the list is empty");},
	}
}