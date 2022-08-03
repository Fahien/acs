// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{mem::fast::Register16, Inc16, Mux16, Signal, Signal16, Time, Unit};

/// Memory device which can increment its value in every time unit.
#[derive(Default)]
pub struct Counter {
    time: Time,
    inp: Signal16,
    reset: Signal,
    load: Signal,
    inc: Signal,
    out: Signal16,

    load_mux: Mux16,
    incrementer: Inc16,
    inc_mux: Mux16,
    reset_mux: Mux16,
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
        self.time += 1;

        let inc_mux_out = self
            .inc_mux
            .sim(self.out, self.incrementer.sim(self.out), self.inc);

        let load_mux_out = self.load_mux.sim(inc_mux_out, self.inp, self.load);

        let reset_mux_out = self
            .reset_mux
            .sim(load_mux_out, Signal16::FALSE, self.reset);

        self.out = self.reg.sim(reset_mux_out, Signal::HI);
    }

    fn tock(&mut self) {
        self.time += 1;
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
        let mut counter = Counter::default();

        counter.tick();
        assert_eq!(counter.out, Signal16::FALSE);

        counter.inc = Signal::HI;
        counter.tick();
        assert_eq!(counter.out, Signal16::FALSE);
        counter.tock();
        assert_eq!(counter.out, Signal16::new(1));

        counter.load = Signal::HI;
        counter.inp = Signal16::from(42);
        counter.tick();
        assert_eq!(counter.out, Signal16::new(1));
        counter.tock();
        assert_eq!(counter.out, counter.inp);

        counter.reset = Signal::HI;
        counter.tick();
        assert_eq!(counter.out, counter.inp);
        counter.tock();
        assert_eq!(counter.out, Signal16::FALSE);
    }
}
