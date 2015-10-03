fn again(F:<Fn(i32)->i32 >,num)
{
	F(num)
}

fn main() {
	let triples = |x| x*3;
	let num = 5;
	println!("{:?}",triples(5));
}

