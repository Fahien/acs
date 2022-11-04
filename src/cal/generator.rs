// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    expression::{Expression, Term},
    segment::Segment,
    statement::Statement,
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
            Type::I16 => 2,
        }
    }

    /// Returns the size in words of the type
    fn get_type_size_in_words(&self, typ: &Type) -> u16 {
        self.get_type_size(typ) / 2
    }

    fn gen_term(&self, term: &Term) -> Vec<VmInstruction> {
        match term {
            Term::IntLiteral(integer) => {
                let integer = unsafe { std::mem::transmute::<i16, u16>(*integer) };
                vec![VmInstruction::Push(Segment::Constant, integer)]
            }
        }
    }

    pub fn gen_expression(&self, expr: &Expression) -> Vec<VmInstruction> {
        self.gen_term(expr.term.as_ref())
    }

    pub fn gen_return(&mut self, expr: &Option<Expression>) -> Vec<VmInstruction> {
        let mut ret = vec![];
        if let Some(expr) = expr {
            ret.extend(self.gen_expression(expr));
        }
        // Return is not known at this point. Let `gen_function` set it before returning.
        ret.push(VmInstruction::Return(0));
        ret
    }

    pub fn gen_statement(&mut self, statement: &Statement) -> Vec<VmInstruction> {
        match statement {
            Statement::Return(expr) => self.gen_return(expr),
            Statement::Expression(expression) => self.gen_expression(expression),
        }
    }

    pub fn gen_statements(&mut self, statements: &[Statement]) -> Vec<VmInstruction> {
        let mut ret = vec![];
        for statement in statements {
            ret.extend(self.gen_statement(statement));
        }
        ret
    }

    /// Generates VM instructions for a function
    pub fn gen_function(&mut self, function: &Function) -> Vec<VmInstruction> {
        let mut ret = vec![VmInstruction::Function(
            function.name.clone(),
            function.local_count,
        )];

        ret.extend(self.gen_statements(&function.body_statements));

        // Set the return type size to all return instruction
        let return_type_size_in_words = self.get_type_size_in_words(&function.return_type);

        ret.iter_mut().for_each(|instr| {
            if let VmInstruction::Return(size_in_words) = instr {
                *size_in_words = return_type_size_in_words;
            }
        });

        // Add a return if missing
        if !matches!(ret.last(), Some(VmInstruction::Return(_))) {
            ret.push(VmInstruction::Return(return_type_size_in_words));
        }
        ret
    }

    /// Generates VM instructions for a module
    pub fn gen_module(&mut self, module: &Module) -> Vec<VmInstruction> {
        module
            .functions
            .iter()
            .flat_map(|function| self.gen_function(function))
            .collect()
    }

    /// Generates VM instructions for a series of modules
    pub fn gen(&mut self, modules: &[Module]) -> Vec<VmInstruction> {
        let mut instructions = preamble();
        instructions.extend(modules.iter().flat_map(|module| self.gen_module(module)));
        instructions
    }
}

pub fn generate(module: Module) -> Result<Vec<VmInstruction>, String> {
    Ok(Generator::default().gen_module(&module))
}

pub trait Generate {
    fn generate(&self) -> Result<Vec<VmInstruction>, String>;
}

impl Generate for str {
    fn generate(&self) -> Result<Vec<VmInstruction>, String> {
        let module: Module = self.parse()?;
        Ok(Generator::default().gen_module(&module))
    }
}
