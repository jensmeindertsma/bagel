pub mod expression;
pub mod statement;
pub mod visitor;

use expression::Expression;
use statement::Statement;

#[derive(Debug)]
pub struct Tree {
    pub kind: TreeKind,
    pub line: usize,
}

impl Tree {
    pub fn expression(expression: Expression) -> Self {
        let line = expression.line;
        Self {
            kind: TreeKind::Expression(expression),
            line,
        }
    }

    pub fn statement(statement: Statement) -> Self {
        let line = statement.line;
        Self {
            kind: TreeKind::Statement(statement),
            line,
        }
    }
}

#[derive(Debug)]
pub enum TreeKind {
    Expression(Expression),
    Statement(Statement),
}
