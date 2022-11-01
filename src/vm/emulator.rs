// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{
    mem::fast::Ram16k,
    vm::{instruction::VmInstruction, segment::Segment},
    Signal16,
};

/// This emulator implements all the VM commands with Rust operating
/// on its own instance of a memory
pub struct VmEmulator {
    instructions: Vec<VmInstruction>,
    /// Current instruction index
    instruction_index: usize,

    pub ram: Ram16k,

    /// The symbol table maps labels (and function names) to their indices in
    /// the code so that we can jump to them when needed
    symbol_table: HashMap<String, usize>,
}

impl Default for VmEmulator {
    fn default() -> Self {
        let mut ram = Ram16k::default();
        ram.data[0] = 256.into();

        Self {
            instructions: Default::default(),
            instruction_index: 0,
            ram,
            symbol_table: Default::default(),
        }
    }
}

impl VmEmulator {
    /// Loads a program into the emulator
    pub fn load(&mut self, instructions: Vec<VmInstruction>) {
        self.instructions = instructions;

        // Populate symbol table
        for (num, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                VmInstruction::Label(label) | VmInstruction::Function(label, ..) => {
                    self.symbol_table.insert(label.clone(), num);
                }
                _ => (),
            }
        }
    }

    pub fn get_segment_address(&self, segment: Segment) -> i16 {
        match segment {
            Segment::Pointer | Segment::Static | Segment::Temp => {
                let segment_pointer = segment.get_base_address();
                segment_pointer as i16
            }
            _ => {
                let segment_pointer = self.ram.data[segment.get_base_address()];
                i16::from(segment_pointer)
            }
        }
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
        let segment_address = self.get_segment_address(segment) + i16::from(offset);
        self.ram.data[segment_address as usize] = element;
    }

    /// Fetches the next instruction and executes it
    pub fn step(&mut self) {
        if self.instruction_index >= self.instructions.len() {
            return;
        }

        let instruction = self.instructions[self.instruction_index].clone();

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
            VmInstruction::Label(_) => (), // already processed on load
            VmInstruction::Goto(label) => {
                self.instruction_index = *self.symbol_table.get(&label).unwrap();
            }
            VmInstruction::IfGoto(label) => {
                let value = self.pop();
                if value != Signal16::FALSE {
                    self.instruction_index = *self.symbol_table.get(&label).unwrap();
                }
            }
            VmInstruction::Function(_function, local_count) => {
                // Allocate enough space on the stack for the local segment of this function
                for _ in 0..local_count {
                    self.push(0.into());
                }
            }
            VmInstruction::Call(function, arg_count) => {
                // Save current function's state by storing some important
                // values onto the stack. Important consideration at this point:
                // arguments for the function we are calling are already on the stack

                // Save current instruction index so that we can return from
                // the function we are calling
                let return_address = self.instruction_index as i16;
                self.push(return_address.into());

                // Save local pointer
                let caller_local_pointer = self.ram.data[Segment::Local.get_base_address()];
                self.push(caller_local_pointer);

                // Save argument pointer
                let caller_arg_pointer = self.ram.data[Segment::Argument.get_base_address()];
                self.push(caller_arg_pointer);

                // Save this pointer
                let caller_this_pointer = self.ram.data[Segment::This.get_base_address()];
                self.push(caller_this_pointer);

                // Save that pointer
                let caller_that_pointer = self.ram.data[Segment::That.get_base_address()];
                self.push(caller_that_pointer);

                // Set new argument pointer
                let stack_pointer = self.ram[Segment::Stack.get_base_address()];
                self.ram[Segment::Argument.get_base_address()] =
                    stack_pointer - 5 - arg_count as i16;

                // Set new local pointer
                self.ram[Segment::Local.get_base_address()] = stack_pointer;

                let function_address = self.symbol_table.get(&function).unwrap();
                // We want to execute the "function" instruction so
                // subtract one, as it will be incremented later.
                self.instruction_index = *function_address - 1;
            }
            VmInstruction::Return(return_size_in_words) => {
                // Get current local address
                let lcl = self.ram[Segment::Local.get_base_address()] as usize;

                // Get return address
                let return_address = self.ram[lcl - 5] as usize;
                self.instruction_index = return_address;

                // Stack pointer is set to argument pointer plus the size of
                // the return value (as we have stored it at the beginning of
                // the argument section)
                let current_arg_address = self.ram[Segment::Argument.get_base_address()];

                // Restore local pointer: LCL = *(lcl-4)
                self.ram[Segment::Local.get_base_address()] = self.ram[lcl - 4];

                // Restore argument pointer: ARG = *(lcl-3)
                self.ram[Segment::Argument.get_base_address()] = self.ram[lcl - 3];

                // Restore this pointer: THIS = *(lcl-2)
                self.ram[Segment::This.get_base_address()] = self.ram[lcl - 2];

                // Restore that pointer: THAT = *(lcl-1)
                self.ram[Segment::That.get_base_address()] = self.ram[lcl - 1];

                // Overwrite argument section with the return value
                for i in 0..return_size_in_words {
                    let offset = return_size_in_words - 1 - i;
                    let signal = self.pop();
                    self.ram.data[current_arg_address as usize + offset as usize] = signal;
                }

                // Set stack pointer after popping return value
                self.ram[Segment::Stack.get_base_address()] =
                    current_arg_address + return_size_in_words as i16;
            }
        };

        self.instruction_index += 1;
    }
}
