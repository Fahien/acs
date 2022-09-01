// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    mem::fast::Ram16k,
    vm::{instruction::VmInstruction, segment::Segment},
    Signal16,
};

/// This emulator implements all the VM commands with Rust operating
/// on its own instance of a memory
pub struct VmEmulator {
    instructions: Vec<VmInstruction>,
    instruction_index: usize,

    pub ram: Ram16k,
}

impl Default for VmEmulator {
    fn default() -> Self {
        let mut ram = Ram16k::default();
        ram.data[0] = 256.into();

        Self {
            instructions: Default::default(),
            instruction_index: 0,
            ram,
        }
    }
}

impl VmEmulator {
    /// Loads a program into the emulator
    pub fn load(&mut self, instructions: Vec<VmInstruction>) {
        self.instructions = instructions;
    }

    pub fn set(&mut self, segment: Segment, value: Signal16) {
        self.ram.data[segment.get_base_address()] = value;
    }

    /// Pushes `value` on top of the stack
    pub fn push(&mut self, value: Signal16) {
        let stack_pointer = &mut self.ram.data[Segment::Stack.get_base_address()];
        let stack_address: usize = stack_pointer.into();
        *stack_pointer += 1;
        self.ram.data[stack_address] = value;
    }

    /// Pushes `segment[offset]` on top of the stack
    pub fn push_from(&mut self, segment: Segment, offset: Signal16) {
        let value = match segment {
            Segment::Constant => offset,
            Segment::Pointer | Segment::Temp | Segment::Static => {
                let offset: usize = offset.into();
                let segment_address: usize = segment.get_base_address() + offset;
                self.ram.data[segment_address]
            }
            _ => {
                let segment_pointer = self.ram.data[segment.get_base_address()];
                let segment_address: usize = (segment_pointer + offset).into();
                self.ram.data[segment_address]
            }
        };
        self.push(value);
    }

    /// Pops and returns the topmost element of the stack
    pub fn pop(&mut self) -> Signal16 {
        let stack_pointer = &mut self.ram.data[Segment::Stack.get_base_address()];
        *stack_pointer -= 1;
        let stack_address: usize = stack_pointer.into();
        self.ram.data[stack_address]
    }

    /// Pops an element from the stack and stores it into `segment[offset]`
    pub fn pop_into(&mut self, segment: Segment, offset: Signal16) {
        let element = self.pop();

        let segment_address: usize = match segment {
            Segment::Pointer | Segment::Static | Segment::Temp => {
                let segment_pointer = segment.get_base_address();
                segment_pointer + usize::from(offset)
            }
            _ => {
                let segment_pointer = self.ram.data[segment.get_base_address()];
                (segment_pointer + offset).into()
            }
        };

        self.ram.data[segment_address] = element;
    }

    /// Fetches the next instruction and executes it
    pub fn step(&mut self) {
        if self.instruction_index >= self.instructions.len() {
            return;
        }

        let instruction = self.instructions[self.instruction_index];

        match instruction {
            VmInstruction::Push(segment, offset) => self.push_from(segment, offset.into()),
            VmInstruction::Pop(segment, offset) => self.pop_into(segment, offset.into()),
            VmInstruction::Add => {
                let value = self.pop() + self.pop();
                self.push(value);
            }
            VmInstruction::Sub => {
                let b = self.pop();
                let a = self.pop();
                let value = a - b;
                self.push(value);
            }
            VmInstruction::And => {
                let value = self.pop() & self.pop();
                self.push(value);
            }
            VmInstruction::Or => {
                let value = self.pop() | self.pop();
                self.push(value);
            }
            VmInstruction::Neg => {
                let value = -self.pop();
                self.push(value);
            }
            VmInstruction::Not => {
                let value = !self.pop();
                self.push(value);
            }
            VmInstruction::Eq => {
                let value = self.pop() == self.pop();
                self.push(value.into());
            }
            VmInstruction::Lt => {
                // Simply invert the order as we pop from the stack
                let value = self.pop() > self.pop();
                self.push(value.into());
            }
            VmInstruction::Gt => {
                // Simply invert the order as we pop from the stack
                let value = self.pop() < self.pop();
                self.push(value.into());
            }
        };

        self.instruction_index += 1;
    }
}
