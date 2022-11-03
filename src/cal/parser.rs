// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
    error::CalError,
    expression::{Expression, Term},
    statement::Statement,
    structure::{Function, Module, Type},
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
                Ok(Type::from(keyword))
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

    fn parse_term(&mut self) -> Result<Term, CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Integer(int) => Ok(Term::IntLiteral(*int)),
                _ => Err(CalError::new(
                    format!("Failed to parse term, found {:?}", token.value),
                    token.range,
                )),
            }
        } else {
            Err(CalError::new("Expected term".into(), Range::default()))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CalError> {
        let term = self.parse_term()?;
        Ok(Expression::new(Box::new(term)))
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

    pub fn parse_statement(&mut self) -> Result<Option<Statement>, CalError> {
        if let Some(token) = self.tokens.peek().cloned() {
            match &token.value {
                TokenKind::Symbol(Symbol::Semicolon | Symbol::RightBrace) => Ok(None),
                TokenKind::Keyword(Keyword::Return) => {
                    Ok(Some(Statement::Return(self.parse_return()?)))
                }
                _ => {
                    let expression = self.parse_expression()?;
                    if self.tokens.peek_symbol(Symbol::Semicolon) {
                        self.tokens.skip();
                    } else if !self.tokens.peek_symbol(Symbol::RightBrace) {
                        return Err(CalError::new(
                            "Expected `}` or `;` after expression".into(),
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

    pub fn parse_function(&mut self) -> Result<Function, CalError> {
        self.tokens.eat_keyword(Keyword::Function)?;

        let name = self.parse_identifier()?;

        self.tokens.eat_symbol(Symbol::LeftParen)?;
        // TODO parse parameters
        let parameters = vec![];
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

        // TODO parse local variables count
        let local_count = 0;

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
        parse(s.tokenize())
    }
}
