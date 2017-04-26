use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::cmp::Eq;
use std::usize::MAX;

pub struct Graph<T> {
	forwardMap: HashMap<T, usize>,
    reverseMap: Vec<T>,
    edges: Vec<HashSet<usize>>
}

impl <T: Hash + Eq + Clone> Graph<T> {
    pub fn new() -> Self {
    	Graph {
    		forwardMap: HashMap::new(),
            reverseMap: vec![],
            edges: vec![],
    	}
    }

    pub fn add_node(&mut self, node: T) {
        if !self.forwardMap.contains_key(&node) {
            self.forwardMap.insert(node.clone(), self.reverseMap.len());
            self.reverseMap.push(node);
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
        self.forwardMap.contains_key(node)
    }

    pub fn contains_edge(&self, node1: &T, node2: &T) -> bool {
        self.edges[self.get_node_index(node1)].contains(&self.get_node_index(node2))
    }

    pub fn shortest_path(&self, start: &T, finish: &T) -> Option<Vec<T>> {
        let start_index = self.get_node_index(start);
        let finish_index = self.get_node_index(finish);

        let index_path = self.shortest_index_path(start_index, finish_index);
        match index_path {
            Some(path) => Some(path.iter().map(|&i| self.get_node_name(i)).collect()),
            None => None,
        }
    }

    fn shortest_index_path(&self, start: usize, finish: usize) -> Option<Vec<usize>> {
        let num_nodes = self.reverseMap.len();
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
                return Some(self.construct_index_path(start, finish, &ancestors));
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

    fn construct_index_path(&self, start: usize, finish: usize, ancestors: &Vec<Option<usize>>) -> Vec<usize> {
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
        self.forwardMap[node]
    }

    fn get_node_name(&self, index: usize) -> T {
        self.reverseMap[index].clone()
    }
}

#[cfg(test)]
mod GraphTests {
    use super::*;

    #[test]
    fn add_node_test() {
        let node_name = "Test".to_string();
        let mut graph = Graph::new();

        assert!(!graph.forwardMap.contains_key(&node_name));
        assert_eq!(0, graph.reverseMap.len());

        graph.add_node(node_name.clone());

        assert!(graph.forwardMap.contains_key(&node_name));
        assert_eq!(0, graph.forwardMap[&node_name]);
        assert_eq!(graph.reverseMap[0], node_name);
    }

    #[test]
    fn add_edge_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(0, graph.forwardMap[&node1_name]);
        assert_eq!(1, graph.forwardMap[&node2_name]);

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

    fn contains_edge_test() {
        let node1_name = "Test1".to_string();
        let node2_name = "Test2".to_string();
        let mut graph = Graph::new();

        graph.add_node(node1_name.clone());
        graph.add_node(node2_name.clone());

        assert_eq!(0, graph.forwardMap[&node1_name]);
        assert_eq!(1, graph.forwardMap[&node2_name]);

        graph.add_edge(&node1_name, &node2_name);

        assert!(graph.contains_edge(&node1_name, &node2_name));
        assert!(graph.contains_edge(&node2_name, &node1_name));
    }
}