// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::Signal15;

/// 13 signals into one
#[derive(Copy, Clone, Default)]
pub struct Signal13 {
    pub values: i16,
}

impl Signal13 {
    /// All signals `HI`
    pub const TRUE: Self = Self {
        values: 0b1_1111_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Self = Self { values: 0 };

    pub fn new(values: i16) -> Self {
        Self {
            values: values & 0b1_1111_1111_1111,
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

impl From<i16> for Signal13 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Signal13 {
    fn from(value: i32) -> Self {
        Self::from(value as i16)
    }
}

impl From<Signal15> for Signal13 {
    fn from(signal: Signal15) -> Self {
        Self::new(signal.values)
    }
}

impl From<Signal13> for u16 {
    fn from(signal: Signal13) -> Self {
        unsafe { std::mem::transmute(signal.values) }
    }
}

impl From<Signal13> for i16 {
    fn from(signal: Signal13) -> i16 {
        signal.values
    }
}

impl From<Signal13> for usize {
    fn from(signal: Signal13) -> usize {
        unsafe { std::mem::transmute::<i16, u16>(signal.values) as usize }
    }
}

impl Not for Signal13 {
    type Output = Signal13;

    fn not(self) -> Self::Output {
        let values = !self.values;
        Signal13::new(values)
    }
}

impl BitAnd for Signal13 {
    type Output = Signal13;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl PartialEq for Signal13 {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Debug for Signal13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}

impl Display for Signal13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.values, f)
    }
}
