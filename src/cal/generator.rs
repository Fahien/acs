// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    error::CalError,
    expression::{Expression, Literal, Operator, Term, UnaryOperator},
    preamble::preamble,
    segment::Segment,
    statement::{IfStatement, Statement, WhileStatement},
    structure::{Function, Module, Type, Variable},
    symboltable::{SymbolEntry, SymbolTable},
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
    #[allow(clippy::only_used_in_recursion)]
    fn get_type_size(&self, typ: &Type) -> u16 {
        match typ {
            Type::Void => 0,
            Type::I16 | Type::Bool | Type::Char | Type::Ref(_) => 2,
            Type::Array(elem_type, count) => self.get_type_size(elem_type.as_ref()) * count,
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

    #[allow(clippy::only_used_in_recursion)]
    fn gen_literal(&self, literal: &Literal) -> Result<Vec<VmInstruction>, CalError> {
        match literal {
            Literal::I16(integer) => {
                let integer = unsafe { std::mem::transmute::<i16, u16>(*integer) };
                Ok(vec![VmInstruction::Push(Segment::Constant, integer)])
            }
            Literal::Bool(false) => Ok(vec![VmInstruction::Push(Segment::Constant, 0)]),
            Literal::Bool(true) => Ok(vec![
                VmInstruction::Push(Segment::Constant, 0),
                VmInstruction::Not,
            ]),
            Literal::Char(c) => Ok(vec![VmInstruction::Push(Segment::Constant, *c as u16)]),
            Literal::Array(values) => {
                let mut ret = vec![];
                for literal in values {
                    ret.extend(self.gen_literal(literal)?);
                }
                Ok(ret)
            }
        }
    }

    /// Generates VM instructions to push a variable's onto the VM stack
    fn gen_variable(&self, name: &str) -> Result<Vec<VmInstruction>, CalError> {
        if let Some(entry) = self.get_current_symbol_table().get(name) {
            let mut ret = vec![];
            let word_count = self.get_type_size_in_words(&entry.variable.typ);
            for i in 0..word_count {
                ret.push(VmInstruction::Push(entry.segment, entry.offset + i));
            }
            Ok(ret)
        } else {
            Err(CalError::new(
                format!("Undefined variable `{}`", name),
                Range::default(),
            ))
        }
    }

    /// Generates VM Instruction needed to index into an array, after the
    /// index expression has been already generated
    fn gen_index_ref_impl(
        entry: &SymbolEntry,
        elem_size: u16,
    ) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];

        if elem_size > 1 {
            ret.extend(vec![
                VmInstruction::Push(Segment::Constant, elem_size),
                VmInstruction::Call("mul".into(), 2),
            ]);
        }

        ret.extend(vec![
            VmInstruction::Push(Segment::Constant, entry.offset),
            VmInstruction::Add, // offset + index expression
            VmInstruction::Push(Segment::Constant, entry.segment.get_base_address() as u16),
            VmInstruction::Pop(Segment::Pointer, 0),
            VmInstruction::Push(Segment::This, 0),
            VmInstruction::Add, // *segment + offset + index expression
        ]);

        Ok(ret)
    }

    /// Generate VM instructions to index into an array.
    /// - `reference`:
    ///   - `true`: Pushes onto the stack the pointer to the element we are accessing
    ///   - `false`: Pushes onto the stack the element we are accessing
    fn gen_index(
        &self,
        name: &str,
        index_expr: &Expression,
        reference: bool,
    ) -> Result<Vec<VmInstruction>, CalError> {
        if let Some(entry) = self.get_current_symbol_table().get(name) {
            let mut ret = self.gen_expression(index_expr)?;
            // At this point the index is at the top of the stack
            // It should be multiplied by the size of the element of the array
            if let Type::Array(elem_type, _) = &entry.variable.typ {
                let elem_size = self.get_type_size_in_words(elem_type.as_ref());
                ret.extend(Self::gen_index_ref_impl(entry, elem_size)?);

                if !reference {
                    // Put array pointer into the pointer segment for accessing it
                    ret.push(VmInstruction::Pop(Segment::Pointer, 0));
                    // Push onto the stack all the words of the element we are indexing
                    for i in 0..elem_size {
                        ret.push(VmInstruction::Push(Segment::This, i));
                    }
                }

                Ok(ret)
            } else {
                Err(CalError::new(
                    format!("Expected array, found {:?}", entry.variable.typ),
                    Range::default(),
                ))
            }
        } else {
            Err(CalError::new(
                format!("Undefined variable `{}`", name),
                Range::default(),
            ))
        }
    }

    /// Generate VM instructions to push the address of a variable onto the stack
    fn gen_variable_ref(&self, name: &str) -> Result<Vec<VmInstruction>, CalError> {
        if let Some((segment, offset)) =
            self.get_current_symbol_table().get_segment_and_offset(name)
        {
            Ok(vec![
                VmInstruction::Push(Segment::Constant, segment.get_base_address() as u16),
                VmInstruction::Pop(Segment::Pointer, 0),
                VmInstruction::Push(Segment::This, 0),
                VmInstruction::Push(Segment::Constant, offset),
                VmInstruction::Add,
            ])
        } else {
            Err(CalError::new(
                format!("Undefined variable `{}`", name),
                Range::default(),
            ))
        }
    }

    fn gen_unary_operator(
        &self,
        unary_op: UnaryOperator,
        rhs: &Term,
    ) -> Result<Vec<VmInstruction>, CalError> {
        match unary_op {
            UnaryOperator::Ref => match rhs {
                Term::Variable(name) => self.gen_variable_ref(name),
                Term::Index(name, index_expr) => self.gen_index(name, index_expr, true),
                _ => Err(CalError::new(
                    format!("Expected variable after `&`, found {:?}", rhs),
                    Range::default(),
                )),
            },
        }
    }

    fn gen_term(&self, term: &Term) -> Result<Vec<VmInstruction>, CalError> {
        match term {
            Term::Literal(literal) => self.gen_literal(literal),
            Term::Call(name, expressions) => {
                let mut ret = vec![];
                for expr in expressions {
                    ret.extend(self.gen_expression(expr)?);
                }
                ret.push(VmInstruction::Call(name.clone(), expressions.len() as u16));
                Ok(ret)
            }
            Term::Index(name, expr) => self.gen_index(name, expr, false),
            Term::Variable(name) => self.gen_variable(name),
            Term::UnaryOp(unary_op, rhs) => self.gen_unary_operator(*unary_op, rhs.as_ref()),
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
            Operator::And => vec![VmInstruction::And],
            Operator::Or => vec![VmInstruction::Or],
            Operator::Mod => vec![VmInstruction::Call(String::from("mod"), 2)],
            _ => unimplemented!(),
        }
    }

    /// Generates VM instructions to copy the stack backwards into the memory
    /// segment representing a certain variable
    pub fn gen_copy_stack_into_variable(
        &self,
        variable: &Variable,
        segment: Segment,
        offset: u16,
    ) -> Result<Vec<VmInstruction>, CalError> {
        let mut ret = vec![];

        if let Type::Ref(typ) = &variable.typ {
            ret.push(VmInstruction::Push(segment, offset));
            ret.push(VmInstruction::Pop(Segment::Pointer, 0));
            let word_count = self.get_type_size_in_words(typ.as_ref());
            for i in 0..word_count {
                ret.push(VmInstruction::Pop(Segment::This, word_count - i - 1));
            }
        } else {
            // We need to copy the stack backwards according to the size of the variable
            let word_count = self.get_type_size_in_words(&variable.typ);
            for i in 0..word_count {
                ret.push(VmInstruction::Pop(segment, offset + word_count - i - 1));
            }
        }
        Ok(ret)
    }

    pub fn gen_assign_expression_to_variable(
        &self,
        name: &str,
        rhs: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        if let Some(entry) = self.get_current_symbol_table().get(name) {
            let mut ret = self.gen_expression(rhs)?;
            ret.extend(self.gen_copy_stack_into_variable(
                &entry.variable,
                entry.segment,
                entry.offset,
            )?);
            Ok(ret)
        } else {
            Err(CalError::new(
                format!("Undefined variable `{}`", name),
                Range::default(),
            ))
        }
    }

    /// Generates VM instructions to copy the stack backwards into the memory
    /// segment representing a certain index term
    pub fn gen_copy_stack_into_index(
        &self,
        entry: &SymbolEntry,
        index_expr: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        // Push index expression onto the stack
        let mut ret = self.gen_expression(index_expr)?;

        if let Type::Ref(_) = &entry.variable.typ {
            // Reference is already a pointer to the beginning of the array
            ret.push(VmInstruction::Push(entry.segment, entry.offset));
            ret.push(VmInstruction::Add); // pointer + index expression
            ret.push(VmInstruction::Pop(Segment::Pointer, 0)); // put address in pointer
            ret.push(VmInstruction::Pop(Segment::This, 0)); // put rhs in *pointer
        } else {
            ret.extend(vec![
                VmInstruction::Push(Segment::Constant, entry.offset),
                VmInstruction::Add, // offset + index expression
                VmInstruction::Push(Segment::Constant, entry.segment.get_base_address() as u16),
                VmInstruction::Pop(Segment::Pointer, 0),
                VmInstruction::Push(Segment::This, 0),
                VmInstruction::Add, // *segment + offset + expression
                VmInstruction::Pop(Segment::Pointer, 0),
                VmInstruction::Pop(Segment::This, 0), // *pointer = rhs
            ]);
        }
        Ok(ret)
    }

    pub fn gen_assign_expression_to_index(
        &self,
        name: &str,
        index_expr: &Expression,
        rhs: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        if let Some(entry) = self.get_current_symbol_table().get(name) {
            // Push rhs onto the stack
            let mut ret = self.gen_expression(rhs)?;
            ret.extend(self.gen_copy_stack_into_index(entry, index_expr)?);
            Ok(ret)
        } else {
            Err(CalError::new(
                format!("Undefined variable `{}`", name),
                Range::default(),
            ))
        }
    }

    pub fn gen_assign_expression(
        &self,
        term: &Term,
        rhs: &Expression,
    ) -> Result<Vec<VmInstruction>, CalError> {
        // Get variable name from previous term
        match term {
            Term::Variable(name) => self.gen_assign_expression_to_variable(name, rhs),
            Term::Index(name, index_expr) => {
                self.gen_assign_expression_to_index(name, index_expr, rhs)
            }
            _ => Err(CalError::new(
                format!("Expected variable to the left of `=`, found {:?}", term),
                Range::default(),
            )),
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
        ret.extend(self.gen_copy_stack_into_variable(variable, Segment::Local, offset)?);
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

    /// Calculates and returns the size of the local segment
    fn get_local_size_in_words(&self, statements: &[Statement]) -> u16 {
        let mut ret = 0;
        for s in statements {
            if let Statement::Let(variable, _) = s {
                ret += self.get_type_size_in_words(&variable.typ);
            }
        }
        ret
    }

    /// Generates VM instructions for a function
    pub fn gen_function(&mut self, function: &Function) -> Result<Vec<VmInstruction>, CalError> {
        // New symbol table
        self.symbol_tables.push(SymbolTable::default());

        let local_size_in_words = self.get_local_size_in_words(&function.body_statements);

        let mut ret = vec![VmInstruction::Function(
            function.name.clone(),
            local_size_in_words,
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
