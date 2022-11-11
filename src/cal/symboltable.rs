// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{segment::Segment, structure::Variable};

pub struct SymbolEntry {
    pub variable: Variable,
    pub segment: Segment,
    pub offset: u16,
}

impl SymbolEntry {
    pub fn new(variable: Variable, segment: Segment, offset: u16) -> Self {
        Self {
            variable,
            segment,
            offset,
        }
    }
}

#[derive(Default)]
/// Each scope has its own symbol table, with its own number of local variables
pub struct SymbolTable {
    local_count: u16,
    argument_count: u16,
    variables: HashMap<String, SymbolEntry>,
}

impl SymbolTable {
    /// Inserts a new local variable in the symbol table and
    /// returns the index of the newly inserted variable
    pub fn insert_local(&mut self, variable: &Variable) -> u16 {
        let local_number = self.local_count;
        let entry = SymbolEntry::new(variable.clone(), Segment::Local, local_number);
        self.variables.insert(variable.name.clone(), entry);
        self.local_count += 1;
        local_number
    }

    /// Inserts a new argument variable in the symbol table and
    /// returns the index of the newly inserted variable
    pub fn insert_argument(&mut self, variable: &Variable) {
        let argument_number = self.argument_count;
        let entry = SymbolEntry::new(variable.clone(), Segment::Argument, argument_number);
        self.variables.insert(variable.name.clone(), entry);
        self.argument_count += 1;
    }

    /// Returns the segment and the offset of the variable with that `name`
    pub fn get_segment_and_offset(&self, name: &str) -> (Segment, u16) {
        let entry = self.variables.get(name).unwrap();
        (entry.segment, entry.offset)
    }
}
