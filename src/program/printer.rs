use super::tree::{
    expression::{
        operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator},
        Expression, ExpressionKind, Operation, Primitive,
    },
    statement::{Statement, StatementKind},
    visitor::Visitor,
    Tree,
};

pub struct Printer<'a> {
    tree: &'a Tree,
}

impl<'a> Printer<'a> {
    pub fn new(tree: &'a Tree) -> Self {
        Self { tree }
    }

    pub fn print(&self) -> String {
        self.visit_tree(self.tree)
    }

    fn visit_tree(&self, tree: &Tree) -> String {
        match &tree {
            Tree::Expression(expression) => self.visit_expression(expression),
            Tree::Program(statements) => self.visit_program(statements),
            Tree::Statement(statement) => self.visit_statement(statement),
        }
    }
}

impl<'a> Visitor<String> for Printer<'a> {
    fn visit_expression(&self, expression: &Expression) -> String {
        match &expression.kind {
            ExpressionKind::Operation(operation) => match operation {
                Operation::Arithmetic { operator, a, b } => {
                    let operator = match operator {
                        ArithmeticOperator::Add => "+",
                        ArithmeticOperator::Divide => "/",
                        ArithmeticOperator::Multiply => "*",
                        ArithmeticOperator::Subtract => "-",
                    };

                    format!(
                        "({operator} {} {})",
                        self.visit_expression(a),
                        self.visit_expression(b)
                    )
                }
                Operation::Comparison { operator, a, b } => {
                    let operator = match operator {
                        ComparisonOperator::Equal => "==",
                        ComparisonOperator::GreaterEqual => ">=",
                        ComparisonOperator::GreaterThan => ">",
                        ComparisonOperator::LessEqual => "<=",
                        ComparisonOperator::LessThan => "<",
                        ComparisonOperator::NotEqual => "!=",
                    };

                    format!(
                        "({operator} {} {})",
                        self.visit_expression(a),
                        self.visit_expression(b)
                    )
                }
                Operation::Group(expression) => {
                    format!("(group {})", self.visit_expression(expression))
                }
                Operation::Logical {
                    operator,
                    expression,
                } => {
                    let operator = match operator {
                        LogicalOperator::Negate => "-",
                        LogicalOperator::Not => "!",
                    };

                    format!("({operator} {})", self.visit_expression(expression))
                }
            },
            ExpressionKind::Primitive(primitive) => match primitive {
                Primitive::Boolean(value) => value.to_string(),
                Primitive::Nil => "nil".to_owned(),
                Primitive::Number(value) => {
                    if value.fract() == 0.0 {
                        format!("{value:.1}")
                    } else {
                        format!("{value}")
                    }
                }
                Primitive::String(string) => string.clone(),
            },
        }
    }

    fn visit_program(&self, statements: &[Statement]) -> String {
        statements
            .iter()
            .map(|stmt| self.visit_statement(stmt))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn visit_statement(&self, statement: &Statement) -> String {
        match &statement.kind {
            StatementKind::Print(expression) => {
                format!("(print {})", self.visit_expression(expression))
            }
        }
    }
}
