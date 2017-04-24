use std::io::BufRead;
use std::string::String;
use std::collections::HashMap;
use std::collections::HashSet;

// ---------------------------------- GRAPH -----------------------------------------
//#[derive(Debug, PartialEq, Eq)]

pub struct graph {
    pub nodes: HashMap<String, Node>,
}

impl graph {
    pub fn new(alist: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for line in alist {
            let l: Vec<&str> = line.as_str().split(" ").collect();
            let mut ns: Vec<String> = vec![];
            for i in 1..l.len() {ns.push(l[i].to_string());}
            let node = Node::new(l[0].to_string(), ns);
            map.entry(l[0].to_string()).or_insert(node);
        }
        graph {
            nodes: map,
        }
    }

    // iterates thru keys of hashmap to make sure all edges are accounted for
    pub fn backtrack(self) {
        for (key, node) in self.nodes {
            for n in node.neighbors {
                &(self.nodes).get(&n).unwrap().neighbors.insert(key.clone());
            }
        }
    }
}

// ---------------------------------- NODE ------------------------------------------

//#[derive(Debug, PartialEq, Eq)]
pub struct Node {
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
}