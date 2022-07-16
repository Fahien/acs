// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Signal, Unit};

/// The NAND gate is the only built-in logic component
///
/// | a | b |out|
/// |---|---|---|
/// | 0 | 0 | 1 |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | 0 |
#[derive(Default)]
pub struct Nand {
    a: Signal,
    b: Signal,
    out: Signal,
}

impl Nand {
    /// Carries out the simulation and returns `a NAND b`
    pub fn sim(&mut self, a: Signal, b: Signal) -> Signal {
        self.a = a;
        self.b = b;
        self.out = !(self.a & self.b);
        self.out
    }
}

impl Unit for Nand {
    fn eval(&mut self) {
        self.out = !(self.a & self.b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn run() {
        let mut nand = Nand::default();
        assert_eq!(nand.sim(Signal::LO, Signal::LO), Signal::HI);
        assert_eq!(nand.sim(Signal::HI, Signal::LO), Signal::HI);
        assert_eq!(nand.sim(Signal::LO, Signal::HI), Signal::HI);
        assert_eq!(nand.sim(Signal::HI, Signal::HI), Signal::LO);
    }
}
