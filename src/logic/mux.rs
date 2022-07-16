// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{And, Not, Or, Signal, Unit};

/// The Multiplexer gate
///
/// | a | b |sel|out|
/// |---|---|---|---|
/// | * | * | 0 | a |
/// | * | * | 1 | b |
#[derive(Default)]
pub struct Mux {
    a: Signal,
    b: Signal,
    sel: Signal,
    out: Signal,

    not_sel: Not,
    not_sel_and_a: And,
    sel_and_b: And,
    or: Or,
}

impl Mux {
    /// Carries out the simulation and returns `sel ? b : a`
    pub fn sim(&mut self, a: Signal, b: Signal, sel: Signal) -> Signal {
        self.a = a;
        self.b = b;
        self.sel = sel;
        self.eval();
        self.out
    }
}
impl Unit for Mux {
    fn eval(&mut self) {
        self.out = self.or.sim(
            self.not_sel_and_a.sim(self.a, self.not_sel.sim(self.sel)),
            self.sel_and_b.sim(self.sel, self.b),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut mux = Mux::default();

        mux.sim(Signal::LO, Signal::LO, Signal::LO);
        assert_eq!(mux.out, Signal::LO);

        mux.sim(Signal::HI, Signal::LO, Signal::LO);
        assert_eq!(mux.out, Signal::HI);

        mux.sim(Signal::LO, Signal::HI, Signal::LO);
        assert_eq!(mux.out, Signal::LO);

        mux.sim(Signal::HI, Signal::HI, Signal::LO);
        assert_eq!(mux.out, Signal::HI);

        mux.sim(Signal::LO, Signal::LO, Signal::HI);
        assert_eq!(mux.out, Signal::LO);

        mux.sim(Signal::HI, Signal::LO, Signal::HI);
        assert_eq!(mux.out, Signal::LO);

        mux.sim(Signal::LO, Signal::HI, Signal::HI);
        assert_eq!(mux.out, Signal::HI);

        mux.sim(Signal::HI, Signal::HI, Signal::HI);
        assert_eq!(mux.out, Signal::HI)
    }
}
