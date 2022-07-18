// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{FullAdder, HalfAdder, Signal16, Unit};

/// The Add16 chip is able to add two 16-bit numbers
#[derive(Default)]
pub struct Add16 {
    a: Signal16,
    b: Signal16,
    out: Signal16,

    halfadder: HalfAdder,
    fulladders: [FullAdder; 15],
}

impl Add16 {
    /// Carries out the simulation and returns `a + b` (overflowing add)
    pub fn sim(&mut self, a: Signal16, b: Signal16) -> Signal16 {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for Add16 {
    fn eval(&mut self) {
        let (mut curr_sum, mut curr_carry) = self
            .halfadder
            .sim(self.a.get(0..1).into(), self.b.get(0..1).into());
        self.out.set(0, curr_sum);

        for i in 1..16 {
            let fulladder = &mut self.fulladders[i - 1];
            (curr_sum, curr_carry) = fulladder.sim(
                self.a.get(i..i + 1).into(),
                self.b.get(i..i + 1).into(),
                curr_carry,
            );
            self.out.set(i, curr_sum);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut add = Add16::default();

        add.eval();
        assert_eq!(add.out, Signal16::FALSE);

        add.a = Signal16::FALSE;
        add.b = Signal16::TRUE;
        add.eval();
        assert_eq!(add.out, add.b);

        add.a = Signal16::TRUE;
        add.b = Signal16::TRUE;
        add.eval();
        assert_eq!(add.out, Signal16::from(0b1111111111111110u16));

        add.a = Signal16::from(0b1010101010101010u16);
        add.b = Signal16::from(0b0101010101010101i16);
        add.eval();
        assert_eq!(add.out, Signal16::TRUE);

        add.a = Signal16::from(0b0011110011000011i16);
        add.b = Signal16::from(0b0000111111110000i16);
        add.eval();
        assert_eq!(add.out, Signal16::from(0b0100110010110011i16));

        add.a = Signal16::from(0b0001001000110100i16);
        add.b = Signal16::from(0b1001100001110110u16);
        add.eval();
        assert_eq!(add.out, Signal16::from(0b1010101010101010u16));
    }
}
