use std::cell::{RefCell};
use std::ops::Deref;

use crate::lang::{Token};
use crate::lang::token::Punctuation;

use super::Position;

#[derive(Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right
}
#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
    LEqual,
    GEqual,
    BooleanAnd,
    BooleanOr,
    Concat,
}
impl BinaryOperator {
    pub fn get_operator(token: &Token) -> Option<BinaryOperator> {
        match token {
            Token::Punctuation(Punctuation::Plus) => Some(BinaryOperator::Add),
            Token::Punctuation(Punctuation::Minus) => Some(BinaryOperator::Sub),
            Token::Punctuation(Punctuation::Asterisk) => Some(BinaryOperator::Mul),
            Token::Punctuation(Punctuation::Slash) => Some(BinaryOperator::Div),
            Token::Punctuation(Punctuation::Equal) => Some(BinaryOperator::Equal),
            Token::Punctuation(Punctuation::NotEqual) => Some(BinaryOperator::NotEqual),
            Token::Punctuation(Punctuation::Greater) => Some(BinaryOperator::Greater),
            Token::Punctuation(Punctuation::Less) => Some(BinaryOperator::Less),
            Token::Punctuation(Punctuation::GEqual) => Some(BinaryOperator::GEqual),
            Token::Punctuation(Punctuation::LEqual) => Some(BinaryOperator::LEqual),
            Token::Punctuation(Punctuation::BooleanAnd) => Some(BinaryOperator::BooleanAnd),
            Token::Punctuation(Punctuation::BooleanOr) => Some(BinaryOperator::BooleanOr),
            Token::Punctuation(Punctuation::Concat) => Some(BinaryOperator::Concat),
            _ => None
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            BinaryOperator::BooleanAnd | BinaryOperator::BooleanOr => 0,

            BinaryOperator::Equal | BinaryOperator::NotEqual |
            BinaryOperator::Less | BinaryOperator::Greater |
            BinaryOperator::LEqual | BinaryOperator::GEqual => 1,

            BinaryOperator::Add | BinaryOperator::Sub => 2,

            BinaryOperator::Mul | BinaryOperator::Div => 3,

            BinaryOperator::Concat => 4,
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self {
            _ => Associativity::Left
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not
}
impl UnaryOperator {
    pub fn get_operator(p: Punctuation) -> Option<UnaryOperator> {
        match p {
            Punctuation::Minus => Some(UnaryOperator::Negate),
            Punctuation::Exclamation => Some(UnaryOperator::Not),
            _ => None
        }
    }
}

