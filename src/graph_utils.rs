use graph::graph;

pub fn build_graph(alist: Vec<String>) {
    let graph = graph::new(alist);
    graph.print_edges();
}