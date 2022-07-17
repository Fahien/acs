// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

/// 8 signals into one
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Signal8 {
    pub values: i8,
}

impl Signal8 {
    #[allow(overflowing_literals)]
    /// All signals `HI`
    pub const TRUE: Signal8 = Signal8 {
        values: 0b1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Signal8 = Signal8 { values: 0 };

    pub fn new(values: i8) -> Self {
        Self { values }
    }

    pub fn get(&self, index: usize) -> i8 {
        assert!(index < 8);
        (self.values & (1 << index)) >> index
    }
}

impl From<u8> for Signal8 {
    fn from(value: u8) -> Self {
        Self::new(value as i8)
    }
}

impl From<i16> for Signal8 {
    fn from(value: i16) -> Self {
        Self::new(value as i8)
    }
}

impl From<u16> for Signal8 {
    fn from(value: u16) -> Self {
        Self::new(value as i8)
    }
}

impl From<i32> for Signal8 {
    fn from(value: i32) -> Self {
        Self::new(value as i8)
    }
}

impl From<&Signal8> for i8 {
    fn from(signal: &Signal8) -> Self {
        signal.values as i8
    }
}

impl Not for Signal8 {
    type Output = Signal8;

    fn not(self) -> Self::Output {
        Self::new(!self.values)
    }
}

impl BitAnd for Signal8 {
    type Output = Signal8;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl Debug for Signal8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:08b}", self.values))
    }
}

impl Display for Signal8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:08b}", self.values))
    }
}
