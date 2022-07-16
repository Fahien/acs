// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Nand, Not, Signal, Unit};

/// The AND gate is built using a NOT and a NAND gate
///
/// | a | b |out|
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 0 |
/// | 1 | 0 | 0 |
/// | 1 | 1 | 1 |
#[derive(Default)]
pub struct And {
    a: Signal,
    b: Signal,
    out: Signal,

    not: Not,
    nand: Nand,
}

impl And {
    /// Carries out the simulation and returns `a AND b`
    pub fn sim(&mut self, a: Signal, b: Signal) -> Signal {
        self.a = a;
        self.b = b;
        self.eval();
        self.out
    }
}

impl Unit for And {
    fn eval(&mut self) {
        self.out = self.not.sim(self.nand.sim(self.a, self.b));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut and = And::default();

        and.sim(Signal::LO, Signal::LO);
        assert_eq!(and.out, Signal::LO);

        and.sim(Signal::HI, Signal::LO);
        assert_eq!(and.out, Signal::LO);

        and.sim(Signal::LO, Signal::HI);
        assert_eq!(and.out, Signal::LO);

        and.sim(Signal::HI, Signal::HI);
        assert_eq!(and.out, Signal::HI);
    }
}
