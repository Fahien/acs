// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::Signal9;

/// 6 signals into one
#[derive(Copy, Clone, Default)]
pub struct Signal6 {
    pub values: i8,
}

impl Signal6 {
    /// All signals `HI`
    pub const TRUE: Signal6 = Signal6 { values: 0b111111 };

    /// All signals `LO`
    pub const FALSE: Signal6 = Signal6 { values: 0 };

    pub fn new(values: i8) -> Self {
        Self {
            values: values & 0b111111,
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { std::mem::transmute(self.values) }
    }

    pub fn as_usize(&self) -> usize {
        self.values as usize
    }

    pub fn get(&self, index: Range<usize>) -> i8 {
        let shift = index.start;
        let width = index.len() as u32;
        let mask_unshifted = 2i8.pow(width) - 1;
        let mask = mask_unshifted << shift;
        let ret_unshifted = self.values & mask;
        ret_unshifted >> shift
    }
}

impl From<u8> for Signal6 {
    fn from(value: u8) -> Self {
        let value = unsafe { std::mem::transmute(value) };
        Self::new(value)
    }
}

impl From<i32> for Signal6 {
    fn from(value: i32) -> Self {
        Self::new(value as i8)
    }
}

impl From<Signal9> for Signal6 {
    fn from(signal9: Signal9) -> Self {
        Self::new(signal9.into())
    }
}

impl From<&Signal6> for i8 {
    fn from(signal: &Signal6) -> Self {
        signal.values
    }
}

impl From<Signal6> for u8 {
    fn from(signal: Signal6) -> Self {
        unsafe { std::mem::transmute(signal.values) }
    }
}

impl From<Signal6> for usize {
    fn from(signal: Signal6) -> Self {
        signal.values as Self
    }
}

impl Not for Signal6 {
    type Output = Signal6;

    fn not(self) -> Self::Output {
        Signal6::new(!self.values)
    }
}

impl BitAnd for Signal6 {
    type Output = Signal6;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl PartialEq for Signal6 {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Debug for Signal6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}

impl Display for Signal6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.values, f)
    }
}
