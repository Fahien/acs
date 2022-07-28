// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::Signal15;

/// 14 signals into one
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Signal14 {
    pub values: i16,
}

impl Signal14 {
    /// All signals `HI`
    pub const TRUE: Self = Self {
        values: 0b11_1111_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Self = Self { values: 0 };

    pub fn new(values: i16) -> Self {
        Self {
            values: values & 0b11_1111_1111_1111,
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

impl From<i16> for Signal14 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Signal14 {
    fn from(value: i32) -> Self {
        Self::new(value as i16)
    }
}

impl From<Signal15> for Signal14 {
    fn from(signal: Signal15) -> Self {
        Self::new(signal.into())
    }
}

impl From<Signal14> for u16 {
    fn from(signal: Signal14) -> u16 {
        unsafe { std::mem::transmute(signal.values) }
    }
}

impl From<Signal14> for i16 {
    fn from(signal: Signal14) -> i16 {
        signal.values
    }
}

impl From<Signal14> for usize {
    fn from(signal: Signal14) -> Self {
        signal.values as Self
    }
}

impl Not for Signal14 {
    type Output = Signal14;

    fn not(self) -> Self::Output {
        Signal14::new(!self.values)
    }
}

impl BitAnd for Signal14 {
    type Output = Signal14;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl Debug for Signal14 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}

impl Display for Signal14 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.values, f)
    }
}
