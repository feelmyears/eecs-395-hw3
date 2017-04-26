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

use std::io::{stdin, BufRead, BufReader, Read};
use std::env;
use std::fs::File;
use std::string::String;

mod graph;
use graph::{Graph, Path};

mod graph_builder;
use graph_builder::build_graph;

fn main() {
    let graph_reader = get_graph_reader();
    let graph: Graph<String> = graph_builder::build_graph(graph_reader);

    search_graph(stdin(), &graph);
}

fn get_graph_reader() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let ref path = args[1];

    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    return buf_reader
}

fn search_graph<R: Read>(reader: R, graph: &Graph<String>) {
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" || line == "" {
            break;
        }
        
        // l[0] = start, l[1] = end
        let l: Vec<&str> = line.as_str().trim().split(" ").collect(); 
        match l.len() {
            2 => {
                let path = graph.shortest_path(&l[0].to_string(), &l[1].to_string());
                print_result(path);
            },
            _ => println!("Invalid number of nodes. Please input 2 nodes."),
        }
    }
}

fn print_result(path: Path<String>) {
    if path.is_err() {
        println!("{}", path.unwrap_err());
    } else {
        match path.unwrap() {
            Some(p) => println!("{}", p.join(" ")),
            _ => println!("There is no path between these nodes."),
        }
    }
}