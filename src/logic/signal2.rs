// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::Signal3;

/// 2 signals into one
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Signal2 {
    LOLO,
    LOHI,
    HILO,
    HIHI,
}

impl Default for Signal2 {
    fn default() -> Self {
        Self::LOLO
    }
}

impl Signal2 {
    /// All signals `HI`
    pub const TRUE: Self = Self::HIHI;

    /// All signals `LO`
    pub const FALSE: Self = Self::LOLO;

    pub fn new(values: i8) -> Self {
        // Discard non-relevant bits
        let values = values & 0b11;
        match values {
            0 => Self::LOLO,
            1 => Self::LOHI,
            2 => Self::HILO,
            _ => Self::HIHI,
        }
    }

    pub fn get(&self, index: usize) -> i8 {
        assert!(index < 2);
        let mask = 1 << index;
        (*self as i8 & mask) >> index
    }
}

impl From<i8> for Signal2 {
    fn from(value: i8) -> Self {
        Signal2::new(value)
    }
}

impl From<u16> for Signal2 {
    fn from(value: u16) -> Self {
        Signal2::new(value as i8)
    }
}

impl From<i32> for Signal2 {
    fn from(value: i32) -> Self {
        Self::from(value as i8)
    }
}

impl From<&Signal2> for i8 {
    fn from(signal: &Signal2) -> Self {
        *signal as i8
    }
}

impl From<Signal3> for Signal2 {
    fn from(signal3: Signal3) -> Self {
        Signal2::new(i8::from(signal3))
    }
}

impl Not for Signal2 {
    type Output = Signal2;

    fn not(self) -> Self::Output {
        Signal2::new(!(self as i8))
    }
}

impl BitAnd for Signal2 {
    type Output = Signal2;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self as i8 & rhs as i8)
    }
}

impl Debug for Signal2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02b}", *self as i8))
    }
}

impl Display for Signal2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02b}", *self as i8))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let sig = Signal2::new(0b10);

        assert_eq!(sig.get(0), 0);
        assert_eq!(sig.get(1), 1);
    }
}
