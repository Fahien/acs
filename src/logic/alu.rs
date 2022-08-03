// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Add16, And16, Mux16, Not, Not16, Or16Way, Signal, Signal16, Unit};

/// Arithmetic Logic Unit
#[derive(Default)]
pub struct Alu {
    x: Signal16,
    y: Signal16,
    /// Set x input to zero
    zx: Signal,
    /// Negates x input
    nx: Signal,
    /// Set y input to zero
    zy: Signal,
    // Negates y input
    ny: Signal,
    /// - False selects the & function
    /// - True selects the + function
    f: Signal,
    /// Negates the output
    no: Signal,

    out: Signal16,
    /// Whether the output is zero
    zr: Signal,
    /// Whether the output is negative
    ng: Signal,

    x_zero_mux: Mux16,
    not_x: Not16,
    x_negx_mux: Mux16,

    y_zero_mux: Mux16,
    not_y: Not16,
    y_negy_mux: Mux16,

    f_and: And16,
    f_add: Add16,
    f_mux: Mux16,

    not_o: Not16,
    o_nego_mux: Mux16,
    ors_o_zero: Or16Way,
    not_ors: Not,
}

impl Alu {
    /// Carries out the ALU simulation.
    ///
    /// It takes as input two main signals: `x` and `y`. Then the following:
    ///
    /// - `zx`: Set `x` to zero.
    /// - `nx`: Negate `x`.
    /// - `zy`: Set `y` to zero.
    /// - `ny`: Negate `y`.
    /// - `f`:
    ///   - `LO` selects the `&` function
    ///   - `HI` selects the `+` function
    /// - `no`: Negate `out` signal.
    ///
    /// It can return one of the following:
    /// `0`, `1`, `-1`, `x`, `y`, `!x`, `!y`, `-x`, `-y`,
    /// `x+1`, `y+1`, `x-1`, `y-1`, `x+y`, `x-y`, `y-x`, `x&y`, `x|y`,
    ///
    /// And two more signals which tell whether the result is _zero_ or _negative_
    /// in this exact order.
    #[allow(clippy::too_many_arguments)]
    pub fn sim(
        &mut self,
        x: Signal16,
        y: Signal16,
        zx: Signal,
        nx: Signal,
        zy: Signal,
        ny: Signal,
        f: Signal,
        no: Signal,
    ) -> (Signal16, Signal, Signal) {
        self.x = x;
        self.y = y;
        self.zx = zx;
        self.nx = nx;
        self.zy = zy;
        self.ny = ny;
        self.f = f;
        self.no = no;

        self.eval();

        (self.out, self.zr, self.ng)
    }

    pub fn set_x(&mut self, x: Signal16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: Signal16) {
        self.y = y;
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }
}

impl Unit for Alu {
    fn eval(&mut self) {
        // Zero or X
        let x_zero_mux_out = self.x_zero_mux.sim(self.x, Signal16::FALSE, self.zx);

        // Now not X
        let x_negx_mux_out =
            self.x_negx_mux
                .sim(x_zero_mux_out, self.not_x.sim(x_zero_mux_out), self.nx);

        // Zero or Y
        let y_zero_mux_out = self.y_zero_mux.sim(self.y, Signal16::FALSE, self.zy);

        // Now not Y
        let y_negy_mux_out =
            self.y_negy_mux
                .sim(y_zero_mux_out, self.not_y.sim(y_zero_mux_out), self.ny);

        // Select between And and Add
        let f_mux_out = self.f_mux.sim(
            self.f_and.sim(x_negx_mux_out, y_negy_mux_out),
            self.f_add.sim(x_negx_mux_out, y_negy_mux_out),
            self.f,
        );

        // Select between Out and !Out
        let o_nego_mux_out = self
            .o_nego_mux
            .sim(f_mux_out, self.not_o.sim(f_mux_out), self.no);

        self.out = o_nego_mux_out;

        self.zr = self.not_ors.sim(self.ors_o_zero.sim(o_nego_mux_out));

        self.ng = self.out.get(15..16).into();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut alu = Alu::default();

        alu.eval();
        assert_eq!(alu.out, Signal16::FALSE);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        alu.x = Signal16::from(0); // x = 0
        alu.y = Signal16::from(-1); // y = -1

        // Compute 0
        alu.zx = Signal::HI;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, Signal16::FALSE);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, Signal16::from(1));
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute -1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, Signal16::from(-1));
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute !x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, !alu.x);
        assert_eq!(alu.out, Signal16::TRUE);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute !y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, !alu.y);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute -x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, -alu.x);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute -y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, -alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x + 1
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x + 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y + 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.y + 1);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x - 1
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x - 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute y - 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.y - 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x + y
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x + alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x - y
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x - alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y - x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.y - alu.x);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x & y
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x & alu.y);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x | y
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x | alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        alu.x = Signal16::from(0b000000000010001i16); // x = 17
        alu.y = Signal16::from(0b000000000000011i16); // y =  3

        // Compute 0
        alu.zx = Signal::HI;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, Signal16::FALSE);
        assert_eq!(alu.zr, Signal::HI);
        assert_eq!(alu.ng, Signal::LO);

        // Compute 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, Signal16::from(1));
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute -1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, Signal16::from(-1));
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute !x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, !alu.x);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute !y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, !alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute -x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, -alu.x);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute -y
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, -alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x + 1
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x + 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y + 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.y + 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x - 1
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::HI;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x - 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y - 1
        alu.zx = Signal::HI;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.y - 1);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x + y
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x + alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x - y
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x - alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute y - x
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::HI;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.y - alu.x);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::HI);

        // Compute x & y
        alu.zx = Signal::LO;
        alu.nx = Signal::LO;
        alu.zy = Signal::LO;
        alu.ny = Signal::LO;
        alu.f = Signal::LO;
        alu.no = Signal::LO;
        alu.eval();
        assert_eq!(alu.out, alu.x & alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);

        // Compute x | y
        alu.zx = Signal::LO;
        alu.nx = Signal::HI;
        alu.zy = Signal::LO;
        alu.ny = Signal::HI;
        alu.f = Signal::LO;
        alu.no = Signal::HI;
        alu.eval();
        assert_eq!(alu.out, alu.x | alu.y);
        assert_eq!(alu.zr, Signal::LO);
        assert_eq!(alu.ng, Signal::LO);
    }
}
