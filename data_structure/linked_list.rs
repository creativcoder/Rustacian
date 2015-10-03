// Linked list implementation using rust semantics

#[derive(Debug)]
enum NextOrNone {
	Next(Box<Node>),
	None,
}

use NextOrNone::{Next,None};

#[derive(Debug)]
struct Node {
	data: u32,
	next: Box<NextOrNone>,
}

impl Node {
	
	fn init(val:u32) -> Node {
		Node {data:val,next:Box::new(None)}
	}

}

fn main() {

	let head = Node::init(23);

}