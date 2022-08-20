// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    logic::fast::Alu,
    mem::fast::{Counter, Register16},
    Signal, Signal15, Signal16, Unit,
};

/// Fast Central Processing Unit able to execute a given instruction and
/// to determine which instruction should be fetched and executed next
#[derive(Default)]
pub struct Cpu {
    pub in_m: Signal16,
    pub instruction: Signal16,
    pub reset: Signal,

    pub out_m: Signal16,
    pub write_m: Signal,
    pub address_m: Signal15,
    pub pc: Signal16,

    pub a_register: Register16,
    pub d_register: Register16,

    alu: Alu,
    counter: Counter,
}

impl Cpu {
    /// Carries out the simulation, taking as input:
    ///
    /// - `in_m`: data from memory
    /// - `instruction`: fetched from ROM
    /// - `reset`: whether to set the program counter to zero
    ///
    /// It returns the following:
    /// - `write_m`: whether to write to memory
    /// - `out_m`: data going to memory
    /// - `address_m`: memory address
    /// - `pc`: program counter
    pub fn sim(
        &mut self,
        in_m: Signal16,
        instruction: Signal16,
        reset: Signal,
    ) -> (Signal, Signal16, Signal15, Signal16) {
        self.in_m = in_m;
        self.instruction = instruction;
        self.reset = reset;
        self.tick();
        (self.write_m, self.out_m, self.address_m, self.pc)
    }

    pub fn get_pc(&self) -> Signal16 {
        self.pc
    }

    pub fn get_address_m(&self) -> Signal15 {
        self.address_m
    }

    pub fn get_out_m(&self) -> Signal16 {
        self.out_m
    }

    pub fn get_write_m(&self) -> Signal {
        self.write_m
    }
}

impl Unit for Cpu {
    fn tock(&mut self) {
        self.eval();
    }

    fn tick(&mut self) {
        // jjj
        let pos_jump: Signal = self.instruction.get(0..1).into();
        let zr_jump: Signal = self.instruction.get(1..2).into();
        let neg_jump: Signal = self.instruction.get(2..3).into();

        // ddd
        let write_m_signal: Signal = self.instruction.get(3..4).into();
        let load_d_signal = self.instruction.get(4..5).into();
        let load_a_signal = self.instruction.get(5..6).into();

        // cccccc
        let no: Signal = self.instruction.get(6..7).into();
        let f: Signal = self.instruction.get(7..8).into();
        let ny: Signal = self.instruction.get(8..9).into();
        let zy: Signal = self.instruction.get(9..10).into();
        let nx: Signal = self.instruction.get(10..11).into();
        let zx: Signal = self.instruction.get(11..12).into();
        let sel_m: Signal = self.instruction.get(12..13).into();

        let is_c_instruction = self.instruction.get(15..16).into();

        let a_register_out = self.a_register.out();

        // Write and address output signals are quite easy
        self.write_m = write_m_signal & is_c_instruction;
        self.address_m = a_register_out.into();

        // Zero if this is an A-instruction
        let actual_no = no & is_c_instruction;
        let actual_f = f & is_c_instruction;
        let actual_ny = ny & is_c_instruction;
        let actual_zy = zy & is_c_instruction;

        // Just negate is_c_instruction
        let is_a_instruction_out = !is_c_instruction;

        // One if this is an A-instruction
        let actual_nx = nx | is_a_instruction_out;
        let actual_zx = zx | is_a_instruction_out;

        // Zero if this is an A-instruction
        // so to feed the ALU with the A register instead of M
        let actual_sel_m = sel_m & is_c_instruction;

        self.pc = self.counter.out();

        let alu_input_x = self.d_register.out();
        let alu_input_y = if actual_sel_m.as_bool() {
            self.in_m
        } else {
            a_register_out
        };

        let (alu_out, alu_zr, alu_ng) = self.alu.sim(
            alu_input_x,
            alu_input_y,
            actual_zx,
            actual_nx,
            actual_zy,
            actual_ny,
            actual_f,
            actual_no,
        );

        self.out_m = alu_out;

        // Load D when this is an C instruction and the D signal is true
        self.d_register
            .sim(alu_out, is_c_instruction & load_d_signal);

        // Figure out A input after evaluating the ALU
        let a_input = if is_c_instruction.as_bool() {
            alu_out
        } else {
            self.instruction
        };

        // Load A when this is an A instruction or A signal is true
        self.a_register
            .sim(a_input, is_a_instruction_out | load_a_signal);

        // Counter logic
        let comp_not_zr = !alu_zr;
        let comp_not_ng = !alu_ng;
        let comp_gt_zr = comp_not_zr & comp_not_ng;

        let jump_gt_zr = pos_jump & comp_gt_zr;
        let jump_eq_zr = zr_jump & alu_zr;
        let jump_lt_zr = neg_jump & alu_ng;

        // Jump>0 || Jump==0 || Jump<0
        let jump_or0 = jump_gt_zr | jump_eq_zr;
        let jump_or1 = jump_or0 | jump_lt_zr;

        let load_jump = jump_or1 & is_c_instruction;

        // Maybe jump to address in A register
        self.counter
            .sim(a_register_out, self.reset, load_jump, Signal::HI);
    }

    fn eval(&mut self) {
        self.a_register.eval();
        self.d_register.eval();

        self.address_m = self.a_register.out().into();

        let sel_m: Signal = self.instruction.get(12..13).into();
        let is_c_instruction = self.instruction.get(15..16).into();
        let actual_sel_m = sel_m & is_c_instruction;
        let alu_input_y = if actual_sel_m.as_bool() {
            self.in_m
        } else {
            self.a_register.out()
        };

        self.alu.set_x(self.d_register.out());
        self.alu.set_y(alu_input_y);

        self.alu.eval();
        self.out_m = self.alu.out();

        self.counter.eval();
        self.pc = self.counter.out();
    }
}
