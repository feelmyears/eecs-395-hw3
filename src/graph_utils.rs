use graph::graph;

pub fn build_graph(alist: Vec<String>) {
    let graph = graph::new(alist);
    graph.print_edges();
}

#[cfg(test)]
mod build_test {
    use graph::graph;

    #[test]
    fn check_num_edges() {
        let mut alist: Vec<String> = vec![];
        alist.push("a b d".to_string());
        alist.push("b a d".to_string());
        alist.push("c".to_string());
        alist.push("d c".to_string());
        let graph = graph::new(alist);
        let mut edges = 0;
        for node in graph.nodes.values() {
            edges += node.len();
        }
        assert_eq!(8, edges); // undirected graph, must account for reverse edges
    }
}