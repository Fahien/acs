// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

/// Common operations for a logic unit
pub trait Unit {
    /// Evaluates the logic expression as we do with arithmetic
    fn eval(&mut self);

    /// Carries out the tick-phase of a clock cycle: LOW
    fn tick(&mut self) {
        self.eval()
    }

    /// Carries out the tock-phase of a clock cycle: HIGH
    fn tock(&mut self) {
        self.eval()
    }
}
