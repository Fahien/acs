// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

/// 9 signals into one
#[derive(Copy, Clone, Default)]
pub struct Signal9 {
    values: i16,
}

impl Signal9 {
    /// All signals `HI`
    pub const TRUE: Self = Self {
        values: 0b1_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Signal9 = Signal9 { values: 0 };

    pub fn new(values: i16) -> Self {
        Self {
            values: values & 0b1_1111_1111,
        }
    }

    pub fn get(&self, index: Range<usize>) -> i16 {
        let shift = index.start;
        let width = index.len() as u32;
        let mask_unshifted = 2i16.pow(width) - 1;
        let mask = mask_unshifted << shift;
        let ret_unshifted = self.values & mask;
        ret_unshifted >> shift
    }
}

impl From<i16> for Signal9 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Signal9 {
    fn from(value: i32) -> Self {
        Self::from(value as i16)
    }
}

impl From<Signal9> for u16 {
    fn from(signal: Signal9) -> Self {
        signal.values as u16
    }
}

impl From<Signal9> for i8 {
    fn from(signal: Signal9) -> i8 {
        signal.values as i8
    }
}

impl From<Signal9> for i16 {
    fn from(signal: Signal9) -> i16 {
        signal.values as i16
    }
}

impl From<Signal9> for usize {
    fn from(signal: Signal9) -> Self {
        signal.values as Self
    }
}

impl Not for Signal9 {
    type Output = Signal9;

    fn not(self) -> Self::Output {
        let values = !self.values;
        Signal9::new(values)
    }
}

impl BitAnd for Signal9 {
    type Output = Signal9;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl PartialEq for Signal9 {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Debug for Signal9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}

impl Display for Signal9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.values, f)
    }
}
