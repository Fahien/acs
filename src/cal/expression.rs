// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

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
}

impl Expression {
    pub fn new(term: Box<Term>) -> Self {
        Self { term }
    }
}
