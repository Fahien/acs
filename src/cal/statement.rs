// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{expression::Expression, structure::Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub predicate: Expression,
    pub if_branch: Vec<Statement>,
    pub else_branch: Vec<Statement>,
}

impl IfStatement {
    pub fn new(
        predicate: Expression,
        if_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
    ) -> Self {
        Self {
            predicate,
            if_branch,
            else_branch,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhileStatement {
    pub predicate: Expression,
    pub body: Vec<Statement>,
}

impl WhileStatement {
    pub fn new(predicate: Expression, body: Vec<Statement>) -> Self {
        Self { predicate, body }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
    Let(Variable, Expression),
    If(IfStatement),
    While(WhileStatement),
}
