// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::Or8Way;

use super::{Or, Signal, Signal16, Unit};

/// 16-way Or gate
#[derive(Default)]
pub struct Or16Way {
    inp: Signal16,
    out: Signal,

    ors: [Or8Way; 2],
    or: Or,
}

impl Or16Way {
    /// Carries out the simulation and returns `inp[0] OR inp[1] OR .. OR inp[15]`
    pub fn sim(&mut self, inp: Signal16) -> Signal {
        self.inp = inp;
        self.eval();
        self.out
    }
}

impl Unit for Or16Way {
    fn eval(&mut self) {
        let or0_out = self.ors[0].sim(self.inp.get(0..8).into());
        let or1_out = self.ors[0].sim(self.inp.get(8..16).into());
        self.out = self.or.sim(or0_out, or1_out);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut or = Or16Way::default();

        or.eval();
        assert_eq!(or.out, Signal::LO);

        or.inp = Signal16::TRUE;
        or.eval();
        assert_eq!(or.out, Signal::HI);

        or.inp = Signal16::from(0b00010000);
        or.eval();
        assert_eq!(or.out, Signal::HI);

        or.inp = Signal16::from(1);
        or.eval();
        assert_eq!(or.out, Signal::HI);

        or.inp = Signal16::from(0b100110);
        or.eval();
        assert_eq!(or.out, Signal::HI);
    }
}
