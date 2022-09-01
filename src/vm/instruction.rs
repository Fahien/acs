// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{code::VmCode, preprocessor::VmPreprocessedCode, segment::Segment};

/// Sort of _intermediate code_ designed to run on a
/// [stack machine](https://en.wikipedia.org/wiki/Stack_machine).
/// Part of a two-tier compilation model, where a high level language is
/// first translated to this representation and then to machine language.
#[derive(Copy, Clone, Debug)]
pub enum VmInstruction {
    /// Pushes `segment[index]` on top of the stack
    Push(Segment, u16),

    /// Pops an element from the stack and stores it into `segment[index]`
    Pop(Segment, u16),

    /// Pops `y`, pops `x`, and pushes `x + y`
    Add,

    /// Pops `y`, pops `x`, and pushes `x - y`
    Sub,

    /// Pops `y`, pops `x`, and pushes `x == y`
    Eq,

    /// Pops `y`, pops `x`, and pushes `x < y`
    Lt,

    /// Pops `y`, pops `x`, and pushes `x > y`
    Gt,

    /// Pops `x`, and pushes `-x`
    Neg,

    /// Pops `y`, pops `x`, and pushes `x & y`
    And,

    /// Pops `y`, pops `x`, and pushes `x | y`
    Or,

    /// Pops `x`, and pushes `~x` (bitwise not)
    Not,
}

impl VmInstruction {
    pub fn parse(code: &str) -> Vec<VmInstruction> {
        VmPreprocessedCode::builder()
            .include(VmCode::new("default", code))
            .build()
            .into()
    }
}

impl FromStr for VmInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');

        let command = words.next().unwrap();

        match command {
            "push" => {
                let segment = words.next().unwrap().parse().unwrap();
                let value = words.next().unwrap().parse().unwrap();
                Ok(VmInstruction::Push(segment, value))
            }
            "pop" => {
                let segment = words.next().unwrap().parse().unwrap();
                let value = words.next().unwrap().parse().unwrap();
                Ok(VmInstruction::Pop(segment, value))
            }
            "add" => Ok(VmInstruction::Add),
            "sub" => Ok(VmInstruction::Sub),
            "eq" => Ok(VmInstruction::Eq),
            "lt" => Ok(VmInstruction::Lt),
            "gt" => Ok(VmInstruction::Gt),
            "neg" => Ok(VmInstruction::Neg),
            "and" => Ok(VmInstruction::And),
            "or" => Ok(VmInstruction::Or),
            "not" => Ok(VmInstruction::Not),
            _ => Err(format!("Invalid command {}", command)),
        }
    }
}

impl From<VmPreprocessedCode> for Vec<VmInstruction> {
    fn from(preprocessed_code: VmPreprocessedCode) -> Self {
        preprocessed_code
            .get_lines()
            .iter()
            .map(|line| line.parse().unwrap())
            .collect()
    }
}

impl From<VmCode> for Vec<VmInstruction> {
    fn from(code: VmCode) -> Self {
        VmPreprocessedCode::builder().include(code).build().into()
    }
}
