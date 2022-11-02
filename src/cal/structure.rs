// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::statement::Statement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub typ: Type,
}

impl Variable {
    pub fn new(name: String, typ: Type) -> Self {
        Self { name, typ }
    }
}

impl From<String> for Variable {
    fn from(name: String) -> Self {
        Self {
            name,
            typ: Type::Void,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Void,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Variable>,
    /// Number of local variables
    pub local_count: u16,
    pub body_statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

impl Field {
    pub fn new(name: String, typ: Type) -> Self {
        Self { name, typ }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructDec {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    // TODO add constants and structs
}

impl Module {
    pub fn new(name: impl Into<String>, functions: Vec<Function>) -> Self {
        Self {
            name: name.into(),
            functions,
        }
    }
}
