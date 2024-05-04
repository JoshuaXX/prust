//!
//!  # document for lib crate
//!  1. this is for chapter 09. Structs
//!  2. to see the doc run:
//!     ```shell
//!         cargo doc --open
//!     ```
//!
//!     ```
//!       assert_eq!(1, 1);
//!     ```
//!
use std::rc::Rc;

#[derive(Debug)]
pub struct PointTuple(f64, f64, f64);

impl PointTuple {
    pub fn new(x: f64, y: f64, z: f64) -> PointTuple {
        PointTuple(x, y, z)
    }

    pub fn distance(&self, p: &PointTuple) -> f64 {
        let x_square = (self.0 - p.0).powf(2.0);
        let y_square = (self.1 - p.1).powf(2.0);
        let z_square = (self.2 - p.2).powf(2.0);

        (x_square + y_square + z_square).sqrt()
    }
}

#[derive(Debug)]
pub struct PointStruct {
    x: f64,
    y: f64,
    z: f64,
}

impl PointStruct {
    pub fn new(x: f64, y: f64, z: f64) -> PointStruct {
        PointStruct { x, y, z }
    }

    pub fn distance(&self, p: &PointStruct) -> f64 {
        let x_square = (self.x - p.x).powf(2.0);
        let y_square = (self.y - p.y).powf(2.0);
        let z_square = (self.z - p.z).powf(2.0);

        (x_square + y_square + z_square).sqrt()
    }
}

pub struct Node {
    pub tag: String,
    pub children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    pub fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}
