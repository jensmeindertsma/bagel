pub mod expression;
pub mod statement;
pub mod visitor;

use expression::Expression;
use statement::Statement;

#[derive(Debug)]
pub enum Tree {
    Expression(Expression),
    Program(Vec<Statement>),
    Statement(Statement),
}
