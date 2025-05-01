use super::tree::{
    operation::Operation,
    operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator},
    primitive::Primitive,
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
        match tree {
            Tree::Operation(operation) => self.visit_operation(operation),
            Tree::Primitive(primitive) => self.visit_primitive(primitive),
        }
    }
}

impl<'a> Visitor<String> for Printer<'a> {
    fn visit_operation(&self, operation: &Operation) -> String {
        match operation {
            Operation::Arithmetic { operator, a, b } => {
                let operator = match operator {
                    ArithmeticOperator::Add => "+",
                    ArithmeticOperator::Divide => "/",
                    ArithmeticOperator::Multiply => "*",
                    ArithmeticOperator::Subtract => "-",
                };

                format!("({operator} {} {})", self.visit_tree(a), self.visit_tree(b))
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

                format!("({operator} {} {})", self.visit_tree(a), self.visit_tree(b))
            }
            Operation::Group(expression) => format!("(group {})", self.visit_tree(expression)),
            Operation::Logical {
                operator,
                expression,
            } => {
                let operator = match operator {
                    LogicalOperator::Negate => "-",
                    LogicalOperator::Not => "!",
                };

                format!("({operator} {})", self.visit_tree(expression))
            }
        }
    }

    fn visit_primitive(&self, primitive: &Primitive) -> String {
        match primitive {
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
        }
    }
}
