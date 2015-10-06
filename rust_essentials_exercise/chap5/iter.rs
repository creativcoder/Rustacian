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

	// using iterators and adaptors on them
	let mut rng = 0..100;
	let mut rng2 = 0..100;
	let rng_vec:Vec<i32> = rng.collect::<Vec<i32>>();
	// here filter,map,take are adaptors that
	// manipulate iterators 
	// and finally the last collect method
	// is the consumer that empties the iterator
	// and returns a vector
	let even_rng_vec:Vec<i32> =  rng2.filter(|n| n%2==0)
									 .map(|n| n*n*n)
									 .take(5)
									 .collect();
	println!("{:?}",even_rng_vec );				
	let sum = (0..500).fold(0,|sum,n| sum+n);
	println!("The sum is {:?}",sum );
	// exercise 1 of chapter 5
	let prod_cube_int = (1..6)
						.map(|n| n*n*n)
						.fold(1,|prod_cube_int,n| prod_cube_int*n); 
	println!("{:?}",prod_cube_int );
	// exercise 2 of chapter 6
	let arr = [1,9,2,3,14,12];
	let aux_arr = arr.iter();
	let sum_arr = aux_arr.fold(arr[0],|sum_arr,n| sum_arr+n);
	println!("{:?}",sum_arr );
}