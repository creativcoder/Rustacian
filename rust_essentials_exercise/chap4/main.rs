use std::io;

// prepend underscores to ignore warnings by compiler

fn _take_slice(s:&'static str) -> String {
	s.to_string()
}

fn _print_len(s: &str) -> usize {
	s.len()
}


fn main() {
	// making a String from &str
	let mut text = "hello".to_string();
	let _input = io::stdin().read_line(&mut text).ok().expect("Couldnt read");


	// making a heap allocated string
	let mut heap_str = String::with_capacity(25);
	heap_str.push('a');
	heap_str.push('b');
	heap_str.push('c');
	heap_str.push('d');
	heap_str.push_str("efg  hijklmnop qrstu   vwxyz");

	// geting single characters of the string using chars()
	for i in heap_str.chars() {
		println!("{}",i);
	}

	// geting single characters of the string using split()
	for i in heap_str.chars(" ") {
		println!("{}",i);
	}

	// replacing in a string (returns a new `String`)

	let new_heap_str = heap_str.replace("abcd","zzzz");
	println!("{:?}",new_heap_str );
	// converting String to slices
	let heap_str_slice = heap_str;
	println!("{:?}",_print_len(&heap_str_slice));

	// array example - can contain only one type
	let my_arr:[&str;4] = ["One","two","three","four"];
	let empty_arr:[i32;0] = [];
	println!("{:?}",my_arr);
	println!("{:?}",my_arr.iter().last());
}