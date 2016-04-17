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
	next: NextOrNone,
}

impl Node {
	fn new(data: u32) -> Self {
		Node { data:data, next: None }
	}
}

struct List {
	head: NextOrNone
}

impl List {
	fn new(data: u32) -> Self {
		List { head: Next(Box::new(Node::new(data))) }
	}
}

fn main() {
	let linked_list = List::new(23);
}