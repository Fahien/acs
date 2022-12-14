// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{error::CalError, tokenizer::Symbol};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    // `==`
    Eq,
    // `!=`
    Ne,
    // `<`
    Lt,
    // `>`
    Gt,
    /// `=`
    Assign,
    /// `&`
    And,
    /// `|`
    Or,
    /// `%`
    Mod,
}

impl Operator {
    pub fn from_symbol(sym: Symbol) -> Result<Self, CalError> {
        match sym {
            Symbol::Plus => Ok(Self::Add),
            Symbol::Minus => Ok(Self::Sub),
            Symbol::Asterisk => Ok(Self::Mul),
            Symbol::Slash => Ok(Self::Div),
            Symbol::Eq => Ok(Self::Eq),
            Symbol::Ne => Ok(Self::Ne),
            Symbol::Lt => Ok(Self::Lt),
            Symbol::Gt => Ok(Self::Gt),
            Symbol::Assign => Ok(Self::Assign),
            Symbol::Ampersand => Ok(Self::And),
            Symbol::VerticalBar => Ok(Self::Or),
            Symbol::Percent => Ok(Self::Mod),
            _ => Err(format!("Failed to convert `{:?}` to an operator", sym).into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `&`
    Ref,
}

impl UnaryOperator {
    pub fn from_symbol(sym: Symbol) -> Result<Self, CalError> {
        match sym {
            Symbol::Ampersand => Ok(Self::Ref),
            _ => Err(format!("Failed to convert `{:?}` to an unary operator", sym).into()),
        }
    }
}

/// An enum for literals will come in handy when defining arrays
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    I16(i16),
    Bool(bool),
    Char(char),
    Array(Vec<Literal>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Literal(Literal),
    /// Call a function with a list of arguments
    Call(String, Vec<Expression>),
    Variable(String),
    /// Call the index operator on a variable, where
    /// index is the result of an expression.
    Index(String, Expression),

    /// Apply an unary operator to the term to its right
    UnaryOp(UnaryOperator, Box<Term>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
    pub term: Box<Term>,

    /// The term on the left may be followed by an operator and another
    /// expression on the right
    pub op_and_expr: Option<(Operator, Box<Expression>)>,
}

impl Expression {
    pub fn new(term: Box<Term>, op_and_expr: Option<(Operator, Box<Expression>)>) -> Self {
        Self { term, op_and_expr }
    }
}
