// Refactored previous part01.rs to trait impls and generic types

pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing,
}

// aliasing for Number ADT's
type NumberOrNothing = SomethingOrNothing<i32>;

pub use self::SomethingOrNothing::*;
pub use self::NumberOrNothing::*;



impl<T> SomethingOrNothing<T> {

	fn new(o:Option<T>) -> Self {
		match o {None => Nothing,Some(t) => Something(t) }
	}

	fn to_option(self) -> Option<T> {
		match self{
		Nothing => None,
		Something(t)=>Some(t),
	}
	}
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}


impl NumberOrNothing {

	fn print_min(self){
		match self {
			Nothing => println!("The list is empty"),
			Something(min) => println!("The minimum from the list is {}",min),
		}
	}
}

pub trait Minimum : Copy {
	fn min(self,b:Self) -> Self;
}

pub fn vec_min <T:Minimum> (v:Vec<T>) -> SomethingOrNothing<T> {
	let mut min = Nothing;
	for e in v {
		min = Something(match min{Nothing => e,Something(n)=>e.min(n)})
	}
	min
}


fn read_vec(rang:u32) -> Vec<i32> {
	match rang {
		0 => {vec![]},
		_ => {vec![87,4,7,6,5,9,453]},
	}
}

pub fn main(){
	let vec_list = read_vec(8);
	let min = vec_min(vec_list);

	min.print_min();
}



