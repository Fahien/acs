// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::Demux4Way;

use super::{And, Not, Signal, Signal3, Unit};

/// 8-way demultiplexor
#[derive(Default)]
pub struct Demux8Way {
    inp: Signal,
    sel: Signal3,
    a: Signal,
    b: Signal,
    c: Signal,
    d: Signal,
    e: Signal,
    f: Signal,
    g: Signal,
    h: Signal,

    demux_a_b_c_d: Demux4Way,
    demux_e_f_g_h: Demux4Way,

    not_sel2: Not,
    ands: [And; 8],
}

impl Demux8Way {
    /// Carries out the simulation and returns
    /// ```c
    /// if sel == 0 { [inp,  0,  0,  0,  0,  0,  0,  0] }
    /// if sel == 1 { [  0,inp,  0,  0,  0,  0,  0,  0] }
    /// if sel == 2 { [  0,  0,inp,  0,  0,  0,  0,  0] }
    /// if sel == 3 { [  0,  0,  0,inp,  0,  0,  0,  0] }
    /// if sel == 4 { [  0,  0,  0,  0,inp,  0,  0,  0] }
    /// if sel == 5 { [  0,  0,  0,  0,  0,inp,  0,  0] }
    /// if sel == 6 { [  0,  0,  0,  0,  0,  0,inp,  0] }
    /// if sel == 7 { [  0,  0,  0,  0,  0,  0,  0,inp] }
    /// ```
    pub fn sim(&mut self, inp: Signal, sel: Signal3) -> [Signal; 8] {
        self.inp = inp;
        self.sel = sel;
        self.eval();
        [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        ]
    }
}
impl Unit for Demux8Way {
    fn eval(&mut self) {
        let sel01 = self.sel.into();

        let abcd = self.demux_a_b_c_d.sim(self.inp, sel01);
        (self.a, self.b, self.c, self.d) = (abcd[0], abcd[1], abcd[2], abcd[3]);

        let efgh = self.demux_e_f_g_h.sim(self.inp, sel01);
        (self.e, self.f, self.g, self.h) = (efgh[0], efgh[1], efgh[2], efgh[3]);

        let sel2 = self.sel.get(2).into();
        let not_sel2_out = self.not_sel2.sim(sel2);

        self.a = self.ands[0].sim(self.a, not_sel2_out);
        self.b = self.ands[1].sim(self.b, not_sel2_out);
        self.c = self.ands[2].sim(self.c, not_sel2_out);
        self.d = self.ands[3].sim(self.d, not_sel2_out);
        self.e = self.ands[4].sim(self.e, sel2);
        self.f = self.ands[5].sim(self.f, sel2);
        self.g = self.ands[6].sim(self.g, sel2);
        self.h = self.ands[7].sim(self.h, sel2);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut demux = Demux8Way::default();

        demux.sim(Signal::LO, Signal3::new(0));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(1));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(2));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(3));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(4));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(5));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(6));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(7));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(0));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(1));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(2));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(3));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(4));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(5));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(6));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);

        demux.sim(Signal::LO, Signal3::new(7));
        assert_eq!(demux.a, Signal::LO);
        assert_eq!(demux.b, Signal::LO);
        assert_eq!(demux.c, Signal::LO);
        assert_eq!(demux.d, Signal::LO);
        assert_eq!(demux.e, Signal::LO);
        assert_eq!(demux.f, Signal::LO);
        assert_eq!(demux.g, Signal::LO);
        assert_eq!(demux.h, Signal::LO);
    }
}
