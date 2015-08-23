#![allow(dead_code)]

// Enums are an Algebric Data Type
enum NumberorNothing {
Number(i32),
Nothing
}

// hoisting helper to avoid boilerplate code
use self::NumberorNothing::{Number,Nothing};

fn vec_min(vec: Vec<i32>) -> NumberorNothing {
	let mut min = NumberorNothing::Nothing;
	for el in vec {
	match min {
		NumberorNothing::Nothing => {min = NumberorNothing::Number(el);},
		NumberorNothing::Number(n) => {let new_min = min_i32(n, el); min = NumberorNothing::Number(new_min);}
	}
	}
return min;
}

// comparision function
fn min_i32(a:i32,b:i32) -> i32 {

if a > b { a }
else { b }

}

fn read_vec() -> Vec<i32> {
	vec![2,5,3,56,7,4]
}

fn print_min_result(n:NumberorNothing) {
	match n {
		Nothing => print!("Error: The vector is Empty"),
		Number(n) => printl!("The minimum element is: {} ",n);
	}
}

// Entry point
fn main(){
	let vec = read_vec();
	let min = vec_min(vec);
	print_min_result(min);
}
