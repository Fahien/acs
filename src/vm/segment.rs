// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    /// Stack pointer address is at `RAM[0]`
    Stack,

    /// Local address is at `RAM[1]`
    Local,

    /// Arg address is at `RAM[2]`
    Argument,

    /// This address is at `RAM[3]`
    This,

    /// That address is at `RAM[4]`
    That,

    /// Actually not a memory segment
    Constant,

    /// Starts at `RAM[16]`, ends at `RAM[255]` included
    Static,

    /// 8-place segment from `RAM[5]` to `RAM[12]`
    Temp,

    /// Helper registers for the VM translator
    R13,
    R14,
    R15,

    /// Fixed memory segment pushing only `0` or `1` (`This`, `That`)
    Pointer,
}

impl Segment {
    pub fn get_base_address(self) -> usize {
        match self {
            Segment::Stack => 0,
            Segment::Local => 1,
            Segment::Argument => 2,
            Segment::This | Segment::Pointer => 3,
            Segment::That => 4,
            Segment::Temp => 5,
            Segment::R13 => 13,
            Segment::R14 => 14,
            Segment::R15 => 15,
            Segment::Static => 16,
            _ => panic!("Can not get base address for this segment '{:?}'", self),
        }
    }
}

impl FromStr for Segment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Segment::Local),
            "argument" => Ok(Segment::Argument),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "constant" => Ok(Segment::Constant),
            "static" => Ok(Segment::Static),
            "temp" => Ok(Segment::Temp),
            "pointer" => Ok(Segment::Pointer),
            _ => Err(format!("Invalid string segment {}", s)),
        }
    }
}
