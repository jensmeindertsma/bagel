use super::{
    token::{Token, TokenKind},
    tree::{
        expression::{
            operator::{
                ArithmeticOperator, ComparisonOperator, LogicalOperator, Operator, Strength,
            },
            Expression, Operation, Primitive,
        },
        statement::{Statement, StatementKind},
        Tree,
    },
};
use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable};

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
        let token = self
            .tokens
            .peek()
            .ok_or(ParserError::new(ErrorKind::UnexpectedEof, 1))?;

        match token.kind {
            TokenKind::Print => {
                tracing::debug!("parsing first token `{}` as statement", token.kind);

                // this will consume the token we just peeked at
                let first = self.parse_statement()?;

                // we should check if there is another statement of not
                match self.parse_statement() {
                    Err(error) => {
                        match error.kind {
                            ErrorKind::UnexpectedEof => {
                                tracing::debug!("just a single statement");
                                // file contains just single statement
                                Ok(Tree::Statement(first))
                            }
                            _ => Err(error),
                        }
                    }
                    Ok(second) => {
                        let mut statements = vec![first, second];

                        loop {
                            let next = self.parse_statement();

                            match next {
                                Ok(statement) => {
                                    tracing::debug!("found new statement {statement:?}");
                                    statements.push(statement)
                                }
                                Err(error) => {
                                    tracing::debug!("error while parsing program: {error}");
                                    match error.kind {
                                        ErrorKind::UnexpectedEof => break,
                                        ErrorKind::UnexpectedToken {
                                            found: TokenKind::Eof,
                                            ..
                                        } => break,

                                        _ => return Err(error),
                                    }
                                }
                            }
                        }

                        tracing::debug!(
                            "parsed whole program with {} statements",
                            statements.len()
                        );

                        Ok(Tree::Program(statements))
                    }
                }
            }
            _ => {
                tracing::debug!("parsing first token `{}` as expression", token.kind);
                Ok(Tree::Expression(self.parse_expression(0)?))
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self
            .tokens
            .next()
            .ok_or(ParserError::new(ErrorKind::UnexpectedEof, 1))?;

        let statement = match token.kind {
            TokenKind::Print => {
                let expression = self.parse_expression(0)?;

                Statement {
                    kind: StatementKind::Print(expression),
                    line: token.line,
                }
            }
            other => {
                return Err(ParserError::new(
                    ErrorKind::UnexpectedToken {
                        found: other,
                        expected: None,
                    },
                    token.line,
                ))
            }
        };

        let next_token = self
            .tokens
            .next()
            .ok_or(ParserError::new(ErrorKind::UnexpectedEof, statement.line))?;

        if next_token.kind != TokenKind::Semicolon {
            return Err(ParserError::new(
                ErrorKind::UnexpectedToken {
                    found: next_token.kind,
                    expected: Some(TokenKind::Semicolon),
                },
                next_token.line,
            ));
        }

        tracing::debug!("parsed statement\n`{:?}`", statement.kind);

        Ok(statement)
    }

    fn parse_expression(&mut self, minimum_binding_power: u8) -> Result<Expression, ParserError> {
        let mut left_hand_side = {
            let token = self
                .tokens
                .next()
                .ok_or(ParserError::new(ErrorKind::UnexpectedEof, 1))?;

            match token.kind {
                TokenKind::Bang | TokenKind::Minus => {
                    // Here we catch a preceding bang or minus before an expression. These are
                    // logical operators which apply to the whole expression.
                    let operator = match token.kind {
                        TokenKind::Bang => LogicalOperator::Not,
                        TokenKind::Minus => LogicalOperator::Negate,
                        _ => unreachable!("by above pattern match"),
                    };

                    let (_, Some(minimum_binding_power)) = operator.binding_power() else {
                        panic!("failed to get operator binding power")
                    };

                    let expression = self.parse_expression(minimum_binding_power)?;

                    Expression::operation(
                        Operation::Logical {
                            operator,
                            expression: Box::new(expression),
                        },
                        token.line,
                    )
                }

                TokenKind::False => Expression::primitive(Primitive::Boolean(false), token.line),

                TokenKind::LeftParenthesis => {
                    // A left parenthesis marks the beginning of a "group".

                    // First we parse the "inside" of the group. This will be
                    // terminated automatically by the presence of a right
                    // parenthesis. Inside the main loop below, encountering
                    // a right parenthesis will break out of the expression
                    // folding immediately.
                    let inside = self.parse_expression(0)?;

                    let mut line = inside.line;

                    // Then we expect to see the right parenthesis.
                    let next = self
                        .tokens
                        .next()
                        .ok_or(ParserError::new(ErrorKind::UnexpectedEof, line))?;

                    line = next.line;

                    if next.kind != TokenKind::RightParenthesis {
                        return Err(ParserError::new(
                            ErrorKind::UnexpectedToken {
                                found: next.kind,
                                expected: Some(TokenKind::RightParenthesis),
                            },
                            line,
                        ));
                    }

                    Expression::operation(Operation::Group(Box::new(inside)), token.line)
                }

                TokenKind::Nil => Expression::primitive(Primitive::Nil, token.line),

                TokenKind::Number { value, .. } => {
                    Expression::primitive(Primitive::Number(value), token.line)
                }

                TokenKind::String { value } => {
                    Expression::primitive(Primitive::String(value), token.line)
                }

                TokenKind::True => Expression::primitive(Primitive::Boolean(true), token.line),

                _ => {
                    return Err(ParserError::new(
                        ErrorKind::UnexpectedToken {
                            found: token.kind,
                            expected: None,
                        },
                        token.line,
                    ))
                }
            }
        };

        tracing::debug!("left hand side = {:?}", left_hand_side.kind);

        loop {
            // We've got ourselves a left-hand-side, now we look at the operator we expect
            // to follow it. We then keep folding into the left-hand-side new expressions
            // until we find the point where the next operator binds weaker to the latest token
            // than we do. This marks the end of the folding loop.

            let peeked_token = match self.tokens.peek() {
                None => break,
                Some(token) => token,
            };

            let operator: Operator = match peeked_token.kind {
                TokenKind::Eof | TokenKind::RightParenthesis | TokenKind::Semicolon => break,
                TokenKind::Minus => ArithmeticOperator::Subtract.into(),
                TokenKind::Plus => ArithmeticOperator::Add.into(),
                TokenKind::Slash => ArithmeticOperator::Divide.into(),
                TokenKind::Star => ArithmeticOperator::Multiply.into(),
                TokenKind::EqualEqual => ComparisonOperator::Equal.into(),
                TokenKind::Greater => ComparisonOperator::GreaterThan.into(),
                TokenKind::GreaterEqual => ComparisonOperator::GreaterEqual.into(),
                TokenKind::Less => ComparisonOperator::LessThan.into(),
                TokenKind::LessEqual => ComparisonOperator::LessEqual.into(),
                TokenKind::BangEqual => ComparisonOperator::NotEqual.into(),
                _ => todo!("unhandled token {peeked_token:?}"),
            };

            tracing::debug!("parsing operator `{operator:?}`");

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
            let token = self
                .tokens
                .next()
                .expect("next token should exist because we peeked at it");

            let right_hand_side = self.parse_expression(right_binding_power)?;

            tracing::info!("right hand side = {:?}", right_hand_side.kind);

            left_hand_side = Expression::operation(
                match operator {
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
                },
                token.line,
            )
        }

        tracing::debug!("parsed expression\n`{:?}`", left_hand_side.kind);

        Ok(left_hand_side)
    }
}

#[derive(Debug)]
pub struct ParserError {
    kind: ErrorKind,
    line: usize,
}

impl ParserError {
    fn new(kind: ErrorKind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        found: TokenKind,
        expected: Option<TokenKind>,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedEof => write!(f, "unexpected EOF\n[line {}]", self.line),
            ErrorKind::UnexpectedToken { found, expected } => match expected {
                Some(expected) => {
                    write!(
                        f,
                        "found unexpected token `{found:?}`, expected {expected:?}\n[line {}]",
                        self.line
                    )
                }
                None => {
                    write!(
                        f,
                        "found unexpected token `{found:?}`\n[line {}]",
                        self.line
                    )
                }
            },
        }
    }
}

impl Error for ParserError {}
