// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{error::CalError, tokenizer::Symbol};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
}

impl Operator {
    pub fn from_symbol(sym: Symbol) -> Result<Self, CalError> {
        match sym {
            Symbol::Plus => Ok(Self::Add),
            Symbol::Minus => Ok(Self::Sub),
            _ => Err(format!("Failed to convert `{:?}` to an operator", sym).into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    IntLiteral(i16),
    /// Call a function with a list of arguments
    Call(String, Vec<Expression>),
    Variable(String),
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
