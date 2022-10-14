// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

/// A VmCode unit represents a file with VM instructions
pub struct VmCode {
    /// Name of the file
    pub name: String,

    /// VM instructions read from the file
    pub code: String,
}

impl VmCode {
    pub fn new(name: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            code: code.into(),
        }
    }
}
