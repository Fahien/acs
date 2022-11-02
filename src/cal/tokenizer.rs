// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::error::CalError;

/// Useful for debugging errors in the code, the range will tell us where
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn from_str(substr: &str, str: &str) -> Self {
        let start = substr.as_ptr() as usize - str.as_ptr() as usize;
        let end = start + substr.len();
        Self::new(start, end)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Function,
}

impl Keyword {
    pub const MAP: [(&'static str, Keyword); 1] = [("fn ", Keyword::Function)];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    /// '('
    LeftParen,
    /// ')'
    RightParen,
    /// '{'
    LeftBrace,
    /// '}'
    RightBrace,
}

/// We have various kinds of tokens
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    /// Sequence of letters, digits, and underscore, not starting with digit
    Identifier(String),
}

/// Useful for lexical analysys, with the tokenizer we transform series of
/// characters into tokens to feed to the parser
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub value: TokenKind,
    pub range: Range,
}

impl Token {
    pub fn new(value: TokenKind, range: Range) -> Self {
        Self { value, range }
    }
}

/// Tries to strip a keyword from the input and, if succedes, returns
/// the keyword and the new string stripped of that keyword
fn strip_keyword(input: &str) -> Option<(Keyword, &str)> {
    for (prefix, keyword) in &Keyword::MAP {
        if let Some(stripped) = input.strip_prefix(prefix) {
            return Some((*keyword, stripped));
        }
    }
    None
}

/// Tries to strip a symbol from the input and, if succedes, returns
/// the symbol and the new string stripped of that symbol
fn strip_symbol(input: &str) -> Option<(Symbol, &str)> {
    match input.chars().next() {
        Some('(') => Some((Symbol::LeftParen, &input[1..])),
        Some(')') => Some((Symbol::RightParen, &input[1..])),
        Some('{') => Some((Symbol::LeftBrace, &input[1..])),
        Some('}') => Some((Symbol::RightBrace, &input[1..])),
        _ => None,
    }
}

/// Tries to strip an identifier from the input and, if succedes, returns
/// the identifier and the new string stripped of that identifier
fn strip_identifier(input: &str) -> Option<(&str, &str)> {
    if let Some(first_char) = input.chars().next() {
        if first_char.is_alphabetic() || first_char == '_' {
            let mut cut_index = 0;

            for c in input.chars() {
                if c.is_alphanumeric() || c == '_' {
                    cut_index += 1;
                } else {
                    break;
                }
            }

            return Some(input.split_at(cut_index));
        }
    }
    None
}

/// This struct behaves like a peekable iterator of tokens with methods to _eat_
/// tokens and effectively advance the iterator
pub struct Tokens {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Tokens {
    /// Converts a string into a vector of tokens
    fn tokenize(code: &str) -> Result<Vec<Token>, CalError> {
        let mut ret = vec![];

        // Ignore white spaces at the beginning
        let mut input = code.trim_start();

        while !input.is_empty() {
            // Ignore comments
            if input.starts_with("//") {
                if let Some((_, stripped_input)) = input.split_once('\n') {
                    input = stripped_input;
                } else {
                    input = "";
                }
            } else if let Some((keyword, stripped_input)) = strip_keyword(input) {
                // Check for keywords
                input = stripped_input;
                ret.push(Token::new(
                    TokenKind::Keyword(keyword),
                    Range::from_str(input, code),
                ));
            } else if let Some((symbol, stripped_input)) = strip_symbol(input) {
                input = stripped_input;
                ret.push(Token::new(
                    TokenKind::Symbol(symbol),
                    Range::from_str(input, code),
                ));
            } else if let Some((identifier, stripped_input)) = strip_identifier(input) {
                input = stripped_input;
                ret.push(Token::new(
                    TokenKind::Identifier(identifier.into()),
                    Range::from_str(input, code),
                ));
            } else {
                return Err(CalError::new(
                    format!("Failed to parse {}", input),
                    Range::from_str(input, code),
                ));
            }

            // Ignore white spaces after stripping
            input = input.trim_start();
        }

        Ok(ret)
    }

    pub fn new(input: &str) -> Result<Self, CalError> {
        Ok(Self {
            tokens: Self::tokenize(input)?.into_iter().peekable(),
        })
    }

    /// Eats a keyword and advances to the next token
    pub fn eat_keyword(&mut self, keyword: Keyword) -> Result<(), CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Keyword(kw) if *kw == keyword => Ok(()),
                _ => Err(CalError::new(
                    format!(
                        "Expected keyword: {:?}, found {:?}",
                        keyword, token.value
                    ),
                    token.range,
                )),
            }
        } else {
            Err(CalError::new(
                format!("Expected keyword {:?}", keyword),
                Range::default(),
            ))
        }
    }

    /// Eats a symbol and advances to the next token
    pub fn eat_symbol(&mut self, symbol: Symbol) -> Result<(), CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Symbol(sym) if *sym == symbol => Ok(()),
                _ => Err(CalError::new(
                    format!(
                        "Expected symbol: {:?}, found {:?}",
                        symbol, token.value
                    ),
                    token.range,
                )),
            }
        } else {
            Err(CalError::new(
                format!("Expected symbol {:?}", symbol),
                Range::default(),
            ))
        }
    }

    /// Eats an identifier and advances to the next token
    pub fn eat_identifier(&mut self, ident: &str) -> Result<(), CalError> {
        if let Some(token) = self.tokens.next() {
            match &token.value {
                TokenKind::Identifier(id) if *id == ident => Ok(()),
                _ => Err(CalError::new(
                    format!("Expected identifier: {:?}, found {:?}", ident, token.value),
                    token.range,
                )),
            }
        } else {
            Err(CalError::new(
                format!("Expected identifier {:?}", ident),
                Range::default(),
            ))
        }
    }
}

impl Deref for Tokens {
    type Target = Peekable<std::vec::IntoIter<Token>>;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for Tokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

pub fn tokenize(code: &str) -> Result<Tokens, CalError> {
    Tokens::new(code)
}

pub trait Tokenize {
    fn tokenize(&self) -> Result<Tokens, CalError>;
}

impl Tokenize for str {
    fn tokenize(&self) -> Result<Tokens, CalError> {
        tokenize(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
