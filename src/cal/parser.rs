// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
    error::CalError,
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

    pub fn parse_function(&mut self) -> Result<Function, CalError> {
        self.tokens.eat_keyword(Keyword::Function)?;

        let name = self.parse_identifier()?;

        self.tokens.eat_symbol(Symbol::LeftParen)?;
        // TODO parse parameters
        let parameters = vec![];
        self.tokens.eat_symbol(Symbol::RightParen)?;

        // TODO: Parse return type
        let return_type = Type::Void;

        self.tokens.eat_symbol(Symbol::LeftBrace)?;
        // TODO parse statements
        let body_statements = vec![];
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_void() -> Result<(), CalError> {
        let module: Module = "fn main() {}".parse()?;
        assert_eq!(module.functions.len(), 1);
        let function = &module.functions[0];
        assert_eq!(function.name, "main");
        assert!(function.parameters.is_empty());
        assert!(function.body_statements.is_empty());
        assert_eq!(function.local_count, 0);
        assert_eq!(function.return_type, Type::Void);
        Ok(())
    }
}
