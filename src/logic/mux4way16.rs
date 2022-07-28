// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Mux16, Signal16, Signal2, Unit};

/// 4-way 16-bit multiplexor
#[derive(Default)]
pub struct Mux4Way16 {
    a: Signal16,
    b: Signal16,
    c: Signal16,
    d: Signal16,
    sel: Signal2,
    out: Signal16,

    mux_a_b: Mux16,
    mux_c_d: Mux16,
    mux_out: Mux16,
}

impl Mux4Way16 {
    /// Carries out the simulation and returns
    /// ```c
    /// if sel == 0b00 { a }
    /// if sel == 0b01 { b }
    /// if sel == 0b10 { c }
    /// if sel == 0b11 { d }
    /// ```
    pub fn sim(&mut self, signals: [Signal16; 4], sel: Signal2) -> Signal16 {
        self.a = signals[0];
        self.b = signals[1];
        self.c = signals[2];
        self.d = signals[3];
        self.sel = sel;
        self.eval();
        self.out
    }

    pub fn sel(&self) -> Signal2 {
        self.sel
    }
}

impl Unit for Mux4Way16 {
    fn eval(&mut self) {
        let sel0 = self.sel.get(0).into();
        let sel1 = self.sel.get(1).into();

        self.out = self.mux_out.sim(
            self.mux_a_b.sim(self.a, self.b, sel0),
            self.mux_c_d.sim(self.c, self.d, sel0),
            sel1,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut mux = Mux4Way16::default();

        let signals = [Signal16::FALSE; 4];

        mux.sim(signals, Signal2::new(0));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal2::new(1));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal2::new(2));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal2::new(3));
        assert_eq!(mux.out, Signal16::FALSE);

        let signals = [
            Signal16::from(0b0001001000110100u16),
            Signal16::from(0b1001100001110110u16),
            Signal16::from(0b1010101010101010u16),
            Signal16::from(0b0101010101010101u16),
        ];

        mux.sim(signals, Signal2::new(0));
        assert_eq!(mux.out, mux.a);

        mux.sim(signals, Signal2::new(1));
        assert_eq!(mux.out, mux.b);

        mux.sim(signals, Signal2::new(2));
        assert_eq!(mux.out, mux.c);

        mux.sim(signals, Signal2::new(3));
        assert_eq!(mux.out, mux.d);
    }
}
