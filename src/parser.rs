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

    fn parse_expression(&mut self, _minimum_binding_power: u8) -> Result<Tree, ParserError> {
        let left_hand_side = match self.tokens.next().ok_or(ParserError::UnexpectedEOF)? {
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
                    expression: Box::new(inside),
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
                    expression: Box::new(expression),
                }
            }
            _ => todo!(),
        };

        // loop {
        //     let token = match self.tokens.next() {
        //         None => break,
        //         Some(Token::RightParenthesis) => break,
        //     };
        // }

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
