use super::{Operation, Primitive};

pub trait Visitor<R> {
    fn visit_operation(&self, operation: &Operation) -> R;
    fn visit_primitive(&self, primitive: &Primitive) -> R;
}
