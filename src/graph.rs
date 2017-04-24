pub struct graph<T> {
    node: Node<T>,
    size: usize,
    none: Option<T>,
}

// -----------------------------------------------------------------

#[derive(Debug, Eq)]
struct Node<T> {
    value: Option<T>,
    neighbors: Box<[Option<Box<Node<T>>>]>,
}

impl<T> Node<T> {

    fn new(input: &str) -> Self {

    }
}