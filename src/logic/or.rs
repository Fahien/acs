// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand, Not, Signal, Unit};

/// The OR gate is built using two NOTs and a NAND gate
///
/// | a | b |out|
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | 1 |
#[derive(Default)]
pub struct Or {
    a: Signal,
    b: Signal,
    out: Signal,

    not_a: Not,
    not_b: Not,
    nand: Nand,
}

impl Or {
    /// Carries out the simulation and returns `a OR b`
    pub fn sim(&mut self, a: Signal, b: Signal) -> Signal {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for Or {
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
        let mut or = Or::default();

        or.sim(Signal::LO, Signal::LO);
        assert_eq!(or.out, Signal::LO);

        or.sim(Signal::HI, Signal::LO);
        assert_eq!(or.out, Signal::HI);

        or.sim(Signal::LO, Signal::HI);
        assert_eq!(or.out, Signal::HI);

        or.sim(Signal::HI, Signal::HI);
        assert_eq!(or.out, Signal::HI);
    }
}
