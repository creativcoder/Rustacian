use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn write(data:&mut String)  {
	let mut buffer = File::create("foo.txt").unwrap();
	buffer.write(b"as bytese");
}

fn main() {
	let mut buffer = File::create("foo.txt").unwrap();
	let f = File::open("paragraph.txt").unwrap();
	let mut reader = BufReader::new(&f);
   	let line = &mut String::new();
   	reader.read_line(line);
   	while(line.trim()!="") {
   		println!("{:?}",line.trim());
   		line.clear();	
   		reader.read_line(line);
   	}
}
