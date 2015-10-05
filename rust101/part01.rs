// Refactoring part00.rs to using expressions and inherent methods for the NumberOrNothing algebric data type

enum NumberOrNothing {
	Number(i32),
	Nothing,
}

impl NumberOrNothing {
	fn print_min(self){
		match self {
			Nothing => println!("The list is empty"),
			Number(min) => println!("The minimum from the list is {}",min),
		}
	}
	fn print_sum(self){
		match self {
			Nothing => println!("No elements in list"),
			Number(min) => println!("The sum is {}",min),
		}
	}
}

use self::NumberOrNothing::{Number,Nothing};


//fn sqr(a:i32) -> i32 {a*a}
//fn abs(a:i32) -> i32 { if a>=0 {a} else {-a} }

fn vec_min(vec:Vec<i32>) -> NumberOrNothing {

	//min_i32 is a clousure
	let min_i32 = |a,b| { if a > b {b} else {a} };

	let mut min = Nothing;
	for el in vec {

		match min {
			Nothing => {min = Number(el);},
			Number(n) => {let new_min = min_i32(n,el);min = Number(new_min);},
		}

	}
	min
}

// Exercise 01.1 -----------------------------------------------

fn vec_sum(vec:Vec<i32>) -> NumberOrNothing {
	match vec.len() {
		0 => {Nothing}
		_ => {let mut sum = 0; for i in vec {sum+=i;} Number(sum) }
	}
}
//-------------------------------------------------Exercise 01.1



// Exercise 01.2 -----------------------------------------------

fn vec_print(vec:Vec<i32>) {
	for e in vec {
		println!("{}",e);
	}
}
//-------------------------------------------------Exercise 01.2



fn read_vec(rang:u32) -> Vec<i32> {
	match rang {
		0 => {vec![]},
		_ => {vec![87,4,7,6,5,9,453]},
	}
}



pub fn main(){
	let vec_list = read_vec(8);
	let min = vec_min(vec_list);
	vec_sum(vec![5,4,3,4]).print_sum();
	vec_print(vec![8,9,3,5]);
	min.print_min();
}



