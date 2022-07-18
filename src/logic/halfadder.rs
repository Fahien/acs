// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{And, Signal, Unit, Xor};

/// The Half-adder chip is able to add together 2 binary numbers.
/// The result is a `sum` and a `carry` bit
#[derive(Default)]
pub struct HalfAdder {
    a: Signal,
    b: Signal,
    sum: Signal,
    carry: Signal,

    xor: Xor,
    and: And,
}

impl HalfAdder {
    pub fn new(a: Signal, b: Signal) -> Self {
        Self {
            a,
            b,
            ..Default::default()
        }
    }

    /// Carries out the simulation but computing `a + b` and returns
    /// their `sum` and `carry` bit in this exact order.
    pub fn sim(&mut self, a: Signal, b: Signal) -> (Signal, Signal) {
        self.a = a;
        self.b = b;
        self.eval();
        (self.sum, self.carry)
    }
}

impl Unit for HalfAdder {
    fn eval(&mut self) {
        self.sum = self.xor.sim(self.a, self.b);
        self.carry = self.and.sim(self.a, self.b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut add = HalfAdder::new(Signal::HI, Signal::LO);

        add.sim(Signal::HI, Signal::LO);
        assert_eq!(add.sum, Signal::HI);
        assert_eq!(add.carry, Signal::LO);

        add.sim(Signal::HI, Signal::HI);
        assert_eq!(add.sum, Signal::LO);
        assert_eq!(add.carry, Signal::HI);
    }
}
