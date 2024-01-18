use std::fmt::{Display, Formatter, self};

pub enum Token {
    LParen,
    RParen,
    Lambda,
    Dot,
    Var(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Lambda => write!(f, "\\"),
            Token::Dot => write!(f, "."),
            Token::Var(var) => write!(f, "{}", var),
        }
    }
}