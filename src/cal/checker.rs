// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::CalError,
    generator::generate,
    parser::Parser,
    tokenizer::{Range, Tokenize},
};

#[derive(Copy, Clone, Default)]
#[wasm_bindgen]
pub struct JsRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct JsCalError {
    message: String,
    pub range: JsRange,
}

#[wasm_bindgen]
impl JsCalError {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            message: String::new(),
            range: JsRange::default(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<Range> for JsRange {
    fn from(range: Range) -> Self {
        JsRange {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<CalError> for JsCalError {
    fn from(err: CalError) -> Self {
        JsCalError {
            message: err.message,
            range: err.range.into(),
        }
    }
}

/// The checker combines both the tokenizer and the parser.
/// This is useful for using this in the Cal Language Server.
#[wasm_bindgen]
pub fn check(code: &str) -> Result<(), JsCalError> {
    let tokens = code.tokenize()?;
    let module = Parser::new(tokens).parse_module()?;
    generate(module)?;
    Ok(())
}
