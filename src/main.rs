//! EECS 395
//! Homework 3: Rust Graph
//! By Andy McConnell
//!
//! This program takes an association list as the first argument. Then takes in
//! start and end nodes on the graph. If a path exists between the two points, the
//! program will print out the shortest path between the two points. The program continues
//! to take in two nodes until the user inputs a blank line.
//!
//! The graph assumes edges are undirected and that nodes have unique identifiers.

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
    let graph: Graph<String> = build_graph(graph_reader);

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
        if line == "" {
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