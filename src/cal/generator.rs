// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    error::CalError,
    expression::{Expression, Operator, Term},
    segment::Segment,
    statement::Statement,
    structure::{Function, Module, Type, Variable},
    symboltable::SymbolTable,
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
pub struct Generator {
    symbol_tables: Vec<SymbolTable>,
}

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

    fn get_current_symbol_table(&self) -> &SymbolTable {
        self.symbol_tables.last().unwrap()
    }

    fn get_current_symbol_table_mut(&mut self) -> &mut SymbolTable {
        self.symbol_tables.last_mut().unwrap()
    }

    fn gen_term(&self, term: &Term) -> Vec<VmInstruction> {
        match term {
            Term::IntLiteral(integer) => {
                let integer = unsafe { std::mem::transmute::<i16, u16>(*integer) };
                vec![VmInstruction::Push(Segment::Constant, integer)]
            }
            Term::Call(name, expressions) => {
                let mut ret = vec![];
                for expr in expressions {
                    ret.extend(self.gen_expression(expr));
                }
                ret.push(VmInstruction::Call(name.clone(), expressions.len() as u16));
                ret
            }
            Term::Variable(name) => {
                let (segment, offset) =
                    self.get_current_symbol_table().get_segment_and_offset(name);
                vec![VmInstruction::Push(segment, offset)]
            }
        }
    }

    /// Generate a VM instruction for an operator
    fn gen_operator(&self, op: &Operator) -> VmInstruction {
        match op {
            Operator::Add => VmInstruction::Add,
        }
    }

    pub fn gen_expression(&self, expr: &Expression) -> Vec<VmInstruction> {
        // Generate instructions for the term
        let mut ret = self.gen_term(expr.term.as_ref());

        // Generate instructions for the operator and the right side expression
        if let Some((op, expr)) = &expr.op_and_expr {
            ret.extend(self.gen_expression(expr.as_ref()));
            ret.push(self.gen_operator(op));
        }

        ret
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

    pub fn gen_let(
        &mut self,
        variable: &Variable,
        assign_expression: &Expression,
    ) -> Vec<VmInstruction> {
        let mut ret = vec![];
        ret.extend(self.gen_expression(assign_expression));
        let offset = self.get_current_symbol_table_mut().insert_local(variable);
        ret.push(VmInstruction::Pop(Segment::Local, offset));
        ret
    }

    pub fn gen_statement(&mut self, statement: &Statement) -> Vec<VmInstruction> {
        match statement {
            Statement::Return(expr) => self.gen_return(expr),
            Statement::Expression(expression) => self.gen_expression(expression),
            Statement::Let(variable, assign_expression) => {
                self.gen_let(variable, assign_expression)
            }
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
        // New symbol table
        self.symbol_tables.push(SymbolTable::default());

        let mut ret = vec![VmInstruction::Function(
            function.name.clone(),
            function.local_count,
        )];

        // Add function arguments to symbol table
        for arg in &function.parameters {
            self.get_current_symbol_table_mut().insert_argument(arg);
        }

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

        // Clear symbol table for this function
        self.symbol_tables.pop();

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

pub fn generate(module: Module) -> Vec<VmInstruction> {
    Generator::default().gen_module(&module)
}

pub trait Generate {
    fn generate(&self) -> Result<Vec<VmInstruction>, CalError>;
}

impl Generate for str {
    fn generate(&self) -> Result<Vec<VmInstruction>, CalError> {
        let module: Module = self.parse()?;
        Ok(Generator::default().gen_module(&module))
    }
}
