use std::env;

fn odd_or_even(val:i32) -> Option<i32> {

	match val {
	x if x % 2 == 0 => Some(x),
	_ => None

}
}

fn read_input1() -> i32 {
	43	
}

fn read_input2() -> i32 {
	66
}

fn main()
{
 // to pass numbers in command line args

let args_vec: Vec<_> = env::args().collect();

//let num = read_input1()	;
//let num2 = read_input2();

match odd_or_even(args_vec[1].parse::<i32>().unwrap()) {
	Some(n) => println!("{} is even",n),
	None => println!("Number is odd"),
};

match odd_or_even(args_vec[2].parse::<i32>().unwrap()) {
	Some(n) => println!("{} is even",n),
	None => println!("Number is odd"),
};

}
