// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{Signal16, Unit};

/// Built-in register chip acting as a keyboard
#[derive(Default)]
pub struct Keyboard {
    out: Signal16,
}

impl Keyboard {
    pub fn set(&mut self, value: Signal16) {
        self.out = value;
    }

    /// Returns the 16-bit character code of the currently pressed key on the
    /// physical board, or `0` if no key is pressed
    pub fn out(&self) -> Signal16 {
        self.out
    }
}

impl Unit for Keyboard {
    fn eval(&mut self) {}
}
