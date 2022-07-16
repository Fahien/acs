// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Signal {
    LO,
    HI,
}

impl Default for Signal {
    fn default() -> Self {
        Self::LO
    }
}

impl Signal {
    pub fn new(int: i8) -> Self {
        if int == 0 {
            Self::LO
        } else {
            Self::HI
        }
    }

    pub fn as_bool(&self) -> bool {
        *self == Self::HI
    }
}

impl From<bool> for Signal {
    fn from(value: bool) -> Self {
        if value {
            Self::HI
        } else {
            Self::LO
        }
    }
}

impl From<u8> for Signal {
    fn from(value: u8) -> Self {
        if value == 0 {
            Self::LO
        } else {
            Self::HI
        }
    }
}

impl From<u16> for Signal {
    fn from(value: u16) -> Self {
        Self::new(value as i8)
    }
}

impl From<i16> for Signal {
    fn from(value: i16) -> Self {
        Self::new(value as i8)
    }
}

impl From<i32> for Signal {
    fn from(value: i32) -> Self {
        Self::new(value as i8)
    }
}

impl From<&Signal> for bool {
    fn from(signal: &Signal) -> Self {
        *signal == Signal::HI
    }
}

impl From<&Signal> for i8 {
    fn from(signal: &Signal) -> Self {
        *signal as i8
    }
}

impl From<Signal> for u16 {
    fn from(signal: Signal) -> Self {
        signal as u16
    }
}

impl Not for Signal {
    type Output = Signal;

    fn not(self) -> Self::Output {
        match self {
            Self::HI => Self::LO,
            Self::LO => Self::HI,
        }
    }
}

impl BitAnd for Signal {
    type Output = Signal;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self as i8 & rhs as i8)
    }
}

impl BitOr for Signal {
    type Output = Signal;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self as i8 | rhs as i8)
    }
}

impl BitAndAssign for Signal {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self::new(*self as i8 & rhs as i8)
    }
}

impl BitOrAssign for Signal {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self::new(*self as i8 | rhs as i8);
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", *self as i8))
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", *self as i8))
    }
}

impl PartialEq<i8> for Signal {
    fn eq(&self, other: &i8) -> bool {
        *self as i8 == *other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signal() {
        let sig = Signal::HI;
        assert_eq!(sig, 1);

        let zero = Signal::LO;
        assert_eq!(!zero, 1);
        assert_eq!(zero, !sig);
    }

    #[test]
    fn fmt() {
        let one = Signal::HI;
        assert_eq!(one.to_string(), "1");

        let zero = Signal::LO;
        assert_eq!(zero.to_string(), "0");
    }
}
