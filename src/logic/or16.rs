// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand16, Not16, Signal16, Unit};

/// Similar to the Or gate, but it operates in bunches of 16 signals
#[derive(Default)]
pub struct Or16 {
    a: Signal16,
    b: Signal16,
    out: Signal16,

    not_a: Not16,
    not_b: Not16,
    nand: Nand16,
}

impl Or16 {
    /// Carries out the simulation and returns the logic operation `a OR b`
    /// over all the 16 signals
    pub fn sim(&mut self, a: Signal16, b: Signal16) -> Signal16 {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for Or16 {
    fn eval(&mut self) {
        self.out = self
            .nand
            .sim(self.not_a.sim(self.a), self.not_b.sim(self.b));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut or = Or16::default();

        or.sim(Signal16::FALSE, Signal16::FALSE);
        assert_eq!(or.out, Signal16::FALSE);

        or.sim(Signal16::TRUE, Signal16::FALSE);
        assert_eq!(or.out, Signal16::TRUE);

        or.sim(Signal16::FALSE, Signal16::TRUE);
        assert_eq!(or.out, Signal16::TRUE);

        or.sim(Signal16::TRUE, Signal16::TRUE);
        assert_eq!(or.out, Signal16::TRUE);
    }
}
