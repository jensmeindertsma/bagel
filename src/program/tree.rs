pub mod operation;
pub mod operator;
pub mod primitive;
pub mod visitor;

use operation::Operation;
use primitive::Primitive;

#[derive(Debug)]
pub enum Tree {
    Operation(Operation),
    Primitive(Primitive),
}
