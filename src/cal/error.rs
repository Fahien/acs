// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::tokenizer::Range;

#[derive(Clone, Debug)]
pub struct CalError {
    pub message: String,
    pub range: Range,
}

impl CalError {
    pub fn new(message: String, range: Range) -> Self {
        Self { message, range }
    }
}

impl From<String> for CalError {
    fn from(message: String) -> Self {
        Self::new(message, Range::default())
    }
}

impl FromStr for CalError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CalError::new(s.into(), Range::default()))
    }
}
