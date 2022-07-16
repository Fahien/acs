// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand, Signal, Unit};

/// The NOT gate is built using a NAND gate
///
/// |inp|out|
/// |---|---|
/// | 0 | 1 |
/// | 1 | 0 |
#[derive(Default)]
pub struct Not {
    inp: Signal,
    out: Signal,

    nand: Nand,
}

impl Not {
    /// Carries out the simulation and returns `NOT inp`
    pub fn sim(&mut self, inp: Signal) -> Signal {
        self.inp = inp;
        self.eval();
        self.out
    }
}

impl Unit for Not {
    fn eval(&mut self) {
        self.out = self.nand.sim(self.inp, self.inp);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut not = Not::default();

        not.sim(Signal::LO);
        assert_eq!(not.out, Signal::HI);

        not.sim(Signal::HI);
        assert_eq!(not.out, Signal::LO);
    }
}
