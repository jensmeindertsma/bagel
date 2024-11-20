use core::fmt::{self, Formatter};

#[derive(Debug)]
pub enum Token {
    And,
    Bang,
    BangEqual,
    Class,
    Comma,
    Dot,
    Else,
    Eof,
    Equal,
    EqualEqual,
    False,
    For,
    Fun,
    Greater,
    GreaterEqual,
    Identifier { lexeme: String },
    If,
    LeftBrace,
    LeftParenthesis,
    Less,
    LessEqual,
    Minus,
    Nil,
    Number { literal: String, value: f64 },
    Or,
    Plus,
    Print,
    Return,
    RightBrace,
    RightParenthesis,
    Semicolon,
    Slash,
    Star,
    String { value: String },
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "AND and null"),
            Self::Bang => write!(f, "BANG ! null"),
            Self::BangEqual => write!(f, "BANG_EQUAL != null"),
            Self::Class => write!(f, "CLASS class null"),
            Self::Comma => write!(f, "COMMA , null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Eof => write!(f, "EOF  null"),
            Self::Else => write!(f, "ELSE else null"),
            Self::Equal => write!(f, "EQUAL = null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Self::False => write!(f, "FALSE false null"),
            Self::For => write!(f, "FOR for null"),
            Self::Fun => write!(f, "FUN fun null"),
            Self::Greater => write!(f, "GREATER > null"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Self::Identifier { lexeme } => write!(f, "IDENTIFIER {lexeme} null"),
            Self::If => write!(f, "IF if null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Self::Less => write!(f, "LESS < null"),
            Self::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Self::Minus => write!(f, "MINUS - null"),
            Self::Nil => write!(f, "NIL nil null"),
            Self::Number { literal, value } => {
                if *value == value.trunc() {
                    write!(f, "NUMBER {literal} {value}.0")
                } else {
                    write!(f, "NUMBER {literal} {value}")
                }
            }
            Self::Or => write!(f, "OR or null"),
            Self::Plus => write!(f, "PLUS + null"),
            Self::Print => write!(f, "PRINT print null"),
            Self::Return => write!(f, "RETURN return null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Self::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Star => write!(f, "STAR * null"),
            Self::String { value } => write!(f, "STRING \"{value}\" {value}"),
            Self::Super => write!(f, "SUPER super null"),
            Self::This => write!(f, "THIS this null"),
            Self::True => write!(f, "TRUE true null"),
            Self::Var => write!(f, "VAR var null"),
            Self::While => write!(f, "WHILE while null"),
        }
    }
}
