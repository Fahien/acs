// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{mem::fast::Ram8, Demux8Way, Mux8Way16, Signal, Signal16, Signal6, Time, Unit};

/// A Random Access Memory consisting of 64 16-bit register (8 `Ram8`)
#[derive(Default)]
pub struct Ram64 {
    time: Time,
    inp: Signal16,
    load: Signal,
    address: Signal6,
    out: Signal16,

    demux: Demux8Way,
    mux: Mux8Way16,
    rams: [Ram8; 8],
}

impl Ram64 {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data input
    /// - `load`: whether to write `inp` into the RAM
    /// - `address`: which register to select
    ///
    /// It returns the value stored at `address. Furthermore, when `load` is
    /// `HI`, `inp` is written at location `address`, and the loaded value will
    /// be emitted from the next time step onward.
    pub fn sim(&mut self, inp: Signal16, load: Signal, address: Signal6) -> Signal16 {
        self.inp = inp;
        self.load = load;
        self.address = address;
        self.tick();
        self.out
    }
}

impl Unit for Ram64 {
    fn tick(&mut self) {
        self.time += 1;
        let mux_signal = self.address.get(3..6).into();
        let ram_loads = self.demux.sim(self.load, mux_signal);
        let mut ram_outs = [Signal16::FALSE; 8];
        for i in 0..8 {
            ram_outs[i] = self.rams[i].sim(self.inp, ram_loads[i], self.address);
        }
        self.out = self.mux.sim(ram_outs, mux_signal);
    }

    fn tock(&mut self) {
        self.time += 1;
        self.eval();
    }

    fn eval(&mut self) {
        let mux_signal = self.address.get(3..6).into();
        let mut ram_outs = [Signal16::FALSE; 8];
        (0..8).for_each(|i| {
            self.rams[i].set_address(self.address);
            self.rams[i].eval();
            ram_outs[i] = self.rams[i].out()
        });
        self.out = self.mux.sim(ram_outs, mux_signal);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut ram = Ram64::default();
        ram.tick();
        assert_eq!(ram.out, Signal16::FALSE);
        ram.tock();
        assert_eq!(ram.out, Signal16::FALSE);

        ram.load = Signal::HI;
        ram.tick();
        assert_eq!(ram.out, Signal16::FALSE);
        ram.tock();
        assert_eq!(ram.out, Signal16::FALSE);

        ram.load = Signal::LO;
        ram.inp = Signal16::new(11111);
        ram.tick();
        assert_eq!(ram.out, Signal16::FALSE);
        ram.tock();
        assert_eq!(ram.out, Signal16::FALSE);

        ram.load = Signal::HI;
        ram.tick();
        assert_eq!(ram.out, Signal16::FALSE);
        ram.tock();
        assert_eq!(ram.out, ram.inp);
    }
}
