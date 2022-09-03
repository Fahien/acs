// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{ops::BitOrAssign, str::FromStr};

#[repr(u16)]
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum AsmInstruction {
    /// A-instructions start with `0`
    /// - symbolic: `@n` where `n` is value in range `[0, 32767]`
    A(u16),
    /// C-instructions start with `111`
    /// - symbolic: `dest = comp; jump`
    ///   - `comp` is mandatory
    ///   - `dest` can be empty and `=` is omitted
    ///   - `jump` can be empty and `;` is omitted
    C(Dest, Comp, Jump),
    Label(String),
    Symbol(String),
}

impl Default for AsmInstruction {
    fn default() -> Self {
        Self::A(0)
    }
}

impl From<&AsmInstruction> for u16 {
    fn from(inst: &AsmInstruction) -> Self {
        match inst {
            AsmInstruction::A(value) => value & 0b0111_1111_1111_1111,
            AsmInstruction::C(dest, comp, jump) => {
                0b1110_0000_0000_0000 | *dest as u16 | *comp as u16 | *jump as u16
            }
            AsmInstruction::Label(label) => {
                panic!("Can not convert label to machine instruction: {}", label)
            }
            AsmInstruction::Symbol(symbol) => {
                panic!("Can not convert symbol to machine instruction: {}", symbol)
            }
        }
    }
}

/// Where to store `Comp`.
/// Format is this: `**** **** **dd d***`
#[repr(u16)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Dest {
    /// Value is not stored
    Null = 0 << 3,
    /// `RAM[A]`
    M = 1 << 3,
    /// D register
    D = 2 << 3,
    /// D register and `RAM[A]`
    DM = 3 << 3,
    /// A register
    A = 4 << 3,
    /// A register and `RAM[A]`
    AM = 5 << 3,
    /// A and D register
    AD = 6 << 3,
    /// A and D register and `RAM[A]`
    AMD = 7 << 3,
}

impl From<Dest> for u16 {
    fn from(dest: Dest) -> Self {
        unsafe { std::mem::transmute(dest) }
    }
}

impl From<u16> for Dest {
    fn from(n: u16) -> Self {
        unsafe { std::mem::transmute(n & 0b0000_0000_0011_1000) }
    }
}

impl BitOrAssign for Dest {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Dest::from(u16::from(*self) | u16::from(rhs));
    }
}

impl FromStr for Dest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dest = Dest::Null;
        if s.contains('M') {
            dest |= Dest::M;
        }
        if s.contains('D') {
            dest |= Dest::D;
        }
        if s.contains('A') {
            dest |= Dest::A;
        }
        Ok(dest)
    }
}

/// Comp part of a C-instruction.
/// Format is this: `***a cccc cc** ****`
#[repr(u16)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Comp {
    Zero = 0b0_1010_10 << 6,
    One = 0b0_1111_11 << 6,
    MinusOne = 0b0_1110_10 << 6,
    D = 0b0_0011_00 << 6,
    A = 0b0_1100_00 << 6,
    M = 0b1_1100_00 << 6,
    NotD = 0b0_0011_01 << 6,
    NotA = 0b0_1100_01 << 6,
    NotM = 0b1_1100_01 << 6,
    MinusD = 0b0_0011_11 << 6,
    MinusA = 0b0_1100_11 << 6,
    MinusM = 0b1_1100_11 << 6,
    DPlusOne = 0b0_0111_11 << 6,
    APlusOne = 0b0_1101_11 << 6,
    MPlusOne = 0b1_1101_11 << 6,
    DMinusOne = 0b0_0011_10 << 6,
    AMinusOne = 0b0_1100_10 << 6,
    MMinusOne = 0b1_1100_10 << 6,
    DPlusA = 0b0_0000_10 << 6,
    DPlusM = 0b1_0000_10 << 6,
    DMinusA = 0b0_0100_11 << 6,
    DMinusM = 0b1_0100_11 << 6,
    AMinusD = 0b0_0001_11 << 6,
    MMinusD = 0b1_0001_11 << 6,
    DAndA = 0b0_0000_00 << 6,
    DAndM = 0b1_0000_00 << 6,
    DOrA = 0b0_0101_01 << 6,
    DOrM = 0b1_0101_01 << 6,
}

impl From<u16> for Comp {
    fn from(n: u16) -> Self {
        unsafe { std::mem::transmute(n & 0b0001_1111_1100_0000) }
    }
}

