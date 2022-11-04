// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    asm::instruction::AsmInstruction, generator::Generator, parser::Parser, tokenizer::tokenize,
    VmTranslator,
};

/// Compiles Cal source code and returns a series of asm instructions
pub fn compile(input: &str) -> Result<Vec<AsmInstruction>, String> {
    let module = Parser::new(tokenize(input)).parse_module()?;
    let vm_instructions = Generator::default().gen(&[module]);
    Ok(VmTranslator::default().translate(vm_instructions))
}

pub trait Compile {
    /// Compiles Cal source code and returns a series of asm instructions
    fn compile(&self) -> Result<Vec<AsmInstruction>, String>;
}

impl Compile for str {
    fn compile(&self) -> Result<Vec<AsmInstruction>, String> {
        compile(self)
    }
}
