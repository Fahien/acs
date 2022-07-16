// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{And, Not, Signal, Unit};

/// The Demultiplexer gate
///
/// |inp|sel| a | b |
/// |---|---|---|---|
/// | * | 0 |inp| 0 |
/// | * | 1 | 0 |inp|
#[derive(Default)]
pub struct Demux {
    inp: Signal,
    sel: Signal,
    a: Signal,
    b: Signal,

    not_sel: Not,
    not_sel_and_inp: And,
    sel_and_inp: And,
}

impl Demux {
    /// Carries out the simulation and returns `sel ? (inp, 0) : (0, inp)`
    pub fn sim(&mut self, inp: Signal, sel: Signal) -> (Signal, Signal) {
        self.inp = inp;
        self.sel = sel;
        self.eval();
        (self.a, self.b)
    }
}

impl Unit for Demux {
    fn eval(&mut self) {
        self.a = self
            .not_sel_and_inp
            .sim(self.not_sel.sim(self.sel), self.inp);
        self.b = self.sel_and_inp.sim(self.sel, self.inp);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut demux = Demux::default();

        demux.sim(Signal::LO, Signal::LO);
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);

        demux.sim(Signal::HI, Signal::LO);
        assert_eq!(demux.a, Signal::HI);
        assert_eq!(demux.b, Signal::LO);

        demux.sim(Signal::LO, Signal::HI);
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);

        demux.sim(Signal::HI, Signal::HI);
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::HI);
    }
}
