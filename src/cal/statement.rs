// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::expression::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
}
