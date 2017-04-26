use std::string::String;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize::MAX;

// ---------------------------------- GRAPH -----------------------------------------
pub struct graph {
    pub nodes: HashMap<String, HashSet<String>>
}

type Path = Vec<String>;

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

    pub fn find_path(&self, start: String, finish: String) -> Option<Path> {
        if !(self.nodes.contains_key(&start) && self.nodes.contains_key(&finish)) {
            None
        } else if start == finish {
            Some(vec![start.clone()])
        } else {
            self.shortest_path(start.clone(), finish.clone())
        }
    }

    fn shortest_path(&self, start: String, finish: String) -> Option<Path> {
        let mut ancestors: HashMap<String, Option<String>> = HashMap::new();
        let mut distances: HashMap<String, usize> = HashMap::new();
        let mut remaining: HashSet<String> = HashSet::new();

        for node in self.nodes.keys() {
            ancestors.insert(node.clone(), None);
            distances.insert(node.clone(), MAX);
            remaining.insert(node.clone());
        }

        *distances.entry(start.clone()).or_insert(0) = 0;
        
        while !remaining.is_empty() {
            let next_node = self.get_next_node(&remaining, &distances);
            
            if next_node == finish {
                return Some(self.construct_path(start.clone(), finish.clone(), &ancestors));
            }
            
            remaining.remove(&next_node);
            let ref neighbors = self.nodes[&next_node];
            for neighbor in neighbors {
                if remaining.contains(neighbor) {
                    let new_dist: usize = distances[&next_node] + 1;
                    if new_dist < distances[neighbor] {
                        if let Some(d) = distances.get_mut(neighbor) {
							*d = new_dist;	
						}

						if let Some(a) = ancestors.get_mut(neighbor) {
							*a = Some(next_node.clone());
						}	
                    }
                }		
            }
            
        }

        None
    }

    fn get_next_node(&self, remaining: &HashSet<String>, distances: &HashMap<String, usize>) -> String {
        let mut min_dist: usize = MAX;
        let mut min_node: Option<&str> = None;

        for node in remaining {
            if distances[node] <= min_dist {
                min_dist = distances[node];
                min_node = Some(node);
            }
        }

        return min_node.unwrap().to_string();
    }

    fn construct_path(&self, start: String, finish: String, ancestors: &HashMap<String, Option<String>>) -> Path {
        let mut path = Path::new();
        let mut curr = finish;

        while let Some(ref a) = ancestors[&curr] {
            path.push(curr.to_string());
            curr = a.clone();
        }
        path.push(start.clone());
        path.reverse();
        return path;
    }
}