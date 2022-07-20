// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Add16, Signal16, Unit};

/// The Inc16 chip is able to add 1 to a given number.
#[derive(Default)]
pub struct Inc16 {
    inp: Signal16,
    out: Signal16,

    add: Add16,
}

impl Inc16 {
    /// Carries out the simulation and returns `inp + 1` (overflowing add)
    pub fn sim(&mut self, inp: Signal16) -> Signal16 {
        self.inp = inp;
        self.eval();
        self.out
    }
}

impl Unit for Inc16 {
    fn eval(&mut self) {
        self.out = self.add.sim(self.inp, Signal16::new(1));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut add = Inc16::default();

        add.eval();
        assert_eq!(add.out, Signal16::new(1));

        add.inp = Signal16::TRUE;
        add.eval();
        assert_eq!(add.out, Signal16::new(0));

        add.inp = Signal16::new(5);
        add.eval();
        assert_eq!(add.out, Signal16::new(6));

        add.inp = Signal16::new(-5);
        add.eval();
        assert_eq!(add.out, Signal16::new(-4));
    }
}
