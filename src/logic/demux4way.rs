// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{And, Demux, Not, Signal, Signal2, Unit};

/// 4-way demultiplexor
#[derive(Default)]
pub struct Demux4Way {
    inp: Signal,
    sel: Signal2,
    a: Signal,
    b: Signal,
    c: Signal,
    d: Signal,

    demux_a_b: Demux,
    demux_c_d: Demux,

    not_sel1: Not,
    ands: [And; 4],
}

impl Demux4Way {
    /// Carries out the simulation and returns
    /// ```c
    /// if sel == 0 { [inp,  0,  0,  0] }
    /// if sel == 1 { [  0,inp,  0,  0] }
    /// if sel == 2 { [  0,  0,inp,  0] }
    /// if sel == 3 { [  0,  0,  0,inp] }
    /// ```
    pub fn sim(&mut self, inp: Signal, sel: Signal2) -> [Signal; 4] {
        self.inp = inp;
        self.sel = sel;
        self.eval();
        [self.a, self.b, self.c, self.d]
    }
}

impl Unit for Demux4Way {
    fn eval(&mut self) {
        let sel0 = self.sel.get(0).into();
        (self.a, self.b) = self.demux_a_b.sim(self.inp, sel0);
        (self.c, self.d) = self.demux_c_d.sim(self.inp, sel0);

        let sel1 = self.sel.get(1).into();
        let not_sel1_out = self.not_sel1.sim(sel1);

        self.a = self.ands[0].sim(self.a, not_sel1_out);
        self.b = self.ands[1].sim(self.b, not_sel1_out);
        self.c = self.ands[2].sim(self.c, sel1);
        self.d = self.ands[3].sim(self.d, sel1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut demux = Demux4Way::default();

        demux.sim(Signal::LO, Signal2::new(0));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::LO, Signal2::new(1));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::LO, Signal2::new(2));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::LO, Signal2::new(3));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::HI, Signal2::new(0));
        assert_eq!(demux.a, Signal::HI);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::HI, Signal2::new(1));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::HI);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::HI, Signal2::new(2));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::HI);
        assert_eq!(demux.d, Signal::LO);

        demux.sim(Signal::HI, Signal2::new(3));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::HI);
    }
}
