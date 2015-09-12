#![warn(dead_code)]
#![warn(unused_assignments)]

use std::f32::consts;
static RAD : f32 = 3.14;



enum Optional {
	Something(i32),
	Nothing,
}

mod circle_util {

pub fn get_radius() -> Option<f32> {
Some(3.45)
}

pub fn get_minimum(v:Vec<i32>) -> i32 {
	let mut min = 9999;
	for i in v {
		if i<=min {min=i;}
		}
		min
}

}


fn main() {
let rad = ::circle_util::get_radius().unwrap_or(RAD);
println!("The Area of Circle is {:?} (m)^2",consts::PI*rad*rad);

//println!("{:?}",u32::from_str_radix(&"34",10).unwrap());
	let num_arr = vec![24,204,34,32,24,234];

	let access_outer = |x:&Vec<i32>| {println!("{:?}",x[3]);};
	
	//println!("The minimum from the list is  {:?}",::circle_util::get_minimum(num_arr));

	let str1 = "John";  //this is a string slice
	let str2 = "Doe" ;
	// to perform string concatenation
	let str3 = str1.to_string() + str2; //str3 is now a string

	println!("{:?}",str3+"c");


	// depicting truncation
	let flo = 34.999_f64;
	let lost = flo as i32;
	println!("{:?}",lost);

	struct Location<T> {
		x : T,
		y : T,
	}

	impl<T> Location<T> {
		fn get_location_name(self) -> &'static str {
			"Center"
		}
	}

	let l1 = Location {x:0,y:0};
	println!("{:?}",l1.get_location_name());

	/*type d_int = [[u32;4];4];
	let mut GRAPH : d_int = [[0,4,33,3],
								 [3,0,33,3],
								 [3,4,0,3],
								 [3,4,33,0]];
	
	fn add_weight(g:&d_int,u:i32,v:i32,w:i32) {
		g[u][v] = w;
	} 
	add_weight(&GRAPH,2,1,99);
	println!("{:?}",GRAPH[2][1]);*/


	// integer overflow example
	let mut num:u16 = 1000;
	num = 65536;
	println!("{:?}",num);

	// memory addresses example
	let v = vec![1,2,4,5];
	let hex_addr = format!("{:p}",&v[0]);
	println!("{:?}",hex_addr);
	println!("{:p}",&v[1]);

	let y:&f64=&56.;
	println!("{:?}",y );
	access_outer(&num_arr);
}
