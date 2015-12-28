enum State {
    WHITE,
    GRAY,
    BLACK,
}

pub trait Traversal {
    fn bfs(&self) {

    }
    fn dfs(&self) {
        
    }
}

impl Traversal for Graph {
    fn bfs(&self) {

    }
    fn dfs(&self) {

    }
}

use State::*;

pub struct Vertex {
    v_id:u32,
    data:String,
    state:State,
    in_degree:u64,
    out_degree:u64,
}

impl Vertex {
    fn new(v_id:u32,data:&str) -> Self {
        let data = data.to_string();
        Vertex {v_id:v_id,data:data,state:WHITE,in_degree:0,out_degree:0}
    }
}
 
pub struct Graph {
    v_size:u64,
    v_vec:Vec<Vertex>,
    e_vec:Vec<(Vertex,Vertex)>,
}

impl Graph {
    pub fn new(nodes:u64) -> Self {
        Graph {v_size:0,v_vec:vec![],e_vec:vec![]}
    }

    pub fn add_vertex(&mut self,v:Vertex) {
        self.v_vec.push(v);
    }
}
#[test]
fn graph_init() {
    let mut graph = Graph::new(12);
    let v1 = Vertex::new(0,"Bob");
    let v2 = Vertex::new(1,"Alice");
    let v3 = Vertex::new(2,"Sam");
    let v4 = Vertex::new(3,"Derek");
    graph.add_vertex(v1);
    graph.add_vertex(v2);
    graph.add_vertex(v3);
    graph.add_vertex(v4);
}
