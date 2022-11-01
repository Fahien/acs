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

#[cfg(test)]
mod test {
    use crate::Computer;

    use super::*;

    #[test]
    fn hello_void() -> Result<(), String> {
        let asm_instructions = "fn main() {}".compile()?;
        let mut computer = Computer::default();
        computer.set_instructions(asm_instructions);
        for _ in 0..128 {
            computer.ticktock();
        }
        assert_eq!(computer.get_memory().ram[0], 256);
        Ok(())
    }
}
