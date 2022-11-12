// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

pub mod error;

pub mod tokenizer;

pub mod expression;
pub mod parser;
pub mod statement;
pub mod structure;

pub mod generator;
pub mod symboltable;

pub mod compiler;

#[cfg(target_arch = "wasm32")]
pub mod checker;
