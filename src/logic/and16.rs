// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand16, Not16, Signal16, Unit};

/// Similar to the AND gate, but it operates in bunches of 16 signals
#[derive(Default)]
pub struct And16 {
    a: Signal16,
    b: Signal16,
    out: Signal16,

    not: Not16,
    nand: Nand16,
}

impl And16 {
    /// Carries out the simulation and returns the logic operation `a AND b`
    /// over all the 16 signals
    pub fn sim(&mut self, a: Signal16, b: Signal16) -> Signal16 {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for And16 {
    fn eval(&mut self) {
        self.out = self.not.sim(self.nand.sim(self.a, self.b));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut and = And16::default();

        and.sim(Signal16::FALSE, Signal16::FALSE);
        assert_eq!(and.out, Signal16::FALSE);

        and.sim(Signal16::TRUE, Signal16::FALSE);
        assert_eq!(and.out, Signal16::FALSE);

        and.sim(Signal16::FALSE, Signal16::TRUE);
        assert_eq!(and.out, Signal16::FALSE);

        and.sim(Signal16::TRUE, Signal16::TRUE);
        assert_eq!(and.out, Signal16::TRUE);
    }
}
