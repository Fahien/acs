// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError,
    tokenizer::{Keyword, Symbol, Tokenize},
};

#[test]
fn hello_void() -> Result<(), CalError> {
    let mut tokens = "fn main() {}".tokenize();
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn return_zero() -> Result<(), CalError> {
    let mut tokens = "fn main() -> i16 { 0 }".tokenize();
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_integer(0)?;
    tokens.eat_symbol(Symbol::RightBrace)?;

    let mut tokens = "fn main() -> i16 { return 0; }".tokenize();
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::Return)?;
    tokens.eat_integer(0)?;
    tokens.eat_symbol(Symbol::Semicolon)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}
