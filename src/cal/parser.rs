// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
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

    fn parse_identifier(&mut self) -> Result<String, String> {
        let token = self.tokens.next();
        if let Some(Token::Identifier(id)) = token {
            Ok(id)
        } else {
            Err(format!("Failed to parse identifier, found {:?}", token))
        }
    }

    pub fn parse_function(&mut self) -> Result<Function, String> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_void() -> Result<(), String> {
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
