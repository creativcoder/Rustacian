
#[derive(Debug)]
struct List {
	data: i32,
	next: Option<Box<List> >,
}

impl List {
	fn insert(&self,d:i32){
		let mut temp = List {data:d,next:None};
		self.next = Some(temp);
	}
}

fn main() {
	let head = List {data:34,next:None};
	println!("head is {:?}",head.data);
}