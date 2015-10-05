// implementing generic traits for vec_min and added trait impls for floats


pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing,
}

pub use self::SomethingOrNothing::*;

// taking aliases for some instances of generic type
type NumberOrNothing = SomethingOrNothing<i32>;
type FloatNumOrNothing = SomethingOrNothing<f32>;

impl<T> SomethingOrNothing<T> {
	fn new(o:Option<T>) -> Self {
		match o {None => Nothing,Some(t) => Something(t) }
	}
	fn to_option(self) -> Option<T> {
		match self { Nothing => None,Something(t) => Some(t) }
	}
}

pub trait Minimum : Copy {
	fn min(self,b:Self) -> Self;
}

pub fn vec_min <T:Minimum>(v:Vec<T>) -> SomethingOrNothing<T> {
	let mut min = Nothing;
	for e in v {
		min = Something(match min {
			Nothing => e,
			Something(n) => e.min(n),
		});
	}
	min
}

impl Minimum for i32 {
	fn min(self,b:Self) -> Self {
		if self < b {self} else {b}
	}
}

impl Minimum for f32 {
	fn min(self,b:Self) -> Self {
		if self < b {self} else {b}
	}
}


impl NumberOrNothing { 
	pub fn print(self) {
		match self {
			Nothing => println!("The nunber is : <nothing>"),
			Something(n) => println!("The minimum no from the vec is {}",n),
		};
	}
}

impl FloatNumOrNothing { 
	pub fn print(self) {
		match self {
			Nothing => println!("The nunber is : <nothing>"),
			Something(n) => println!("The number is {}",n),
		};
	}
}

fn read_vec_int() -> Vec<i32> {
	vec![18,5,7,3,9,27]
}

fn read_vec_float() -> Vec<f32> {
	vec![18.4,5.2,7.35,66.99,9.2,27.23]
}

pub fn main() {
	let vec_int = read_vec_int();
	let vec_float = read_vec_float();
	let min_int = vec_min(vec_int);
	let min_float = vec_min(vec_float);
	min_int.print();
	min_float.print();
}

