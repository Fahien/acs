// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::code::VmCode;

/// Processes one or more VM code units into a vector of processed string lines.
/// Ideally you first create a `builder`, then include as many `VmCode`s you need,
/// and finally you `build` to obtain a `VmPreprocessedCode`.
#[derive(Default)]
pub struct VmPreprocessedCode {
    lines: Vec<String>,
}

impl VmPreprocessedCode {
    pub fn builder() -> VmPreprocessedCodeBuilder {
        VmPreprocessedCodeBuilder::default()
    }

    pub fn get_lines(self) -> Vec<String> {
        self.lines
    }
}

#[derive(Default)]
pub struct VmPreprocessedCodeBuilder {
    pub lines: Vec<String>,

    /// Static variables limit is 240
    next_static_value: u8,
    statics: HashMap<String, u8>,
}

impl VmPreprocessedCodeBuilder {
    fn get_or_create_static(&mut self, static_id: String) -> u8 {
        if let Some(value) = self.statics.get(&static_id) {
            *value
        } else {
            let next_static_value = self.next_static_value;
            self.statics.insert(static_id, next_static_value);
            self.next_static_value += 1;
            next_static_value
        }
    }

    pub fn include(mut self, code: VmCode) -> Self {
        let current_lines: Vec<String> = code
            .code
            .lines()
            .map(|line| {
                // Ignore comments at the end of the line
                let line = if let Some(index) = line.find("//") {
                    &line[0..index]
                } else {
                    line
                };
                line.trim_matches(|c| c == ' ' || c == '\n' || c == '\r' || c == '\t')
            })
            .filter_map(|line| {
                if line.starts_with("pop static ") {
                    let static_num = line.strip_prefix("pop static ").unwrap();
                    let id = format!("{}{}", code.name, static_num);
                    let num = self.get_or_create_static(id);
                    Some(format!("pop static {}", num))
                } else if line.starts_with("push static ") {
                    let static_num = line.strip_prefix("push static ").unwrap();
                    let id = format!("{}{}", code.name, static_num);
                    let num = self.get_or_create_static(id);
                    Some(format!("push static {}", num))
                } else if !line.is_empty() {
                    Some(line.into())
                } else {
                    None
                }
            })
            .collect();

        self.lines.extend(current_lines);
        self
    }

    pub fn include_all(mut self, codes: Vec<VmCode>) -> Self {
        for code in codes {
            self = self.include(code);
        }
        self
    }

    pub fn build(self) -> VmPreprocessedCode {
        VmPreprocessedCode { lines: self.lines }
    }
}
