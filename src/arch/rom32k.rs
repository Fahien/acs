// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{asm::instruction::AsmInstruction, Signal15, Signal16, Unit};

/// Instruction memory, preloaded with the required program.
pub struct Rom32k {
    address: Signal15,
    out: Signal16,

    instructions: Vec<AsmInstruction>,
}

impl Rom32k {
    /// Carries out the simulation taking as input an address and
    /// returns the corresponding instruction
    pub fn sim(&mut self, address: Signal15) -> Signal16 {
        self.address = address;
        self.eval();
        self.out
    }

    pub fn out(&self) -> Signal16 {
        self.out
    }

    pub fn set_address(&mut self, address: impl Into<Signal15>) {
        self.address = address.into();
    }

    pub fn set_instructions(&mut self, mut instructions: Vec<AsmInstruction>) {
        instructions.resize(self.instructions.len(), AsmInstruction::default());
        self.instructions.clone_from_slice(&instructions);
    }

    pub fn get_index(&self) -> usize {
        let index: usize = self.address.into();
        assert!(index < (1 << 15));
        index
    }

    pub fn get_instruction(&self) -> &AsmInstruction {
        &self.instructions[self.get_index()]
    }
}

impl Default for Rom32k {
    fn default() -> Self {
        Self {
            address: Default::default(),
            out: Default::default(),
            instructions: vec![AsmInstruction::default(); 1 << 15],
        }
    }
}

impl Unit for Rom32k {
    fn eval(&mut self) {
        self.out = Signal16::from(self.get_instruction());
    }
}
