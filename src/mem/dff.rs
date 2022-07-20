// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Signal, Unit};

/// Data Flip-Flop
#[derive(Default)]
pub struct Dff {
    inp: Signal,
    out: Signal,

    state: Signal,
}

impl Dff {
    /// Carries out the simulation and returns `inp` at `t-1`
    pub fn sim(&mut self, inp: Signal) -> Signal {
        self.inp = inp;
        self.tick();
        self.out
    }

    pub fn get_out(&self) -> Signal {
        self.out
    }
}

impl Unit for Dff {
    fn tick(&mut self) {
        self.out = self.state;
        self.state = self.inp;
    }

    fn eval(&mut self) {
        self.out = self.state;
    }
}
