extern crate graphlib;

use graphlib::Graph;
use graphlib::Vertex;

fn main() {
	let v = Vertex::new('A',"Alice");
	let mut graph = Graph::new();
	graph.add_vertex(v);
	println!("{:?}",v );
}
