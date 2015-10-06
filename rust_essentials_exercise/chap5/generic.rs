
struct Pair<T> {
    first: T,
    second: T,
}

fn sqr(n:f64) -> Result<f64,&'static str> {
	match n {
		0.0 => {Err("Bad move")},
		_ => {Ok(n*n)}
	}
}

fn main() {
	let pair_str : Pair<&str> = Pair{first:"Awesome",second:"Rust"};
	let pair_f64 : Pair<f64> = Pair{first:34.2,second:67.13};
	println!("{:?}",sqr(-4.5));
	let x=23;
	let y = 0;
	assert!(y!=0,"You are attempting to divide by zero");
} 
