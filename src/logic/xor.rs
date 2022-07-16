// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{And, Nand, Or, Signal, Unit};

/// The XOR gate is built using a NAND, an OR, and an AND gate
///
/// | a | b |out|
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | 0 |
#[derive(Default)]
pub struct Xor {
    a: Signal,
    b: Signal,
    out: Signal,

    nand: Nand,
    or: Or,
    and: And,
}

impl Xor {
    /// Carries out the simulation and returns `a XOR b`
    pub fn sim(&mut self, a: Signal, b: Signal) -> Signal {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for Xor {
    fn eval(&mut self) {
        self.out = self
            .and
            .sim(self.nand.sim(self.a, self.b), self.or.sim(self.a, self.b));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut xor = Xor::default();

        xor.sim(Signal::LO, Signal::LO);
        assert_eq!(xor.out, Signal::LO);

        xor.sim(Signal::HI, Signal::LO);
        assert_eq!(xor.out, Signal::HI);

        xor.sim(Signal::LO, Signal::HI);
        assert_eq!(xor.out, Signal::HI);

        xor.sim(Signal::HI, Signal::HI);
        assert_eq!(xor.out, Signal::LO);
    }
}
