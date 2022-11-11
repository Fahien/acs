// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{expression::Expression, structure::Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
    Let(Variable, Expression),
}
