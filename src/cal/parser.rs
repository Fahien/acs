// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
    expression::{Expression, Term},
    statement::Statement,
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

    fn parse_identifier(&mut self) -> Result<String, String> {
        let token = self.tokens.next();
        if let Some(Token::Identifier(id)) = token {
            Ok(id)
        } else {
            Err(format!("Failed to parse identifier, found {:?}", token))
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let token = self.tokens.next();
        match token {
            Some(Token::Keyword(keyword)) => Ok(Type::from(keyword)),
            _ => Err(format!("Failed to parse type, found {:?}", token)),
        }
    }

    fn parse_term(&mut self) -> Result<Term, String> {
        let token = self.tokens.next();
        match token {
            Some(Token::Integer(int)) => Ok(Term::IntLiteral(int)),
            _ => Err(format!("Failed to parse term, found {:?}", token)),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        let term = self.parse_term()?;
        Ok(Expression::new(Box::new(term)))
    }

    fn parse_return(&mut self) -> Result<Option<Expression>, String> {
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

    pub fn parse_let(&mut self) -> Result<Statement, String> {
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

    pub fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::Symbol(Symbol::Semicolon | Symbol::RightBrace) => Ok(None),
                Token::Keyword(Keyword::Return) => {
                    Ok(Some(Statement::Return(self.parse_return()?)))
                }
                Token::Keyword(Keyword::Let) => Ok(Some(self.parse_let()?)),
                _ => {
                    let expression = self.parse_expression()?;
                    if self.tokens.peek_symbol(Symbol::Semicolon) {
                        self.tokens.skip();
                    } else if !self.tokens.peek_symbol(Symbol::RightBrace) {
                        return Err("Expected `}` or `;` after expression".into());
                    }
                    Ok(Some(Statement::Expression(expression)))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
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

    pub fn parse_function(&mut self) -> Result<Function, String> {
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

        let local_count = Self::count_local_variables(&body_statements) as u16;

        Ok(Function {
            return_type,
            name,
            parameters,
            local_count,
            body_statements,
        })
    }

    pub fn parse_module(&mut self) -> Result<Module, String> {
        let mut functions = vec![];

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Keyword(Keyword::Function) => functions.push(self.parse_function()?),
                _ => return Err(format!("Failed to parse function, found {:?}", token)),
            }
        }

        Ok(Module::new("main", functions))
    }
}

pub fn parse(tokens: Tokens) -> Result<Module, String> {
    Parser::new(tokens).parse_module()
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s.tokenize())
    }
}
