// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{HalfAdder, Or, Signal, Unit};

/// By means of two Half-adders, the Full-adder is able to add three bits.
/// The result is a `sum` and a `carry` bit.
#[derive(Default)]
pub struct FullAdder {
    a: Signal,
    b: Signal,
    c: Signal,
    sum: Signal,
    carry: Signal,

    halfadder_a_b: HalfAdder,
    halfadder_ab_c: HalfAdder,
    or: Or,
}

impl FullAdder {
    /// Carries out the simulation but computing `a + b + c` and returns
    /// their `sum` and `carry` bit in this exact order.
    pub fn sim(&mut self, a: Signal, b: Signal, c: Signal) -> (Signal, Signal) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.eval();
        (self.sum, self.carry)
    }
}
impl Unit for FullAdder {
    fn eval(&mut self) {
        let (a_b_sum, a_b_carry) = self.halfadder_a_b.sim(self.a, self.b);
        let (ab_c_sum, ab_c_carry) = self.halfadder_ab_c.sim(a_b_sum, self.c);
        self.sum = ab_c_sum;
        self.carry = self.or.sim(a_b_carry, ab_c_carry);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut add = FullAdder::default();

        add.eval();
        assert_eq!(add.sum, Signal::LO);
        assert_eq!(add.carry, Signal::LO);

        add.c = Signal::HI;
        add.eval();
        assert_eq!(add.sum, Signal::HI);
        assert_eq!(add.carry, Signal::LO);

        add.b = Signal::HI;
        add.c = Signal::LO;
        add.eval();
        assert_eq!(add.sum, Signal::HI);
        assert_eq!(add.carry, Signal::LO);

        add.c = Signal::HI;
        add.eval();
        assert_eq!(add.sum, Signal::LO);
        assert_eq!(add.carry, Signal::HI);

        add.a = Signal::HI;
        add.b = Signal::LO;
        add.c = Signal::LO;
        add.eval();
        assert_eq!(add.sum, Signal::HI);
        assert_eq!(add.carry, Signal::LO);

        add.c = Signal::HI;
        add.eval();
        assert_eq!(add.sum, Signal::LO);
        assert_eq!(add.carry, Signal::HI);

        add.b = Signal::HI;
        add.c = Signal::LO;
        add.eval();
        assert_eq!(add.sum, Signal::LO);
        assert_eq!(add.carry, Signal::HI);

        add.c = Signal::HI;
        add.eval();
        assert_eq!(add.sum, Signal::HI);
        assert_eq!(add.carry, Signal::HI);
    }
}
