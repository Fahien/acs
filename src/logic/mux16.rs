// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{And16, Not16, Or16, Signal, Signal16, Unit};

/// Similar to the Mux gate, but it operates in bunches of 16 signals
#[derive(Default)]
pub struct Mux16 {
    a: Signal16,
    b: Signal16,
    sel: Signal,
    out: Signal16,

    not_sel: Not16,
    not_sel_and_a: And16,
    sel_and_b: And16,
    or: Or16,
}

impl Mux16 {
    /// Carries out the simulation and returns `sel ? b : a` over all the 16 signals
    pub fn sim(&mut self, a: Signal16, b: Signal16, sel: Signal) -> Signal16 {
        self.a = a;
        self.b = b;
        self.sel = sel;
        self.eval();
        self.out
    }

    pub fn set_a(&mut self, a: Signal16) {
        self.a = a;
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }
}

impl Unit for Mux16 {
    fn eval(&mut self) {
        self.out = self.or.sim(
            self.not_sel_and_a
                .sim(self.a, self.not_sel.sim(self.sel.into())),
            self.sel_and_b.sim(self.sel.into(), self.b),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut mux = Mux16::default();

        mux.sim(Signal16::FALSE, Signal16::FALSE, Signal::LO);
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(Signal16::TRUE, Signal16::FALSE, Signal::LO);
        assert_eq!(mux.out, Signal16::TRUE);

        mux.sim(Signal16::FALSE, Signal16::TRUE, Signal::LO);
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(Signal16::TRUE, Signal16::TRUE, Signal::LO);
        assert_eq!(mux.out, Signal16::TRUE);

        mux.sim(Signal16::FALSE, Signal16::FALSE, Signal::HI);
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(Signal16::TRUE, Signal16::FALSE, Signal::HI);
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(Signal16::FALSE, Signal16::TRUE, Signal::HI);
        assert_eq!(mux.out, Signal16::TRUE);

        mux.sim(Signal16::TRUE, Signal16::TRUE, Signal::HI);
        assert_eq!(mux.out, Signal16::TRUE)
    }
}
