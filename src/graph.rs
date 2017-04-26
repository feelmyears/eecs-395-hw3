use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::cmp::Eq;
use std::usize::MAX;
use std::iter::FromIterator;

pub struct Graph<T> {
	forward_map: HashMap<T, usize>,
    reverse_map: Vec<T>,
    edges: Vec<HashSet<usize>>
}
pub type Path<T> = Result<Option<Vec<T>>, &'static str>;

impl <T: Hash + Eq + Clone> Graph<T> {
    pub fn new() -> Self {
    	Graph {
    		forward_map: HashMap::new(),
            reverse_map: vec![],
            edges: vec![],
    	}
    }

    pub fn add_node(&mut self, node: T) {
        if !self.forward_map.contains_key(&node) {
            self.forward_map.insert(node.clone(), self.reverse_map.len());
            self.reverse_map.push(node);
            self.edges.push(HashSet::new());
        }
    }

    pub fn add_edge(&mut self, node1: &T, node2: &T) {
        let node1_index = self.get_node_index(node1);
        let node2_index = self.get_node_index(node2);

        self.edges[node1_index].insert(node2_index);
        self.edges[node2_index].insert(node1_index);
    }

    pub fn contains_node(&self, node: &T) -> bool {
        self.forward_map.contains_key(node)
    }

    pub fn contains_edge(&self, node1: &T, node2: &T) -> bool {
        self.edges[self.get_node_index(node1)].contains(&self.get_node_index(node2))
    }

    pub fn shortest_path(&self, start: &T, finish: &T) -> Path<T> where T: FromIterator<T> {
        if !(self.contains_node(start) && self.contains_node(finish)) {
            return Err("Invalid node choice. At least one node does not exist.");
        } else if !self.path_exists(start, finish) {
            return Ok(None);
        }

        let start_index = self.get_node_index(start);
        let finish_index = self.get_node_index(finish);

        let index_path = self.shortest_index_path(start_index, finish_index);
        match index_path {
            Some(path) => Ok(Some(path.iter().map(|&i| self.get_node_name(i)).collect())),
            None => Ok(None),
        }
    }
    
    fn path_exists(&self, start: &T, finish: &T) -> bool {
        return self.path_exists_recur(self.get_node_index(start), self.get_node_index(finish), HashSet::new());
    }

