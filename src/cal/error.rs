// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

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
