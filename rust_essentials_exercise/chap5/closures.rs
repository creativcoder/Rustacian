fn power(x:i32) -> i32 {
	x+10
}

fn five_times<F:Fn(i32)->i32>(f:F,x:i32) -> i32 {
	f(f(f(f(x))))
}

fn main() {
	
	let num = 10;
	// using a higher order function here
	println!("{:?}",five_times(power,num));
	// or we can use a closure here to replace power function
	println!("{:?}",five_times(|n|n+10,num));
}