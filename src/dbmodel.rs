
// Define a struct to
// represent the 
//coordinates in the
// K-D tree
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub coordinates: Vec<f64>,
}

impl Vector {
    pub fn new(coordinates: Vec<f64>) -> Self {
        Vector { coordinates }
    }
}

// Define a struct to 
//represent a KD tree node
#[derive(Debug)]
pub struct Node {
    pub vector: Vector,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(vector: Vector) -> Self {
        Node {
            vector,
            left: None,
            right: None,
        }
    }
}