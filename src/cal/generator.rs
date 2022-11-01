// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    error::CalError,
    structure::{Function, Module, Type},
    vm::instruction::VmInstruction,
};

/// The preable is added at the beginning of the program and it is responsible
/// of calling the main function and going into and endless loop when returning
fn preamble() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Call("main".into(), 0),
        VmInstruction::Label("END".into()),
        VmInstruction::Goto("END".into()),
    ]
}

/// Generates VM instructions from parsed code.
#[derive(Default)]
pub struct Generator {}

impl Generator {
    /// Returns the size in bytes of the type
    fn get_type_size(&self, typ: &Type) -> u16 {
        match typ {
            Type::Void => 0,
        }
    }

    /// Returns the size in words of the type
    fn get_type_size_in_words(&self, typ: &Type) -> u16 {
        self.get_type_size(typ) / 2
    }

    /// Generates VM instructions for a function
    pub fn gen_function(&mut self, function: &Function) -> Result<Vec<VmInstruction>, CalError> {
        Ok(vec![
            VmInstruction::Function(function.name.clone(), function.local_count),
            VmInstruction::Return(self.get_type_size_in_words(&function.return_type)),
        ])
    }

    /// Generates VM instructions for a module
    pub fn gen_module(&mut self, module: &Module) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];
        for function in &module.functions {
            ret.extend(self.gen_function(function)?);
        }
        Ok(ret)
    }

    /// Generates VM instructions for a series of modules
    pub fn gen(&mut self, modules: &[Module]) -> Result<Vec<VmInstruction>, CalError> {
        let mut instructions = preamble();
        for module in modules {
            instructions.extend(self.gen_module(module)?);
        }
        Ok(instructions)
    }
}

pub fn generate(module: Module) -> Result<Vec<VmInstruction>, CalError> {
    Generator::default().gen_module(&module)
}

pub trait Generate {
    fn generate(&self) -> Result<Vec<VmInstruction>, CalError>;
}

impl Generate for str {
    fn generate(&self) -> Result<Vec<VmInstruction>, CalError> {
        let module: Module = self.parse()?;
        Generator::default().gen_module(&module)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_void() -> Result<(), CalError> {
        let module = "fn main() {}".parse()?;
        let vm_instructions = Generator::default().gen_module(&module)?;
        match &vm_instructions[0] {
            VmInstruction::Function(name, local_count) => {
                assert_eq!(name, "main");
                assert_eq!(*local_count, 0);
            }
            _ => panic!(),
        }
        match &vm_instructions[1] {
            VmInstruction::Return(return_size_in_words) => {
                assert_eq!(*return_size_in_words, 0);
            }
            _ => panic!(),
        }
        Ok(())
    }
}
