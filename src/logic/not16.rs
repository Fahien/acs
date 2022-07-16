// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand16, Signal16, Unit};

/// Similar to the Not gate, but it operates in bunches of 16 signals
#[derive(Default)]
pub struct Not16 {
    inp: Signal16,
    out: Signal16,

    nand: Nand16,
}

impl Not16 {
    /// Carries out the simulation and returns the logic operation `a NOT b`
    /// over all the 16 signals
    pub fn sim(&mut self, inp: Signal16) -> Signal16 {
        self.inp = inp;
        self.eval();
        self.out
    }
}

impl Unit for Not16 {
    fn eval(&mut self) {
        self.out = self.nand.sim(self.inp, self.inp);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut not = Not16::default();

        not.sim(Signal16::FALSE);
        assert_eq!(not.out, Signal16::TRUE);

        not.sim(Signal16::TRUE);
        assert_eq!(not.out, Signal16::FALSE);
    }
}
