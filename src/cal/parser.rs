// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
    error::CalError,
    expression::{Expression, Operator, Term},
    statement::{IfStatement, Statement},
    structure::{Function, Module, Type, Variable},
    tokenizer::*,
};

pub struct Parser {
    tokens: Tokens,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Parser {
        Self { tokens }
    }

    fn parse_identifier(&mut self) -> Result<String, CalError> {
        if let Some(token) = self.tokens.next() {
            if let TokenKind::Identifier(id) = token.value {
                Ok(id)
            } else {
                Err(CalError {
                    message: format!("Expected identifier, found {:?}", token.value),
                    range: token.range,
                })
            }
        } else {
            Err(CalError::new(
                "Expected identifier".into(),
                Range::default(),
            ))
        }
    }

    fn parse_type(&mut self) -> Result<Type, CalError> {
        if let Some(token) = self.tokens.next() {
            if let TokenKind::Keyword(keyword) = token.value {
                Type::from_keyword(keyword)
            } else {
                Err(CalError::new(
                    format!("Expected type, found {:?}", token.value),
                    token.range,
                ))
            }
        } else {
            Err(CalError::new("Expected type".into(), Range::default()))
        }
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, CalError> {
        let mut expressions = vec![];
        if self.tokens.peek_symbol(Symbol::RightParen) {
            return Ok(expressions);
        }
        expressions.extend(self.parse_expression());
        while self.tokens.peek_symbol(Symbol::Comma) {
            self.tokens.skip();
            expressions.push(self.parse_expression()?);
        }
        Ok(expressions)
    }

    fn parse_term(&mut self) -> Result<Term, CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Keyword(Keyword::True) => Ok(Term::BoolLiteral(true)),
                TokenKind::Keyword(Keyword::False) => Ok(Term::BoolLiteral(false)),
                TokenKind::Integer(int) => Ok(Term::IntLiteral(*int)),
                TokenKind::Identifier(identifier) => {
                    // Parse subroutine call
                    if self.tokens.peek_symbol(Symbol::LeftParen) {
                        self.tokens.skip();
                        let expression_list = self.parse_expression_list()?;
                        self.tokens.eat_symbol(Symbol::RightParen)?;
                        Ok(Term::Call(identifier.clone(), expression_list))
                    } else {
                        Ok(Term::Variable(identifier.clone()))
                    }
                }
                _ => Err(CalError::new(
                    format!("Failed to parse term, found {:?}", token.value),
                    token.range,
                )),
            }
        } else {
            Err(CalError::new("Expected term".into(), Range::default()))
        }
    }

    fn parse_operator(&mut self) -> Result<Operator, CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Symbol(symbol) => Ok(Operator::from_symbol(*symbol)?),
                token_kind => Err(CalError::new(
                    format!("Expected operator, found {:?}", token_kind),
                    Range::default(),
                )),
            }
        } else {
            Err(CalError::new("Expected operator".into(), Range::default()))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CalError> {
        let term = self.parse_term()?;

        // Following a term there can be an operator
        let op_and_exprm = if self.tokens.peek_operator() {
            let op = self.parse_operator()?;
            let expression = self.parse_expression()?;
            Some((op, Box::new(expression)))
        } else {
            None
        };

        Ok(Expression::new(Box::new(term), op_and_exprm))
    }

    fn parse_return(&mut self) -> Result<Option<Expression>, CalError> {
        self.tokens.eat_keyword(Keyword::Return)?;
        if self.tokens.peek_symbol(Symbol::Semicolon) {
            self.tokens.skip();
            Ok(None)
        } else {
            let expression = self.parse_expression()?;
            self.tokens.eat_symbol(Symbol::Semicolon)?;
            Ok(Some(expression))
        }
    }

    pub fn parse_let(&mut self) -> Result<Statement, CalError> {
        self.tokens.eat_keyword(Keyword::Let)?;
        let variable_name = self.parse_identifier()?;
        self.tokens.eat_symbol(Symbol::Colon)?;
        let variable_type = self.parse_type()?;
        let variable = Variable::new(variable_name, variable_type);
        self.tokens.eat_symbol(Symbol::Assign)?;
        let assign_expression = self.parse_expression()?;
        self.tokens.eat_symbol(Symbol::Semicolon)?;
        Ok(Statement::Let(variable, assign_expression))
    }

    pub fn parse_if(&mut self) -> Result<Statement, CalError> {
        self.tokens.eat_keyword(Keyword::If)?;
        let predicate = self.parse_expression()?;
        self.tokens.eat_symbol(Symbol::LeftBrace)?;
        let if_branch = self.parse_statements()?;
        self.tokens.eat_symbol(Symbol::RightBrace)?;

        let mut else_branch = vec![];
        if self.tokens.peek_keyword(Keyword::Else) {
            self.tokens.eat_keyword(Keyword::Else)?;
            self.tokens.eat_symbol(Symbol::LeftBrace)?;
            else_branch.extend(self.parse_statements()?);
            self.tokens.eat_symbol(Symbol::RightBrace)?;
        }

        Ok(Statement::If(IfStatement::new(
            predicate,
            if_branch,
            else_branch,
        )))
    }

    pub fn parse_statement(&mut self) -> Result<Option<Statement>, CalError> {
        if let Some(token) = self.tokens.peek().cloned() {
            match &token.value {
                TokenKind::Symbol(Symbol::Semicolon | Symbol::RightBrace) => Ok(None),
                TokenKind::Keyword(Keyword::Return) => {
                    Ok(Some(Statement::Return(self.parse_return()?)))
                }
                TokenKind::Keyword(Keyword::Let) => Ok(Some(self.parse_let()?)),
                TokenKind::Keyword(Keyword::If) => Ok(Some(self.parse_if()?)),
                _ => {
                    let expression = self.parse_expression()?;
                    if self.tokens.peek_symbol(Symbol::Semicolon) {
                        self.tokens.skip();
                    } else if !self.tokens.peek_symbol(Symbol::RightBrace) {
                        return Err(CalError::new(
                            format!(
                                "Expected `}}` or `;` after expression, found {:?}",
                                self.tokens.peek()
                            ),
                            token.range,
                        ));
                    }
                    Ok(Some(Statement::Expression(expression)))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, CalError> {
        let mut statements = vec![];
        while let Some(statement) = self.parse_statement()? {
            statements.push(statement);
        }
        Ok(statements)
    }

    /// Returns the number of local variables in the `statements`
    pub fn count_local_variables(statements: &[Statement]) -> usize {
        statements
            .iter()
            .filter(|s| matches!(s, Statement::Let(_, _)))
            .count()
    }

    pub fn parse_parameters(&mut self) -> Result<Vec<Variable>, CalError> {
        let mut ret = vec![];

        while !self.tokens.peek_symbol(Symbol::RightParen) {
            let name = self.parse_identifier()?;
            self.tokens.eat_symbol(Symbol::Colon)?;
            let typ = self.parse_type()?;
            let parameter = Variable::new(name, typ);
            ret.push(parameter);
            if self.tokens.peek_symbol(Symbol::Comma) {
                self.tokens.skip();
            }
        }

        Ok(ret)
    }

    pub fn parse_function(&mut self) -> Result<Function, CalError> {
        self.tokens.eat_keyword(Keyword::Function)?;

        let name = self.parse_identifier()?;

        self.tokens.eat_symbol(Symbol::LeftParen)?;
        let parameters = self.parse_parameters()?;
        self.tokens.eat_symbol(Symbol::RightParen)?;

        let return_type = if self.tokens.peek_symbol(Symbol::RightArrow) {
            self.tokens.skip();
            self.parse_type()?
        } else {
            Type::Void
        };

        self.tokens.eat_symbol(Symbol::LeftBrace)?;
        let body_statements = self.parse_statements()?;
        self.tokens.eat_symbol(Symbol::RightBrace)?;

        let local_count = Self::count_local_variables(&body_statements) as u16;

        Ok(Function {
            return_type,
            name,
            parameters,
            local_count,
            body_statements,
        })
    }

    pub fn parse_module(&mut self) -> Result<Module, CalError> {
        let mut functions = vec![];

        while let Some(token) = self.tokens.peek() {
            match &token.value {
                TokenKind::Keyword(Keyword::Function) => functions.push(self.parse_function()?),
                _ => {
                    return Err(CalError::new(
                        format!("Expected function, found {:?}", token.value),
                        token.range,
                    ))
                }
            }
        }

        Ok(Module::new("main", functions))
    }
}

pub fn parse(tokens: Tokens) -> Result<Module, CalError> {
    Parser::new(tokens).parse_module()
}

impl FromStr for Module {
    type Err = CalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s.tokenize()?)
    }
}
