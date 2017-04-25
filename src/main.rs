//! RUST GRAPH
//! Homework 3
//!
//! This program takes an association list as input, as well as  
//! a start and end point on the graph from user input.
//! By constructing a specialized graph struct, the program searches
//! for a path between the two identified points, then requests another
//! pair. The program terminates when the user inputs a blank line or 
//! 999.
//!

use std::io::{BufRead,BufReader,Read};
use std::env;
use std::fs::File;

mod graph;

mod graph_utils;
use graph_utils::{build_graph, search_graph};

fn main() {
    let graph_reader = get_graph_reader();
    let alist = read_graph(graph_reader);
    
    let graph = build_graph(alist);
    search_graph(&graph);
}

fn get_graph_reader() -> BufReader<File> {
	let args: Vec<String> = env::args().collect();
	let ref path = args[1];

	let file = File::open(path).unwrap();
	let buf_reader = BufReader::new(file);
	return buf_reader;
}

fn read_graph<R: Read>(reader: R) -> Vec<String> {
    let mut alist: Vec<String> = vec![];
    for line in BufReader::new(reader).lines() {
	    let line = match line {
	        Ok(line) => line,
	        Err(err) => panic!("Failed to read line: {}", err),
	    };
        alist.push(line.trim().to_string());
    }
    return alist;
}

// ---------------------------------- TESTS -----------------------------------------

#[cfg(test)] 
mod input_tests {
    use super::read_graph;
    use std::io::Cursor;
    
    #[test]    
    fn read_multi_lines() {
        let mut expected: Vec<String> = vec![];
        expected.push("a b d".to_string());
        expected.push("b a d".to_string());
        expected.push("c".to_string());
        expected.push("d c".to_string());
	    assert_read(expected, "a b d\nb a d\nc\nd c\n".to_string());
    }
    #[test]
    fn return_nothing() {
        let expected: Vec<String> = vec![];
	    assert_read(expected, "\n".to_string());
    }
    fn assert_read(expected: Vec<String>, input: String) {
	    let mock_read = Cursor::new(input);
	    let result: Vec<String> = read_graph(mock_read);
        for i in 0..result.len()-1 {
	        assert_eq!(expected[i].as_bytes(), result[i].as_bytes());
        }
    }
}