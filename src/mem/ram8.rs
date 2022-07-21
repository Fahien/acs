// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{mem::fast::Register16, Demux8Way, Mux8Way16, Signal, Signal16, Signal3, Time, Unit};

/// A Random Access Memory consisting of 8 16-bit registers
#[derive(Default)]
pub struct Ram8 {
    time: Time,
    inp: Signal16,
    load: Signal,
    address: Signal3,
    out: Signal16,

    demux: Demux8Way,
    mux: Mux8Way16,
    registers: [Register16; 8],
}

impl Ram8 {
    /// Carries out the simulation, taking as input:
    ///
    /// - `inp`: data input
    /// - `load`: whether to write `inp` into the RAM
    /// - `address`: which register to select
    ///
    /// It returns the value stored at `address. Furthermore, when `load` is
    /// `HI`, `inp` is written at location `address`, and the loaded value will
    /// be emitted from the next time step onward.
    pub fn sim(&mut self, inp: Signal16, load: Signal, address: Signal3) -> Signal16 {
        self.inp = inp;
        self.load = load;
        self.address = address;
        self.tick();
        self.out
    }
}

impl Unit for Ram8 {
    fn tick(&mut self) {
        self.time += 1;

        let register_loads = self.demux.sim(self.load, self.address);
        let mut register_outs = [Signal16::FALSE; 8];

        for i in 0..8 {
            register_outs[i] = self.registers[i].sim(self.inp, register_loads[i]);
        }

        self.out = self.mux.sim(register_outs, self.address);
    }

    fn tock(&mut self) {
        self.time += 1;
        self.eval();
    }

    fn eval(&mut self) {
        let mut register_outs = [Signal16::FALSE; 8];
        (0..8).for_each(|i| {
            self.registers[i].eval();
            register_outs[i] = self.registers[i].out();
        });
        self.out = self.mux.sim(register_outs, self.address);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut ram = Ram8::default();
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
