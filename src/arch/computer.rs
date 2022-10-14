// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    arch::fast::{Cpu, Memory},
    asm::instruction::AsmInstruction,
    Keyboard, Rom32k, Screen, Signal, Time, Unit,
};

/// The computer consists of three parts:
/// - Central Processing Unit (CPU)
/// - Memory (RAM)
/// - Instruction Memory (ROM)
///
/// This configuration with two separate address space for instructions and data
/// is called _Hardware architecture_.
///
/// The intruction memory `out` signal is feed to the `instruction` input of the
/// CPU, which is then executed. This execution may involve reading or writing
/// data to memory registers. The CPU also figures out the instruction to fetch
/// next and emits this address through its `pc` output. The `pc` output is feed
/// to the `address` input of the instruction memory.
#[derive(Default)]
pub struct Computer {
    time: Time,
    reset: Signal,

    rom: Rom32k,
    cpu: Cpu,
    memory: Memory,
}

impl Computer {
    /// Loads instructions into the ROM
    pub fn set_instructions(&mut self, instructions: Vec<AsmInstruction>) {
        self.rom.set_instructions(instructions);
    }

    pub fn reset(&mut self) {
        self.reset = Signal::HI;
    }

    pub fn get_screen(&self) -> &Screen {
        self.memory.get_screen()
    }

    pub fn get_keyboard_mut(&mut self) -> &mut Keyboard {
        self.memory.get_keyboard_mut()
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    /// Advances one cicle
    pub fn ticktock(&mut self) {
        self.tick();
        self.tock();
    }
}

impl Unit for Computer {
    fn tick(&mut self) {
        self.time += 1;

        self.rom.set_address(self.cpu.get_pc());
        self.rom.eval();
        self.memory.inp = self.cpu.get_out_m();
        self.memory.load = self.cpu.get_write_m();
        self.memory.address = self.cpu.get_address_m();
        self.memory.eval();

        let (write_m, out_m, address_m, pc) =
            self.cpu.sim(self.memory.out(), self.rom.out(), self.reset);

        self.rom.sim(pc.into());
        self.memory.sim(out_m, write_m, address_m);
    }

    fn tock(&mut self) {
        self.time += 1;
        self.eval();
    }

    fn eval(&mut self) {
        self.cpu.eval();
        self.rom.set_address(self.cpu.get_pc());
        self.rom.eval();
        self.memory.inp = self.cpu.get_out_m();
        self.memory.load = self.cpu.get_write_m();
        self.memory.address = self.cpu.get_address_m();
        self.memory.eval();
    }
}