impl FromStr for Comp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::MinusOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "M" => Ok(Comp::M),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "!M" => Ok(Comp::NotM),
            "-D" => Ok(Comp::MinusD),
            "-A" => Ok(Comp::MinusA),
            "-M" => Ok(Comp::MinusM),
            "D+1" => Ok(Comp::DPlusOne),
            "A+1" => Ok(Comp::APlusOne),
            "M+1" => Ok(Comp::MPlusOne),
            "D-1" => Ok(Comp::DMinusOne),
            "A-1" => Ok(Comp::AMinusOne),
            "M-1" => Ok(Comp::MMinusOne),
            "D+A" => Ok(Comp::DPlusA),
            "D+M" => Ok(Comp::DPlusM),
            "D-A" => Ok(Comp::DMinusA),
            "D-M" => Ok(Comp::DMinusM),
            "A-D" => Ok(Comp::AMinusD),
            "M-D" => Ok(Comp::MMinusD),
            "D&A" => Ok(Comp::DAndA),
            "D&M" => Ok(Comp::DAndM),
            "D|A" => Ok(Comp::DOrA),
            "D|M" => Ok(Comp::DOrM),
            _ => Err(format!("Invalid comp string `{}`", s)),
        }
    }
}

/// Jump part of a C-instruction.
/// Format is: `**** **** **** *jjj`
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Jump {
    /// No jump
    No = 0,
    /// Jump if `comp > 0`
    Gt = 1,
    /// Jump if `comp == 0`
    Eq = 2,
    /// Jump if `comp >= 0`
    Ge = 3,
    /// Jump if `comp < 0`
    Lt = 4,
    /// Jump if `comp != 0`
    Ne = 5,
    /// Jump if `comp <= 0`
    Le = 6,
    /// Unconditional jump
    Jump = 7,
}

impl From<u16> for Jump {
    fn from(n: u16) -> Self {
        unsafe { std::mem::transmute(n as u8 & 0b0000_0111) }
    }
}

impl FromStr for Jump {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Jump::No),
            "JGT" => Ok(Jump::Gt),
            "JEQ" => Ok(Jump::Eq),
            "JGE" => Ok(Jump::Ge),
            "JLT" => Ok(Jump::Lt),
            "JNE" => Ok(Jump::Ne),
            "JLE" => Ok(Jump::Le),
            "JMP" => Ok(Jump::Jump),
            _ => Err(format!("Invalid jump string: `{}`", s)),
        }
    }
}

impl From<u16> for AsmInstruction {
    fn from(i: u16) -> Self {
        // last bit is not set
        let is_a_instruction = (!i & 0b1000_0000_0000_0000) != 0;

        // last 3 bits are set
        let is_c_instruction = (i & 0b10_0000_0000_0000 != 0)
            && (i & 0b100_0000_0000_0000 != 0)
            && (i & 0b1000_0000_0000_0000 != 0);

        if is_a_instruction {
            Self::A(i)
        } else if is_c_instruction {
            Self::C(i.into(), i.into(), i.into())
        } else {
            panic!("Failed to convert {:b} to asm instruction", i);
        }
    }
}

impl std::fmt::Binary for AsmInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&u16::from(self), f)
    }
}

impl FromStr for AsmInstruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(symbol_with_suffix) = line.strip_prefix('(') {
            // Label
            if let Some(symbol) = symbol_with_suffix.strip_suffix(')') {
                Ok(AsmInstruction::Label(symbol.into()))
            } else {
                Err("Missing `)` for label".into())
            }
        } else if let Some(literal_str) = line.strip_prefix('@') {
            // A instruction
            // Check whether there is a number to the right of the @
            if let Ok(literal) = literal_str.parse() {
                Ok(AsmInstruction::A(literal))
            } else {
                Ok(AsmInstruction::Symbol(literal_str.into()))
            }
        } else {
            // C instruction: dest = comp ; jump
            let (equal_index, comp_start) = if let Some(index) = line.find('=') {
                (index, index + 1)
            } else {
                (0, 0)
            };
            let (semicolon_index, jump_start) = if let Some(index) = line.find(';') {
                (index, index + 1)
            } else {
                (line.len(), line.len())
            };

            // Dest is optional, in that case = is omitted
            let dest_str = &line[0..equal_index];
            let dest = dest_str.parse().unwrap();

            // Jump is optional, in that case ; is omitted
            let jump_str = &line[jump_start..];
            let jump = jump_str.parse().unwrap();

            // Comp is mandatory
            let comp_str = &line[comp_start..semicolon_index];
            let comp: Comp = comp_str.parse().unwrap();

            Ok(AsmInstruction::C(dest, comp, jump))
        }
    }
}
