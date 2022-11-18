// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    error::CalError,
    expression::{Expression, Operator, Term},
    preamble::preamble,
    segment::Segment,
    statement::{IfStatement, Statement, WhileStatement},
    structure::{Function, Module, Type, Variable},
    symboltable::SymbolTable,
    tokenizer::Range,
    vm::instruction::VmInstruction,
};

/// Generates VM instructions from parsed code.
#[derive(Default)]
pub struct Generator {
    symbol_tables: Vec<SymbolTable>,
    label_count: u32,
}

impl Generator {
    /// Generate a label at VM instructions level
    fn next_label(&mut self) -> String {
        let ret = format!("VM_LABEL{}", self.label_count);
        self.label_count += 1;
        ret
    }

    /// Returns the size in bytes of the type
    fn get_type_size(&self, typ: &Type) -> u16 {
        match typ {
            Type::Void => 0,
            Type::I16 | Type::Bool => 2,
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

    fn gen_term(&self, term: &Term) -> Result<Vec<VmInstruction>, CalError> {
        match term {
            Term::IntLiteral(integer) => {
                let integer = unsafe { std::mem::transmute::<i16, u16>(*integer) };
                Ok(vec![VmInstruction::Push(Segment::Constant, integer)])
            }
            Term::BoolLiteral(false) => Ok(vec![VmInstruction::Push(Segment::Constant, 0)]),
            Term::BoolLiteral(true) => Ok(vec![
                VmInstruction::Push(Segment::Constant, 0),
                VmInstruction::Not,
            ]),
            Term::Call(name, expressions) => {
                let mut ret = vec![];
                for expr in expressions {
                    ret.extend(self.gen_expression(expr)?);
                }
                ret.push(VmInstruction::Call(name.clone(), expressions.len() as u16));
                Ok(ret)
            }
            Term::Variable(name) => {
                if let Some((segment, offset)) =
                    self.get_current_symbol_table().get_segment_and_offset(name)
                {
                    Ok(vec![VmInstruction::Push(segment, offset)])
                } else {
                    Err(CalError::new(
                        format!("Undefined variable `{}`", name),
                        Range::default(),
                    ))
                }
            }
        }
    }

    /// Generate a VM instruction for an operator
    fn gen_operator(&self, op: &Operator) -> Vec<VmInstruction> {
        match op {
            Operator::Add => vec![VmInstruction::Add],
            Operator::Sub => vec![VmInstruction::Sub],
            Operator::Mul => vec![VmInstruction::Call(String::from("mul"), 2)],
            Operator::Div => vec![VmInstruction::Call(String::from("div"), 2)],
            Operator::Eq => vec![VmInstruction::Eq],
            Operator::Ne => vec![VmInstruction::Eq, VmInstruction::Not],
            Operator::Lt => vec![VmInstruction::Lt],
            Operator::Gt => vec![VmInstruction::Gt],
            _ => unimplemented!(),
        }
    }

    pub fn gen_assign_expression(
        &self,
        term: &Term,
        rhs: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        // Get variable name from previous term
        if let Term::Variable(name) = term {
            if let Some((segment, offset)) =
                self.get_current_symbol_table().get_segment_and_offset(name)
            {
                let mut ret = self.gen_expression(rhs)?;
                ret.push(VmInstruction::Pop(segment, offset));
                Ok(ret)
            } else {
                Err(CalError::new(
                    format!("Undefined variable `{}`", name),
                    Range::default(),
                ))
            }
        } else {
            Err(CalError::new(
                format!("Expected variable to the left of `=`, found {:?}", term),
                Range::default(),
            ))
        }
    }

    pub fn gen_expression(&self, expr: &Expression) -> Result<Vec<VmInstruction>, CalError> {
        if let Some((op, rhs)) = &expr.op_and_expr {
            if *op == Operator::Assign {
                // Special case for assign expression
                self.gen_assign_expression(expr.term.as_ref(), rhs.as_ref())
            } else {
                // Common case
                let mut ret = self.gen_term(expr.term.as_ref())?;
                ret.extend(self.gen_expression(rhs.as_ref())?);
                ret.extend(self.gen_operator(op));
                Ok(ret)
            }
        } else {
            // Generate instructions for the term only
            self.gen_term(expr.term.as_ref())
        }
    }

    pub fn gen_return(
        &mut self,
        expr: &Option<Expression>,
    ) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];
        if let Some(expr) = expr {
            ret.extend(self.gen_expression(expr)?);
        }
        // Return is not known at this point. Let `gen_function` set it before returning.
        ret.push(VmInstruction::Return(0));
        Ok(ret)
    }

    pub fn gen_let(
        &mut self,
        variable: &Variable,
        assign_expression: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];
        ret.extend(self.gen_expression(assign_expression)?);
        let offset = self.get_current_symbol_table_mut().insert_local(variable);
        ret.push(VmInstruction::Pop(Segment::Local, offset));
        Ok(ret)
    }

    /// Generates VM instructions for an if statement
    pub fn gen_if(&mut self, if_stat: &IfStatement) -> Result<Vec<VmInstruction>, CalError> {
        let else_label = self.next_label();
        let endif_label = self.next_label();

        let mut ret = self.gen_expression(&if_stat.predicate)?;
        ret.push(VmInstruction::Not);
        ret.push(VmInstruction::IfGoto(else_label.clone()));

        ret.extend(self.gen_statements(&if_stat.if_branch)?);
        ret.push(VmInstruction::Goto(endif_label.clone()));

        ret.push(VmInstruction::Label(else_label));
        ret.extend(self.gen_statements(&if_stat.else_branch)?);

        ret.push(VmInstruction::Label(endif_label));

        Ok(ret)
    }

    /// Generates VM instructions for a while statement
    pub fn gen_while(
        &mut self,
        while_stat: &WhileStatement,
    ) -> Result<Vec<VmInstruction>, CalError> {
        let while_label = self.next_label();
        let endwhile_label = self.next_label();

        let mut ret = vec![VmInstruction::Label(while_label.clone())];
        ret.extend(self.gen_expression(&while_stat.predicate)?);
        ret.push(VmInstruction::Not);
        ret.push(VmInstruction::IfGoto(endwhile_label.clone()));

        ret.extend(self.gen_statements(&while_stat.body)?);
        ret.push(VmInstruction::Goto(while_label));

        ret.push(VmInstruction::Label(endwhile_label));

        Ok(ret)
    }

    pub fn gen_statement(&mut self, statement: &Statement) -> Result<Vec<VmInstruction>, CalError> {
        match statement {
            Statement::Return(expr) => self.gen_return(expr),
            Statement::Expression(expression) => self.gen_expression(expression),
            Statement::Let(variable, assign_expression) => {
                self.gen_let(variable, assign_expression)
            }
            Statement::If(ifstat) => self.gen_if(ifstat),
            Statement::While(whilestat) => self.gen_while(whilestat),
        }
    }

    pub fn gen_statements(
        &mut self,
        statements: &[Statement],
    ) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];
        for statement in statements {
            ret.extend(self.gen_statement(statement)?);
        }
        Ok(ret)
    }

    /// Generates VM instructions for a function
    pub fn gen_function(&mut self, function: &Function) -> Result<Vec<VmInstruction>, CalError> {
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

        ret.extend(self.gen_statements(&function.body_statements)?);

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

        Ok(ret)
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
