// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use crate::{Signal, Signal16};

/// Fast 16-bit register which, as opposed to the other `Register16`,
/// it does not carries out the simulation with other building blocks,
/// but it emulates it with native code.
#[derive(Default)]
pub struct Register16 {
    inp: Signal16,
    load: Signal,
    out: Signal16,

    bits: Signal16,
}

impl Register16 {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data bit
    /// - `load`: whether to write `inp` into the register
    ///
    /// And, at time `t`, returns (`out`) the following:
    /// ```c
    /// if load(t-1) { inp(t-1) }
    /// else { out(t-1) }
    /// ```
    pub fn sim(&mut self, inp: Signal16, load: Signal) -> Signal16 {
        self.inp = inp;
        self.load = load;

        self.out = self.bits;
        if self.load.as_bool() {
            self.bits = self.inp;
        }

        self.out
    }

    pub fn tock(&mut self) -> Signal16 {
        self.out = self.bits;
        self.out
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }
}

impl Register16 {
    pub fn run(&mut self) {
        self.out = self.bits;
        if self.load.as_bool() {
            self.bits = self.inp;
        }
    }

    pub fn eval(&mut self) {
        self.out = self.bits;
    }
}

impl From<&Register16> for i16 {
    fn from(register: &Register16) -> Self {
        register.bits.into()
    }
}

impl From<i32> for Register16 {
    fn from(_: i32) -> Self {
        Self::default()
    }
}

impl Display for Register16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.out, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut register = Register16::default();
        register.run();
        assert_eq!(register.out, Signal16::FALSE);

        register.inp = Signal16::TRUE;
        register.run();
        assert_eq!(register.out, Signal16::FALSE);
        register.run();
        assert_eq!(register.out, Signal16::FALSE);

        register.load = Signal::HI;
        register.run();
        assert_eq!(register.out, Signal16::FALSE);
        register.run();
        assert_eq!(register.out, Signal16::TRUE);
    }
}
