// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError,
    tokenizer::{Keyword, Symbol, Tokenize},
};

#[test]
fn hello_void() -> Result<(), CalError> {
    let mut tokens = "fn main() {}".tokenize()?;
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
    let mut tokens = "fn main() -> i16 { 0 }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_integer(0)?;
    tokens.eat_symbol(Symbol::RightBrace)?;

    let mut tokens = "fn main() -> i16 { return 0; }".tokenize()?;
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

#[test]
fn def_local() -> Result<(), CalError> {
    let mut tokens = "fn def_local() { let x: i16 = 0; let y: i16 = 1; }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("def_local")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;

    tokens.eat_keyword(Keyword::Let)?;
    tokens.eat_identifier("x")?;
    tokens.eat_symbol(Symbol::Colon)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::Assign)?;
    tokens.eat_integer(0)?;
    tokens.eat_symbol(Symbol::Semicolon)?;

    tokens.eat_keyword(Keyword::Let)?;
    tokens.eat_identifier("y")?;
    tokens.eat_symbol(Symbol::Colon)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::Assign)?;
    tokens.eat_integer(1)?;
    tokens.eat_symbol(Symbol::Semicolon)?;
    Ok(())
}

#[test]
fn one_parameter() -> Result<(), CalError> {
    let mut tokens = "fn identity(x: i16) -> i16 { x }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("identity")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_identifier("x")?;
    tokens.eat_symbol(Symbol::Colon)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_identifier("x")?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn multi_parameters() -> Result<(), CalError> {
    let mut tokens = "fn ignore_y(x: i16, y: i16) -> i16 { x }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("ignore_y")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_identifier("x")?;
    tokens.eat_symbol(Symbol::Colon)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::Comma)?;
    tokens.eat_identifier("y")?;
    tokens.eat_symbol(Symbol::Colon)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::I16)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_identifier("x")?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn call_function() -> Result<(), CalError> {
    let mut tokens = "fn main() { call() }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_identifier("call")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn add() -> Result<(), CalError> {
    let mut tokens = "fn main() { 1 + 2; }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_integer(1)?;
    tokens.eat_symbol(Symbol::Plus)?;
    tokens.eat_integer(2)?;
    tokens.eat_symbol(Symbol::Semicolon)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn if_statement() -> Result<(), CalError> {
    let mut tokens = "fn main() -> bool { if true { true } else { false } }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::Bool)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::If)?;
    tokens.eat_keyword(Keyword::True)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::True)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    tokens.eat_keyword(Keyword::Else)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::False)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}

#[test]
fn while_statement() -> Result<(), CalError> {
    let mut tokens = "fn main() -> bool { while true { return true; } false }".tokenize()?;
    tokens.eat_keyword(Keyword::Function)?;
    tokens.eat_identifier("main")?;
    tokens.eat_symbol(Symbol::LeftParen)?;
    tokens.eat_symbol(Symbol::RightParen)?;
    tokens.eat_symbol(Symbol::RightArrow)?;
    tokens.eat_keyword(Keyword::Bool)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::While)?;
    tokens.eat_keyword(Keyword::True)?;
    tokens.eat_symbol(Symbol::LeftBrace)?;
    tokens.eat_keyword(Keyword::Return)?;
    tokens.eat_keyword(Keyword::True)?;
    tokens.eat_symbol(Symbol::Semicolon)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    tokens.eat_keyword(Keyword::False)?;
    tokens.eat_symbol(Symbol::RightBrace)?;
    Ok(())
}
