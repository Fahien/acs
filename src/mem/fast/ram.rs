// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

macro_rules! impl_fast_ram {
    ($ram:ident, $size:expr, $address:ty) => {
        use crate::{Signal, Signal16, Unit};
        use std::ops::{Index, IndexMut};

        /// Fast RAM which, as opposed to the other RAMs,
        /// it does not carries out the simulation with other building blocks,
        /// but it emulates it with native code.
        pub struct $ram {
            inp: Signal16,
            load: Signal,
            address: $address,
            out: Signal16,

            selected: usize,
            pub data: Vec<Signal16>,
        }

        impl Default for $ram {
            fn default() -> Self {
                Self {
                    inp: Default::default(),
                    load: Default::default(),
                    address: Default::default(),
                    out: Default::default(),
                    selected: Default::default(),
                    data: vec![Signal16::default(); $size],
                }
            }
        }

        impl $ram {
            /// Carries out the simulation, taking as input:
            ///
            /// - `inp`: data input
            /// - `load`: whether to write `inp` into the RAM
            /// - `address`: which register to select
            ///
            /// It returns the value stored at `address. Furthermore, when `load` is
            /// `HI`, `inp` is written at location `address`, and the loaded value will
            /// be emitted from the next time step onward.
            pub fn sim(
                &mut self,
                inp: Signal16,
                load: Signal,
                address: impl Into<$address>,
            ) -> Signal16 {
                self.inp = inp;
                self.load = load;
                self.address = address.into();
                self.tick();
                self.out
            }

            pub fn set_inp(&mut self, inp: impl Into<Signal16>) {
                self.inp = inp.into();
            }

            pub fn set_load(&mut self, load: impl Into<Signal>) {
                self.load = load.into();
            }

            pub fn set_address(&mut self, address: impl Into<$address>) {
                self.address = address.into();
            }

            pub fn out(&self) -> Signal16 {
                self.out
            }
        }

        impl Unit for $ram {
            fn eval(&mut self) {
                self.selected = self.address.into();
                self.out = self.data[self.selected];
            }

            fn tick(&mut self) {
                self.selected = self.address.into();
                self.out = self.data[self.selected];
                if self.load.as_bool() {
                    self.data[self.selected] = self.inp;
                }
            }
        }

        impl Index<usize> for $ram {
            type Output = i16;

            fn index(&self, index: usize) -> &Self::Output {
                self.data[index].get_values()
            }
        }

        impl IndexMut<usize> for $ram {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                self.data[index].get_values_mut()
            }
        }

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn run() {
                let mut ram = $ram::default();
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
    };
}
