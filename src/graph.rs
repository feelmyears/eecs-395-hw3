use std::string::String;
use std::collections::HashMap;
use std::collections::HashSet;

// ---------------------------------- GRAPH -----------------------------------------
pub struct graph {
    pub nodes: HashMap<String, HashSet<String>>
}

impl graph {
    pub fn new(alist: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for line in alist {
            let l: Vec<&str> = line.as_str().split(" ").collect();
            let mut ns = HashSet::new();
            for i in 1..l.len() {ns.insert(l[i].to_string());}
            // before adding key, makes sure key is in neighbor set of all neighbors
            // if neighbors don't exist, add with set containing key
            for n in &ns {
                let mut newset = HashSet::new();
                newset.insert(l[0].to_string());
                map.entry(n.clone()).or_insert(newset).insert(l[0].to_string());
            }
            for n in &ns {
                map.entry(l[0].to_string()).or_insert(ns.clone()).insert(n.to_string());
            }
            
        }
        graph {
            nodes: map,
        }
    }

    // prints each edge in an arbitrary order
    pub fn print_edges(self) {
        println!("Result: ");
        for (key, node) in self.nodes {
            for n in node {
                println!("{} -> {}", key, n);
            }
        }
    }
}

// ---------------------------------- NODE ------------------------------------------

//#[derive(Debug, PartialEq, Eq)]
/*pub struct Node {
    value: String,
    edges: usize,
    pub neighbors: HashSet<String>,
}

impl Node {

    fn new(val: String, ns: Vec<String>) -> Self {
        let mut set = HashSet::new();
        for n in ns {
            set.insert(n);
        }
        Node {
            value: val,
            edges: set.len(),
            neighbors: set,
        }
    }
}*/