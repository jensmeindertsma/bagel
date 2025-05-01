pub mod operation;
pub mod operator;
pub mod primitive;
pub mod visitor;

use operation::Operation;
use primitive::Primitive;

#[derive(Debug)]
pub struct Tree {
    pub kind: Kind,
    pub line: usize,
}

impl Tree {
    pub fn new(kind: Kind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug)]
pub enum Kind {
    Operation(Operation),
    Primitive(Primitive),
}
