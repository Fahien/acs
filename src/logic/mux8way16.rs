// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Mux16, Mux4Way16, Signal16, Signal3, Unit};

/// 8-way 16-bit multiplexor
#[derive(Default)]
pub struct Mux8Way16 {
    a: Signal16,
    b: Signal16,
    c: Signal16,
    d: Signal16,
    e: Signal16,
    f: Signal16,
    g: Signal16,
    h: Signal16,
    sel: Signal3,
    out: Signal16,

    mux_a_b_c_d: Mux4Way16,
    mux_e_f_g_h: Mux4Way16,
    mux_out: Mux16,
}

impl Mux8Way16 {
    /// Carries out the simulation and returns
    /// ```c
    /// if sel == 0 { a }
    /// if sel == 1 { b }
    /// if sel == 2 { c }
    /// if sel == 3 { d }
    /// if sel == 4 { e }
    /// if sel == 5 { f }
    /// if sel == 6 { g }
    /// if sel == 7 { h }
    /// ```
    pub fn sim(&mut self, signals: [Signal16; 8], sel: Signal3) -> Signal16 {
        self.a = signals[0];
        self.b = signals[1];
        self.c = signals[2];
        self.d = signals[3];
        self.e = signals[4];
        self.f = signals[5];
        self.g = signals[6];
        self.h = signals[7];
        self.sel = sel;
        self.eval();
        self.out
    }
}

impl Unit for Mux8Way16 {
    fn eval(&mut self) {
        self.out = self.mux_out.sim(
            self.mux_a_b_c_d
                .sim([self.a, self.b, self.c, self.d], self.sel.into()),
            self.mux_e_f_g_h
                .sim([self.e, self.f, self.g, self.h], self.sel.into()),
            self.sel.get(2).into(),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut mux = Mux8Way16::default();

        let signals = [Signal16::FALSE; 8];

        mux.sim(signals, Signal3::new(0));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(1));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(2));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(3));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(4));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(5));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(6));
        assert_eq!(mux.out, Signal16::FALSE);

        mux.sim(signals, Signal3::new(7));
        assert_eq!(mux.out, Signal16::FALSE);

        let signals = [
            Signal16::from(0b0001001000110100u16),
            Signal16::from(0b0010001101000101u16),
            Signal16::from(0b0011010001010110u16),
            Signal16::from(0b0100010101100111u16),
            Signal16::from(0b0101011001111000u16),
            Signal16::from(0b0110011110001001u16),
            Signal16::from(0b0111100010011010u16),
            Signal16::from(0b1000100110101011u16),
        ];

        mux.sim(signals, Signal3::new(0));
        assert_eq!(mux.out, mux.a);

        mux.sim(signals, Signal3::new(1));
        assert_eq!(mux.out, mux.b);

        mux.sim(signals, Signal3::new(2));
        assert_eq!(mux.out, mux.c);

        mux.sim(signals, Signal3::new(3));
        assert_eq!(mux.out, mux.d);

        mux.sim(signals, Signal3::new(4));
        assert_eq!(mux.out, mux.e);

        mux.sim(signals, Signal3::new(5));
        assert_eq!(mux.out, mux.f);

        mux.sim(signals, Signal3::new(6));
        assert_eq!(mux.out, mux.g);

        mux.sim(signals, Signal3::new(7));
        assert_eq!(mux.out, mux.h);
    }
}
