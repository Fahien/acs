// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{Debug, Display},
    ops::*,
};

use crate::{asm::instruction::AsmInstruction, Signal15};

use super::Signal;

/// 16 signals into one
#[derive(Copy, Clone, Default)]
pub struct Signal16 {
    pub values: i16,
}

impl Signal16 {
    #[allow(overflowing_literals)]
    /// All signals `HI`
    pub const TRUE: Self = Self {
        values: 0b1111_1111_1111_1111,
    };

    /// All signals `LO`
    pub const FALSE: Signal16 = Signal16 { values: 0 };

    pub fn new(values: i16) -> Self {
        Self { values }
    }

    pub fn get(&self, index: Range<usize>) -> i16 {
        let shift = index.start;
        let width = index.len() as u32;
        let mask_unshifted = 2_i16.pow(width) - 1;
        let mask = mask_unshifted << shift;
        let ret_unshifted = self.values & mask;
        ret_unshifted >> shift
    }

    pub fn set(&mut self, index: usize, signal: Signal) {
        // clear nth bit
        self.values &= !(1 << index);
        // set nth bit to signal
        let value = (signal as i16) << index;
        self.values |= value;
    }

    pub fn get_values(&self) -> &i16 {
        &self.values
    }

    pub fn get_values_mut(&mut self) -> &mut i16 {
        &mut self.values
    }
}

impl From<i32> for Signal16 {
    fn from(value: i32) -> Self {
        Self::new(value as i16)
    }
}

impl From<u16> for Signal16 {
    fn from(value: u16) -> Self {
        let value = unsafe { std::mem::transmute(value) };
        Self::new(value)
    }
}

impl From<i16> for Signal16 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<bool> for Signal16 {
    fn from(value: bool) -> Self {
        if value {
            Self::TRUE
        } else {
            Self::FALSE
        }
    }
}

impl From<Signal16> for u16 {
    fn from(signal: Signal16) -> Self {
        unsafe { std::mem::transmute(signal.values) }
    }
}

impl From<&Signal16> for i16 {
    fn from(signal: &Signal16) -> i16 {
        signal.values
    }
}

impl From<Signal16> for i16 {
    fn from(signal: Signal16) -> i16 {
        i16::from(&signal)
    }
}
impl From<&mut Signal16> for usize {
    fn from(signal: &mut Signal16) -> Self {
        signal.values as usize
    }
}

impl From<&Signal16> for usize {
    fn from(signal: &Signal16) -> Self {
        signal.values as usize
    }
}

impl From<Signal16> for usize {
    fn from(signal: Signal16) -> Self {
        usize::from(&signal)
    }
}

impl From<Signal> for Signal16 {
    fn from(signal: Signal) -> Self {
        Self::new((signal as i16) * !0i16) // splat
    }
}

impl From<Signal15> for Signal16 {
    fn from(signal: Signal15) -> Self {
        Self::new(signal.into())
    }
}

impl From<&AsmInstruction> for Signal16 {
    fn from(instr: &AsmInstruction) -> Self {
        Self::from(u16::from(instr))
    }
}

impl Not for Signal16 {
    type Output = Signal16;

    fn not(self) -> Self::Output {
        let values = !self.values;
        Signal16::new(values)
    }
}

impl Neg for Signal16 {
    type Output = Signal16;

    fn neg(self) -> Self::Output {
        let neg_value = i16::from(self);
        Self::from(-neg_value)
    }
}

impl Add<i16> for Signal16 {
    type Output = Signal16;

    fn add(self, rhs: i16) -> Self::Output {
        let add_value = i16::from(self);
        // Ignore overflow
        let (sum, _) = add_value.overflowing_add(rhs);
        Self::from(sum)
    }
}

impl Add<Signal16> for Signal16 {
    type Output = Signal16;

    fn add(self, rhs: Signal16) -> Self::Output {
        self.add(i16::from(rhs))
    }
}

impl AddAssign<i16> for Signal16 {
    fn add_assign(&mut self, rhs: i16) {
        (self.values, _) = self.values.overflowing_add(rhs);
    }
}

impl Sub<i16> for Signal16 {
    type Output = Signal16;

    fn sub(self, rhs: i16) -> Self::Output {
        let sub_value = i16::from(self);
        Self::from(sub_value - rhs)
    }
}

impl SubAssign<i16> for Signal16 {
    fn sub_assign(&mut self, rhs: i16) {
        (self.values, _) = self.values.overflowing_sub(rhs);
    }
}

impl Sub<Signal16> for Signal16 {
    type Output = Signal16;

    fn sub(self, rhs: Signal16) -> Self::Output {
        self.sub(i16::from(rhs))
    }
}

impl PartialOrd for Signal16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.values.partial_cmp(&other.values)
    }
}

impl BitAnd for Signal16 {
    type Output = Signal16;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.values & rhs.values)
    }
}

impl BitOr for Signal16 {
    type Output = Signal16;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.values | rhs.values)
    }
}

impl PartialEq for Signal16 {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl PartialEq<i16> for Signal16 {
    fn eq(&self, other: &i16) -> bool {
        self.values == *other
    }
}

impl Debug for Signal16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:016b}", self.values))
    }
}

impl Display for Signal16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:016b}", self.values))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signal() {
        let sig = Signal16::TRUE;
        assert_eq!(sig.get(0..1), 1);

        let zero = Signal16::FALSE;
        assert_eq!((!zero).get(0..1), 1);
        assert_eq!(zero, !sig);
    }

    #[test]
    fn fmt() {
        let one = Signal16::TRUE;
        assert_eq!(one.to_string(), "1111111111111111");

        let zero = Signal16::FALSE;
        assert_eq!(zero.to_string(), "0000000000000000");
    }
}
