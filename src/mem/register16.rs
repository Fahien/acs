// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use crate::{Bit, Signal, Signal16, Time, Unit};

/// 16-bit register, behaving exactly like a 1-bit register, but
/// with 16-bit values
#[derive(Default)]
pub struct Register16 {
    time: Time,
    inp: Signal16,
    load: Signal,
    out: Signal16,

    bits: [Bit; 16],
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
        self.tick();
        self.out
    }
}

impl Unit for Register16 {
    fn tick(&mut self) {
        self.time += 1;

        for i in 0..16 {
            self.out.set(
                i,
                self.bits[i].sim(self.inp.get(i..i + 1).into(), self.load),
            );
        }
    }

    fn tock(&mut self) {
        self.time += 1;
        for i in 0..16 {
            self.bits[i].tock();
            self.out.set(i, self.bits[i].get_out());
        }
    }

    fn eval(&mut self) {
        for i in 0..16 {
            self.bits[i].eval();
            self.out.set(i, self.bits[i].get_out());
        }
    }
}

impl From<i32> for Register16 {
    fn from(_: i32) -> Self {
        Self::default()
    }
}

impl From<&Register16> for i16 {
    fn from(register: &Register16) -> Self {
        i16::from(&register.out)
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
        register.tick();
        assert_eq!(register.out, Signal16::FALSE);

        register.inp = Signal16::TRUE;
        register.tick();
        assert_eq!(register.out, Signal16::FALSE);
        register.tock();
        assert_eq!(register.out, Signal16::FALSE);

        register.load = Signal::HI;
        register.tick();
        assert_eq!(register.out, Signal16::FALSE);
        register.tock();
        assert_eq!(register.out, Signal16::TRUE);
    }
}
