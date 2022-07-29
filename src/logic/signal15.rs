// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::Signal16;

/// 15 signals into one
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Signal15 {
    pub values: i16,
}

impl Signal15 {
    /// All signals `HI`
    pub const TRUE: Self = Self {
        values: 0b111_1111_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Self = Self { values: 0 };

    pub fn new(values: i16) -> Self {
        Self {
            values: values & 0b111_1111_1111_1111,
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

impl From<i16> for Signal15 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Signal15 {
    fn from(value: i32) -> Self {
        Self::new(value as i16)
    }
}

impl From<Signal16> for Signal15 {
    fn from(signal: Signal16) -> Self {
        Self::new(signal.values)
    }
}

impl From<Signal15> for u16 {
    fn from(signal: Signal15) -> u16 {
        unsafe { std::mem::transmute(signal.values) }
    }
}

impl From<Signal15> for i16 {
    fn from(signal: Signal15) -> i16 {
        signal.values as i16
    }
}

impl From<Signal15> for usize {
    fn from(signal: Signal15) -> Self {
        u16::from(signal) as usize
    }
}

impl Not for Signal15 {
    type Output = Signal15;

    fn not(self) -> Self::Output {
        Signal15::new(!self.values)
    }
}

impl BitAnd for Signal15 {
    type Output = Signal15;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl Debug for Signal15 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:015b}", self.values))
    }
}

impl Display for Signal15 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:015b}", self.values))
    }
}
