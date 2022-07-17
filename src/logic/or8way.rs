// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use super::{Or, Signal, Signal8, Unit};

/// 8-way Or gate
#[derive(Default)]
pub struct Or8Way {
    inp: Signal8,
    out: Signal,

    ors: [Or; 8],
}

impl Or8Way {
    /// Carries out the simulation and returns `inp[0] OR inp[1] OR .. OR inp[7]`
    pub fn sim(&mut self, inp: Signal8) -> Signal {
        self.inp = inp;
        self.eval();
        self.out
    }
}

impl Unit for Or8Way {
    fn eval(&mut self) {
        let mut ors_out = [Signal::LO; 7];
        ors_out[0] = self.ors[0].sim(self.inp.get(0).into(), self.inp.get(1).into());

        for i in 1..7 {
            let prev_or_out = ors_out[i - 1];
            ors_out[i] = self.ors[i].sim(prev_or_out, self.inp.get(i + 1).into());
        }

        self.out = ors_out[6];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut or = Or8Way::default();

        or.sim(Signal8::FALSE);
        assert_eq!(or.out, Signal::LO);

        or.inp = Signal8::TRUE;
        or.sim(Signal8::TRUE);
        assert_eq!(or.out, Signal::HI);

        or.sim(Signal8::from(0b00010000));
        assert_eq!(or.out, Signal::HI);

        or.sim(Signal8::from(0b1));
        assert_eq!(or.out, Signal::HI);

        or.sim(Signal8::from(0b100110));
        assert_eq!(or.out, Signal::HI);
    }
}
