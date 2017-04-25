use graph::graph;
use std::io::{stdin, BufRead};

type Path = Vec<String>;

pub fn build_graph(alist: Vec<String>) -> graph {
    let graph = graph::new(alist);
    return graph;
}

pub fn search_graph(graph: graph) {
    //graph.print_edges();
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" || line == "" {break;}
        let l: Vec<&str> = line.as_str().split(" ").collect(); // l[0] = start, l[1] = end
        // search and print
        // let path = fn returns Option<Path>
        //print_result(path);
    }
}

fn print_result(path: Option<Path>) {
    if path.is_none() {
        println!("There is no path between these values;");
    }
    else {
        for node in path.unwrap() {
            print!("{} ", node);
        }
        println!("");
    }
}

// ---------------------------------- TESTS -----------------------------------------

#[cfg(test)]
mod build_tests {
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
        // undirected graph, must account for reverse edges
        // i.e. d -> a is not input, but a -> d is, so there should be 2 edges
        assert_eq!(8, edges); 
    }

    #[test]
    fn check_num_edges2() {
        let mut alist: Vec<String> = vec![];
        alist.push("a b d".to_string());
        alist.push("b a d".to_string());
        alist.push("c b".to_string());
        alist.push("d c".to_string());
        let graph = graph::new(alist);
        let mut edges = 0;
        for node in graph.nodes.values() {
            edges += node.len();
        }
        // undirected graph, must account for reverse edges
        // i.e. d -> a is not input, but a -> d is, so there should be 2 edges
        assert_eq!(10, edges); 
    }

    #[test]
    fn check_neighbors() {
        let mut alist: Vec<String> = vec![];
        alist.push("a b d".to_string());
        alist.push("b a d".to_string());
        alist.push("c".to_string());
        alist.push("d c".to_string());
        let graph = graph::new(alist);

        for (key, val) in graph.nodes.iter() {
            if key == "a" {assert_eq!(val.contains("b"), true);}
            if key == "a" {assert_eq!(val.contains("d"), true);}
            if key == "a" {assert_eq!(val.contains("c"), false);}

            if key == "b" {assert_eq!(val.contains("a"), true);}
            if key == "b" {assert_eq!(val.contains("c"), false);}
            if key == "b" {assert_eq!(val.contains("d"), true);}

            if key == "c" {assert_eq!(val.contains("a"), false);}
            if key == "c" {assert_eq!(val.contains("b"), false);}
            if key == "c" {assert_eq!(val.contains("d"), true);}

            if key == "d" {assert_eq!(val.contains("a"), true);}
            if key == "d" {assert_eq!(val.contains("b"), true);}
            if key == "d" {assert_eq!(val.contains("c"), true);}
        }
    }
}

#[cfg(test)]
mod search_tests {

}