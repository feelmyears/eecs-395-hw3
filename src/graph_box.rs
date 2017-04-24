use std::collections::HashMap;
use std::string::String;

pub struct EdgeMap<T> {
    edges: HashMap<T, HashSet<T>>
}

pub struct Graph<T> {
	nodes: HashSet<T>,
    edges: EdgeMap<T>
}

impl Graph<T> {
    pub fn new() -> Self {
    	Graph {
    		nodes: HashMap::new(),
            edges: EdgeMap::new()
    	}
    }
}

impl EdgeMap<T> {
    pub fn new() -> Self {
        EdgeMap {
            edges: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, a: &T, b: &T) {
        if self.edge_exists(a, b) {
            false
        }

        if let Some(&a_neighbors) = self.edges.get(a) {
            a_neighbors.insert(b);
        } else if let Some(&b_neighbors) = self.edges.get(b) {
            b_neighbors.insert(a);
        } else {
            let &mut a_neighbors = HashSet::new();
            a_neighbors.insert(b);
            self.edges.insert(a.clone(), a_neighbors)
        }

        true
    }

    pub fn edge_exists(self, a: &T, b: &T) -> bool {
        return self.edges.get(a).contains(b);
    }

    pub fn get_neighbors(self, a: &T) -> HashSet<T> {
        
    }
}