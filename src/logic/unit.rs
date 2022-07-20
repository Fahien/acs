// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

/// Common operations for a logic unit.
///
/// Calling `tick()` and `tock()` determines a cycle,
/// and each cycle is one discrete time unit `t`.
pub trait Unit {
    /// Evaluates the logic expression as we do with arithmetic
    /// without advancing the time.
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
