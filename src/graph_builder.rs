use std::string::String;
use std::io::{BufRead, BufReader, Read};

extern crate regex;
use self::regex::Regex;

use graph_box::Graph;

pub fn build_graph<R: Read>(reader: R) -> Graph<String> {
	let mut graph: Graph<String> = Graph::new();
	let mut lines = BufReader::new(reader).lines();
	let re = Regex::new(r"[\w]+").unwrap();

    while let Some(Ok(line)) = lines.next() {
    	let mut node: Option<String> = None;
    	for caps in re.captures_iter(&line) {
			for m in caps.iter() {
				let tok = m.unwrap().as_str().to_string(); 
				match node {
					Some(n) => {
						graph.add_node(tok.clone());
						graph.add_edge(&tok, &n);
						node = Some(n);
					}
					None 			=> {
						node = Some(tok.clone());
						graph.add_node(tok);
					}
				}
			}			
		}	
    }

	return graph;
}

#[cfg(test)]
mod build_graph_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn build_graph_test() {
    	let input = "a b d\nb a d\nc\nd c".to_string();
    	let reader = Cursor::new(input);
    	let graph = build_graph(reader);

    	let nodes = vec![
    		"a".to_string(),
    		"b".to_string(),
    		"c".to_string(),
    		"d".to_string()
    	];

    	for n in nodes {
    		assert!(graph.contains_node(&n));
    	}

    	let edges = vec![
    		("a".to_string(), "b".to_string()),
    		("a".to_string(), "d".to_string()),
    		("b".to_string(), "a".to_string()),
    		("b".to_string(), "d".to_string()),
    		("d".to_string(), "c".to_string())
    	];

    	for (n1, n2) in edges {
    		assert!(graph.contains_edge(&n1, &n2));
    	}
    }
}