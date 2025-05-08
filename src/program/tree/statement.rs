use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub enum StatementKind {
    // Block(Vec<Statement>),
    // Expression(Expression),
    Print(Expression),
    // While {
    //     condition: Expression,
    //     body: Box<Statement>,
    // },
}
