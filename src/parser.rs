mod tree;

use crate::scanner::Token;
use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable};
use tree::{Operator, Primitive, Tree};

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
        let mut left_hand_side = match self.tokens.next().ok_or(ParserError::UnexpectedEOF)? {
            Token::False => Tree::Primitive(Primitive::Boolean(false)),
            Token::Nil => Tree::Primitive(Primitive::Nil),
            Token::Number { value, .. } => Tree::Primitive(Primitive::Number(value)),
            Token::String { value } => Tree::Primitive(Primitive::String(value)),
            Token::True => Tree::Primitive(Primitive::Boolean(true)),

            Token::LeftParenthesis => {
                let inside = self.parse_expression(0)?;

                let next = self.tokens.next().ok_or(ParserError::UnexpectedEOF)?;
                if next != Token::RightParenthesis {
                    return Err(ParserError::UnexpectedToken(next));
                }

                Tree::Operation {
                    operator: Operator::Group,
                    arguments: vec![inside],
                }
            }

            token @ (Token::Bang | Token::Minus) => {
                let operator = match token {
                    Token::Bang => Operator::Not,
                    Token::Minus => Operator::Negation,
                    _ => unreachable!(),
                };

                let (_, Some(minimum_binding_power)) = operator.binding_power() else {
                    panic!("failed to get operator binding power")
                };

                let expression = self.parse_expression(minimum_binding_power)?;

                Tree::Operation {
                    operator,
                    arguments: vec![expression],
                }
            }
            _ => todo!("unhandled token"),
        };

        loop {
            // We've got ourselves a left-hand-side, now we look at the operator we expect
            // to follow it. We then keep folding into the left-hand-side new expressions
            // until we find the point where the next operator binds weaker to the latest token
            // than we do. This marks the end of the folding loop.

            let operator = match self.tokens.peek() {
                None | Some(Token::Eof) => break,
                Some(Token::RightParenthesis) => break,
                Some(Token::Slash) => Operator::Division,
                Some(Token::Star) => Operator::Multiplication,
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

            left_hand_side = Tree::Operation {
                operator,
                arguments: vec![left_hand_side, right_hand_side],
            };
        }

        Ok(left_hand_side)
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnexpectedToken(Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEOF => write!(f, "unexpected EOF"),
            Self::UnexpectedToken(token) => {
                write!(f, "found unexpected token `{token:?}`")
            }
        }
    }
}

impl Error for ParserError {}
