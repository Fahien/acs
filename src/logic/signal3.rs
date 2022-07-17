// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

/// 3 signals into one
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Signal3 {
    values: i8,
}

impl Signal3 {
    /// All signals `HI`
    pub const TRUE: Signal3 = Signal3 { values: 0b111 };

    /// All signals `LO`
    pub const FALSE: Signal3 = Signal3 { values: 0 };

    pub fn new(values: i8) -> Self {
        Self {
            values: values & 0b111,
        }
    }

    pub fn get(&self, index: usize) -> i8 {
        assert!(index < 3);
        let mask = 1 << index;
        (self.values & mask) >> index
    }
}

impl From<u16> for Signal3 {
    fn from(value: u16) -> Self {
        Self::new(value as i8)
    }
}

impl From<i32> for Signal3 {
    fn from(value: i32) -> Self {
        Self::new(value as i8)
    }
}

impl From<Signal3> for i8 {
    fn from(signal: Signal3) -> Self {
        signal.values
    }
}

impl From<Signal3> for u8 {
    fn from(signal: Signal3) -> Self {
        signal.values as u8
    }
}

impl Not for Signal3 {
    type Output = Signal3;

    fn not(self) -> Self::Output {
        let values = !self.values;
        Signal3::new(values)
    }
}

impl BitAnd for Signal3 {
    type Output = Signal3;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl Debug for Signal3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:03b}", self.values))
    }
}

impl Display for Signal3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:03b}", self.values))
    }
}
