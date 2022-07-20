// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Dff, Mux, Signal, Time, Unit};

/// Single-bit register, storing `0` or `1` over time.
#[derive(Default)]
pub struct Bit {
    time: Time,
    inp: Signal,
    load: Signal,
    out: Signal,

    mux: Mux,
    dff: Dff,
}

impl Bit {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data bit
    /// - `load`: whether to write `inp` into the register
    ///
    /// And, at time `t`, returns (`out`) the following:
    /// ```c
    /// if load(t-1) { inp(t-1) }
    /// else { out(t-1) }
    /// ```
    pub fn sim(&mut self, inp: Signal, load: Signal) -> Signal {
        self.inp = inp;
        self.load = load;
        self.tick();
        self.out
    }
}

impl Unit for Bit {
    fn tick(&mut self) {
        self.time += 1;
        self.out = self
            .dff
            .sim(self.mux.sim(self.dff.get_out(), self.inp, self.load));
    }

    fn tock(&mut self) {
        self.time += 1;
        self.dff.eval();
        self.out = self.dff.get_out();
    }

    fn eval(&mut self) {
        self.dff.eval();
        self.out = self.dff.get_out();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut bit = Bit::default();
        bit.tick();
        assert_eq!(bit.out, Signal::LO);
        bit.tock();
        assert_eq!(bit.out, Signal::LO);

        bit.inp = Signal::HI;
        bit.tick();
        assert_eq!(bit.out, Signal::LO);
        bit.tock();
        assert_eq!(bit.out, Signal::LO);

        bit.load = Signal::HI;
        bit.tick();
        assert_eq!(bit.out, Signal::LO);
        bit.tock();
        assert_eq!(bit.out, Signal::HI);

        bit.inp = Signal::LO;
        bit.tick();
        assert_eq!(bit.out, Signal::HI);
        bit.tock();
        assert_eq!(bit.out, Signal::LO);

        bit.load = Signal::LO;
        bit.tick();
        assert_eq!(bit.out, Signal::LO);
        bit.tock();
        assert_eq!(bit.out, Signal::LO);
    }
}
