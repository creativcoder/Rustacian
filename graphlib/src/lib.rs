use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq,Copy,Clone,Debug,Hash,Eq)]
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

#[derive(PartialEq,Copy,Clone,Debug,Hash,Eq)]
pub struct Vertex<T> {
    v_id:char,
    data:T,
    state:State,
}

impl<T:Eq> Vertex<T> {
    pub fn new(v_id:char,data:T) -> Self {
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

#[derive(PartialEq,Eq,Hash,Clone)]
pub struct Graph<T> {
    vertices:HashSet<Vertex<T>>,
}


impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: HashSet::new()
        }
    }

    pub fn add_vertex(&mut self,v:Vertex<T>) {
        self.vertices.insert(v);
    }
    pub fn find(&self,v:Vertex<T>) -> Option<&Vertex<T>> {
        self.vertices.take(v)
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
    println!("{:?}",graph);
    assert!(v1 == *graph.find('B').unwrap());
}
