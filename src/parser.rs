mod tree;

use crate::scanner::Token;
use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable};
use tree::{
    ArithmeticOperator, ComparisonOperator, LogicalOperator, Operation, Operator, Primitive,
    Strength, Tree,
};

pub struct Parser<T>
where
    T: IntoIterator,
{
    tokens: Peekable<T::IntoIter>,
}

impl<T> Parser<T>
where
    T: IntoIterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Tree, ParserError> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, minimum_binding_power: u8) -> Result<Tree, ParserError> {
        let mut left_hand_side = match self.tokens.next().ok_or(ParserError::UnexpectedEof)? {
            Token::False => Tree::Primitive(Primitive::Boolean(false)),

            Token::Nil => Tree::Primitive(Primitive::Nil),

            Token::Number { value, .. } => Tree::Primitive(Primitive::Number(value)),

            Token::String { value } => Tree::Primitive(Primitive::String(value)),

            Token::True => Tree::Primitive(Primitive::Boolean(true)),

            Token::LeftParenthesis => {
                // A left parenthesis marks the beginning of a "group".

                // First we parse the "inside" of the group. This will be
                // terminated automatically by the presence of a right
                // parenthesis. Inside the main loop below, encountering
                // a right parenthesis will break out of the expression
                // folding immediately.
                let inside = self.parse_expression(0)?;

                // Then we expect to see the right parenthesis.
                let next = self.tokens.next().ok_or(ParserError::UnexpectedEof)?;
                if next != Token::RightParenthesis {
                    return Err(ParserError::UnexpectedToken(next));
                }

                Tree::Operation(Operation::Group(Box::new(inside)))
            }

            token @ (Token::Bang | Token::Minus) => {
                // Here we catch a preceding bang or minus before an expression. These are
                // logical operators which apply to the whole expression.
                let operator = match token {
                    Token::Bang => LogicalOperator::Not,
                    Token::Minus => LogicalOperator::Negate,
                    _ => unreachable!("by above pattern match"),
                };

                let (_, Some(minimum_binding_power)) = operator.binding_power() else {
                    panic!("failed to get operator binding power")
                };

                let expression = self.parse_expression(minimum_binding_power)?;

                Tree::Operation(Operation::Logical {
                    operator,
                    expression: Box::new(expression),
                })
            }

            unexpected_token => {
                return Err(ParserError::UnexpectedToken(unexpected_token));
            }
        };

        loop {
            // We've got ourselves a left-hand-side, now we look at the operator we expect
            // to follow it. We then keep folding into the left-hand-side new expressions
            // until we find the point where the next operator binds weaker to the latest token
            // than we do. This marks the end of the folding loop.

            let operator: Operator = match self.tokens.peek() {
                None | Some(Token::Eof) => break,

                Some(Token::RightParenthesis) => break,

                Some(Token::Minus) => ArithmeticOperator::Subtract.into(),
                Some(Token::Plus) => ArithmeticOperator::Add.into(),
                Some(Token::Slash) => ArithmeticOperator::Divide.into(),
                Some(Token::Star) => ArithmeticOperator::Multiply.into(),

                Some(Token::EqualEqual) => ComparisonOperator::Equal.into(),
                Some(Token::Greater) => ComparisonOperator::GreaterThan.into(),
                Some(Token::GreaterEqual) => ComparisonOperator::GreaterEqual.into(),
                Some(Token::Less) => ComparisonOperator::LessThan.into(),
                Some(Token::LessEqual) => ComparisonOperator::LessEqual.into(),
                Some(Token::BangEqual) => ComparisonOperator::NotEqual.into(),
                _ => todo!("unhandled operator"),
            };

            let (Some(left_binding_power), Some(right_binding_power)) = operator.binding_power()
            else {
                panic!("failed to get operator binding power")
            };

            if left_binding_power < minimum_binding_power {
                // We have finished folding the expression because the next operator
                // binds weaker to our current token
                break;
            }

            // We should consume the token we peeked at.
            self.tokens.next();

            let right_hand_side = self.parse_expression(right_binding_power)?;

            left_hand_side = Tree::Operation(match operator {
                Operator::Arithmetic(operator) => Operation::Arithmetic {
                    operator,
                    a: Box::new(left_hand_side),
                    b: Box::new(right_hand_side),
                },

                Operator::Comparison(operator) => Operation::Comparison {
                    operator,
                    a: Box::new(left_hand_side),
                    b: Box::new(right_hand_side),
                },

                _ => unreachable!("by above match statement"),
            })
        }

        Ok(left_hand_side)
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEof,
    UnexpectedToken(Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected EOF"),
            Self::UnexpectedToken(token) => {
                write!(f, "found unexpected token `{token:?}`")
            }
        }
    }
}

impl Error for ParserError {}
