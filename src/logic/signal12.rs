// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

/// 12 signals into one
#[derive(Copy, Clone, Default)]
pub struct Signal12 {
    values: i16,
}

impl Signal12 {
    /// All signals `HI`
    pub const TRUE: Signal12 = Signal12 {
        values: 0b1111_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Signal12 = Signal12 { values: 0 };

    pub fn new(values: i16) -> Self {
        Self {
            values: values & 0b1111_1111_1111,
        }
    }

    pub fn as_usize(&self) -> usize {
        self.values as usize
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

impl From<i16> for Signal12 {
    fn from(value: i16) -> Self {
        Signal12::new(value)
    }
}

impl From<i32> for Signal12 {
    fn from(value: i32) -> Self {
        Self::from(value as i16)
    }
}

impl From<Signal12> for i16 {
    fn from(signal: Signal12) -> i16 {
        signal.values
    }
}

impl Not for Signal12 {
    type Output = Signal12;

    fn not(self) -> Self::Output {
        Signal12::new(!self.values)
    }
}

impl BitAnd for Signal12 {
    type Output = Signal12;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl PartialEq for Signal12 {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Debug for Signal12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}

impl Display for Signal12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.values, f)
    }
}
