use std::io::{BufRead, BufReader, Read};
use std::string::String;
use std::collections::HashMap;

extern crate regex;
use self::regex::Regex;

mod graph;
use graph::*;

pub fn read_input<R: Read>(reader: R) -> Vec<String> {
	let mut nodes = HashMap<String, Box<Node<String>>>;
	let mut lines = BufReader::new(reader).lines();
	let re = Regex::new(r"[\w']+").unwrap();

	while let Some(Ok(line)) = lines.next() {
    	for caps in re.captures_iter(&line) {
			for m in caps.iter() {
				let word = m.unwrap().as_str().to_lowercase(); 
				if !nodes.contains_key(&word) {
					let new_node = Node::new(word.clone());
					nodes.insert(word, Box::new(new_node));
				}
			}			
		}	
    }

}
