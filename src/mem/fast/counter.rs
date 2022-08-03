// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{mem::fast::Register16, Signal, Signal16, Unit};

/// Fast memory device which can increment its value in every time unit.
#[derive(Default)]
pub struct Counter {
    pub inp: Signal16,
    pub reset: Signal,
    pub load: Signal,
    pub inc: Signal,
    pub out: Signal16,

    reg: Register16,
}

impl Counter {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data input
    /// - `reset`: whether to reset to `zero` or not
    /// - `load`: whether to write `inp` into the counter register
    /// - `inc`: whether to increment the counter register
    ///
    /// It returns the following:
    /// ```c
    /// if reset(t-1) { out(t) = 0 }
    /// else if load(t-1) { out(t) = in(t-1) }
    /// else if inc(t-1) { out(t) = out(t-1)+1 }
    /// else { out(t) = out(t-1) }
    /// ```
    pub fn sim(&mut self, inp: Signal16, reset: Signal, load: Signal, inc: Signal) -> Signal16 {
        self.inp = inp;
        self.reset = reset;
        self.load = load;
        self.inc = inc;
        self.tick();
        self.out
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }
}

impl Unit for Counter {
    fn tick(&mut self) {
        let inc_mux_out = if self.inc.as_bool() {
            self.out + 1
        } else {
            self.out
        };

        let load_mux_out = if self.load.as_bool() {
            self.inp
        } else {
            inc_mux_out
        };

        let reset_mux_out = if self.reset.as_bool() {
            Signal16::FALSE
        } else {
            load_mux_out
        };

        self.out = self.reg.sim(reset_mux_out, Signal::HI);
    }

    fn tock(&mut self) {
        self.eval();
    }

    fn eval(&mut self) {
        self.reg.eval();
        self.out = self.reg.out();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut pc = Counter::default();

        pc.tick();
        assert_eq!(pc.out, Signal16::FALSE);

        pc.inc = Signal::HI;
        pc.tick();
        assert_eq!(pc.out, Signal16::FALSE);
        pc.tock();
        assert_eq!(pc.out, Signal16::new(1));

        pc.load = Signal::HI;
        pc.inp = Signal16::new(42);
        pc.tick();
        assert_eq!(pc.out, Signal16::new(1));
        pc.tock();
        assert_eq!(pc.out, pc.inp);

        pc.reset = Signal::HI;
        pc.tick();
        assert_eq!(pc.out, pc.inp);
        pc.tock();
        assert_eq!(pc.out, Signal16::FALSE);
    }
}
