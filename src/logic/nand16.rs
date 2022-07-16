// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand, Signal16, Unit};

/// Similar to the Nand gate, but it operates in bunches of 16 signals
#[derive(Default)]
pub struct Nand16 {
    a: Signal16,
    b: Signal16,
    out: Signal16,

    nands: [Nand; 16],
}

impl Nand16 {
    /// Carries out the simulation and returns the logic operation `a NAND b`
    /// over all the 16 signals
    pub fn sim(&mut self, a: Signal16, b: Signal16) -> Signal16 {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for Nand16 {
    fn eval(&mut self) {
        for i in 0..16 {
            let a = self.a.get(i..i + 1);
            let b = self.b.get(i..i + 1);

            let nand = &mut self.nands[i];
            let nand_out = nand.sim(a.into(), b.into());
            self.out.set(i, nand_out);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::logic::Signal;

    #[test]
    pub fn run() {
        let mut nand = Nand16::default();

        nand.sim(Signal16::FALSE, Signal16::FALSE);
        assert_eq!(nand.out, Signal16::TRUE);

        let mut one = Signal16::default();
        one.set(0, Signal::HI);
        nand.sim(one, Signal16::FALSE);
        assert_eq!(nand.out, Signal16::TRUE);

        nand.sim(Signal16::FALSE, Signal16::TRUE);
        assert_eq!(nand.out, Signal16::TRUE);

        nand.sim(one, Signal16::TRUE);
        assert_eq!(nand.out, !nand.a);
    }
}
