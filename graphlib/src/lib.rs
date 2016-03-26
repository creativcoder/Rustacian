use std::collections::HashMap;

#[derive(PartialEq,Copy,Clone)]
enum State {
    Opaque,
    Visited,
    Explored,
}

pub trait Traversal {

    fn bfs(&self) {

    }
    fn dfs(&self) {
        
    }
}

/*pub mod AlphaBetaPrune {
    use ::Traversal;
    fn run_through<T:Traversal>(graph:&T) {

    }
}*/

impl<T> Traversal for Graph<T> {
    fn bfs(&self) {

    }
    fn dfs(&self) {

    }
}

use State::*;

#[derive(PartialEq,Copy,Clone)]
pub struct Vertex<T> {
    v_id:char,
    data:T,
    state:State,
}

impl<T> Vertex<T> {
    fn new(v_id:char,data:T) -> Self {
        let data = data;
        Vertex {
            v_id:v_id,
            data:data,
            state:Opaque
        }
    }
    fn add_neighbour(&self) {

    }
}

type Id = char;

pub struct Graph<T> {
    vertices:HashMap<Id,Vertex<T>>,
}


impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self,v:Vertex<T>) {
        self.vertices.insert(v.v_id.clone(),v);
    }
    pub fn find(&self,graph_id:char) -> Option<&Vertex<T>> {
        self.vertices.get(&graph_id)
    }
}
#[test]
fn graph_init() {
    let mut graph = Graph::new();
    let v1 = Vertex::new('B',"Bob");
    let v2 = Vertex::new('A',"Alice");
    let v3 = Vertex::new('S',"Sam");
    let v4 = Vertex::new('D',"Derek");
    graph.add_vertex(v1);
    graph.add_vertex(v2);
    graph.add_vertex(v3);
    graph.add_vertex(v4);
    assert!(v1 == *graph.find('B').unwrap());
}
