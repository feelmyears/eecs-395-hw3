use graph::{graph, Node};

pub fn build_graph(alist: Vec<String>) {
    let mut graph = graph::new(alist);
    graph.backtrack();
}