    fn path_exists_recur(&self, start: usize, finish: usize, visited: HashSet<usize>) -> bool {
        if start == finish {
            return true;
        } else {
            let mut path_exists = false;
            for &neighbor in &self.edges[start] {
                if !visited.contains(&neighbor) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(neighbor);
                    path_exists = path_exists || self.path_exists_recur(neighbor, finish, new_visited);
                }
            }
            return path_exists;
        }
    }

    fn shortest_index_path(&self, start: usize, finish: usize) -> Option<Vec<usize>> {
        let num_nodes = self.reverse_map.len();
        let mut ancestors: Vec<Option<usize>> = Vec::with_capacity(num_nodes);
        let mut distances: Vec<usize> = Vec::with_capacity(num_nodes);
        let mut remaining: HashSet<usize> = HashSet::with_capacity(num_nodes);

        for i in 0 .. num_nodes {
            ancestors.push(None);
            distances.push(MAX);
            remaining.insert(i);
        }

        distances[start] = 0;
        while !remaining.is_empty() {
            let n = self.get_next_node_index(&distances, &remaining);
            remaining.remove(&n);

            if n == finish {
                return Some(self.construct_index_path(finish, &ancestors));
            }

            for &neighbor in &self.edges[n] {
                if remaining.contains(&neighbor) {
                    let new_dist: usize = distances[n] + 1;
                    if new_dist < distances[neighbor] {
                        distances[neighbor] = new_dist;
                        ancestors[neighbor] = Some(n);
                    }
                }
            }
        }

        None
    }

    fn get_next_node_index(&self, distances: &Vec<usize>, remaining: &HashSet<usize>) -> usize {
        let mut min_dist: usize = MAX;
        let mut min_node: usize = 0;

        for &node in remaining {
            if distances[node] <= min_dist {
                min_dist = distances[node];
                min_node = node;
            }
        }

        return min_node;
    }

    fn construct_index_path(&self, finish: usize, ancestors: &Vec<Option<usize>>) -> Vec<usize> {
        let mut path: Vec<usize> = vec![];
        let mut node = finish;

        while let Some(ancestor) = ancestors[node] {
            path.push(node);
            node = ancestor;
        }

        path.push(node);
        path.reverse();
        return path;
    }

    fn get_node_index(&self, node: &T) -> usize {
        self.forward_map[node]
    }

    fn get_node_name(&self, index: usize) -> T {
        self.reverse_map[index].clone()
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;

    #[test]
    fn add_node_test() {
        let node_name = "Test".to_string();
        let mut graph = Graph::new();

        assert!(!graph.forward_map.contains_key(&node_name));
        assert_eq!(0, graph.reverse_map.len());

        graph.add_node(node_name.clone());

        assert!(graph.forward_map.contains_key(&node_name));
        assert_eq!(0, graph.forward_map[&node_name]);
        assert_eq!(graph.reverse_map[0], node_name);
    }

    #[test]
    fn add_edge_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(0, graph.forward_map[&node1_name]);
        assert_eq!(1, graph.forward_map[&node2_name]);

        graph.add_edge(&node1_name, &node2_name);

        assert!(graph.edges[0].contains(&1));
        assert!(graph.edges[1].contains(&0));
    }

    #[test]
    fn get_node_index_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(0, graph.get_node_index(&node1_name));
        assert_eq!(1, graph.get_node_index(&node2_name));
    }

    #[test]
    fn get_node_name_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(node1_name, graph.get_node_name(0));
        assert_eq!(node2_name, graph.get_node_name(1));
    }

    #[test]
    fn contains_node_test() {
        let node_name = "Test1".to_string();
        let mut graph = Graph::new();

        graph.add_node(node_name.clone());

        assert!(graph.contains_node(&node_name));
    }

    #[test]
    fn contains_edge_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(0, graph.forward_map[&node1_name]);
        assert_eq!(1, graph.forward_map[&node2_name]);

        graph.add_edge(&node1_name, &node2_name);

        assert!(graph.contains_edge(&node1_name, &node2_name));
        assert!(graph.contains_edge(&node2_name, &node1_name));
    }

    #[test]
    fn connected_shortest_path_test() {
        let nodes = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let edges = vec![
            ("a".to_string(), "b".to_string()),
            ("a".to_string(), "d".to_string()),
            ("b".to_string(), "c".to_string()),
            ("c".to_string(), "e".to_string()),
            ("d".to_string(), "e".to_string()),
        ];

        let paths = vec![
            (vec!["a".to_string(), "b".to_string()], Some(vec!["a".to_string(), "b".to_string()])),
            (vec!["a".to_string(), "c".to_string()], Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])),
            (vec!["e".to_string(), "a".to_string()], Some(vec!["e".to_string(), "d".to_string(), "a".to_string()])),
            (vec!["c".to_string(), "d".to_string()], Some(vec!["c".to_string(), "e".to_string(), "d".to_string()])),
        ];

        let graph = construct_graph(nodes, edges);
        assert_shortest_paths(graph, paths);
    }

    #[test]
    fn disjoint_shortest_path_test() {
        let nodes = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let edges = vec![
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "c".to_string()),
            ("d".to_string(), "e".to_string()),
        ];

        let paths = vec![
            (vec!["a".to_string(), "e".to_string()], None),
            (vec!["c".to_string(), "d".to_string()], None),
            (vec!["d".to_string(), "b".to_string()], None),
        ];

        let graph = construct_graph(nodes, edges);
        assert_shortest_paths(graph, paths);
    }

    #[test]
    fn path_exists_tests() {
        let nodes = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let edges = vec![
            ("a".to_string(), "b".to_string()),
            ("a".to_string(), "c".to_string()),
            ("d".to_string(), "e".to_string()),
        ];

        let paths = vec![
            (vec!["a".to_string(), "b".to_string()], true),
            (vec!["c".to_string(), "b".to_string()], true),
            (vec!["c".to_string(), "a".to_string()], true),
            (vec!["d".to_string(), "e".to_string()], true),
            (vec!["a".to_string(), "d".to_string()], false),
            (vec!["e".to_string(), "c".to_string()], false),
            // (vec!["d".to_string(), "b".to_string()], None),
        ];

        let graph = construct_graph(nodes, edges);
        for (input, expected) in paths {
            assert_eq!(graph.path_exists(&input[0], &input[1]), expected);
        }
    }

    fn construct_graph(nodes: Vec<String>, edges: Vec<(String, String)>) -> Graph<String> {
        let mut graph: Graph<String> = Graph::new();

        for n in nodes {
            graph.add_node(n);
        }

        for (n1, n2) in edges {
            graph.add_edge(&n1, &n2);
        }

        return graph;
    }

    fn assert_shortest_paths(graph: Graph<String>, paths: Vec<(Vec<String>, Option<Vec<String>>)>) {
        for (input, expected) in paths {
            assert_eq!(graph.shortest_path(&input[0], &input[1]).unwrap(), expected);
        }
    }
}