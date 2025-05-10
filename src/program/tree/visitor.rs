use super::{Expression, Statement};

pub trait Visitor<R> {
    fn visit_expression(&self, expression: &Expression) -> R;
    fn visit_program(&self, statements: &[Statement]) -> R;
    fn visit_statement(&self, statement: &Statement) -> R;
}
