fn main() {
	// read inuts from command line
	let mut a = std::env::args().nth(1).unwrap().parse::<u64>().unwrap();
	let mut b = std::env::args().nth(2).unwrap().parse::<u64>().unwrap();
	let mut r = a%b;
	let mut temp = a;
	loop {
		match r {
			0 => {println!("GCD is {}",b );break;},
			_ => {temp=a;a=b;b=temp%b;r=a%b;},
		}
	}
}
