pub fn less_verbose(x:i32) ->&'static str {
	if x < 10 {"less than 10"}
	else { "more than 10" }
}
/// Returns the absolute value of an i32 passed to it.
/// # Example
/// ```
/// let num = abs(-32);
/// ```
/// num has the value 32.
pub fn abs(x:i32) -> i32 {
	if x < 0 {-x}
	else {x}
}

fn main() {
	let health = -243;
	let result = if health <=0 {"Game over"} else {"Continue Playing"};
	println!("{:?}",result );
	println!("{:?}",less_verbose(5) );

	 // infinite loops can be labeled by '<label> 
'outer:	loop {
	'inner : loop {
			println!("Inside loop");
			break 'outer;
		}
		println!("The outer loop region - Never Reaches");
	}
	println!("Out of the loop");

	//simplest iterators 
	/*for i in 0..11 {
		print!("{}",i);
	}*/
	println!("{:?}",abs(-3));
}