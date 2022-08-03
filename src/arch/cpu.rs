// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    mem::fast::{Counter, Register16},
    Alu, And, Mux16, Not, Or, Signal, Signal15, Signal16, Time, Unit,
};

/// Central Processing Unit able to execute a given instruction and to determine
/// which instruction should be fetched and executed next
#[derive(Default)]
pub struct Cpu {
    time: Time,

    in_m: Signal16,
    instruction: Signal16,
    reset: Signal,

    out_m: Signal16,
    write_m: Signal,
    address_m: Signal15,
    pc: Signal16,

    a_register: Register16,
    d_register: Register16,

    actual_no: And,
    actual_f: And,
    actual_ny: And,
    actual_zy: And,
    actual_nx: Or,
    actual_zx: Or,

    actual_sel_m: And,

    load_a: Or,
    load_d: And,
    is_a_instruction: Not,

    mux_a_m: Mux16,
    alu: Alu,
    counter: Counter,
    write_m_if_c: And,

    mux_instruction_out: Mux16,

    // Jump logic
    comp_not_zr: Not,
    comp_not_ng: Not,
    comp_gt_zr: And,
    jump_gt_zr: And,
    jump_eq_zr: And,
    jump_lt_zr: And,

    jump_or: [Or; 2],

    load_jump: And,
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
}

impl Unit for Cpu {
    fn tock(&mut self) {
        self.time += 1;
        self.eval();
    }

    fn tick(&mut self) {
        self.time += 1;

        // jjj
        let pos_jump: Signal = self.instruction.get(0..1).into();
        let zr_jump: Signal = self.instruction.get(1..2).into();
        let neg_jump: Signal = self.instruction.get(2..3).into();

        // ddd
        let write_m_signal = self.instruction.get(3..4).into();
        let load_d_signal = self.instruction.get(4..5).into();
        let load_a_signal = self.instruction.get(5..6).into();

        // cccccc
        let no = self.instruction.get(6..7).into();
        let f = self.instruction.get(7..8).into();
        let ny = self.instruction.get(8..9).into();
        let zy = self.instruction.get(9..10).into();
        let nx = self.instruction.get(10..11).into();
        let zx = self.instruction.get(11..12).into();
        let sel_m = self.instruction.get(12..13).into();

        let is_c_instruction = self.instruction.get(15..16).into();

        let a_register_out = self.a_register.out();

        // Write and address output signals are quite easy
        self.write_m = self.write_m_if_c.sim(write_m_signal, is_c_instruction);
        self.address_m = a_register_out.into();

        // Zero if this is an A-instruction
        let actual_no = self.actual_no.sim(no, is_c_instruction);
        let actual_f = self.actual_f.sim(f, is_c_instruction);
        let actual_ny = self.actual_ny.sim(ny, is_c_instruction);
        let actual_zy = self.actual_zy.sim(zy, is_c_instruction);

        // Just negate is_c_instruction
        let is_a_instruction_out = self.is_a_instruction.sim(is_c_instruction);

        // One if this is an A-instruction
        let actual_nx = self.actual_nx.sim(nx, is_a_instruction_out);
        let actual_zx = self.actual_zx.sim(zx, is_a_instruction_out);

        // Zero if this is an A-instruction
        // so to feed the ALU with the A register instead of M
        let actual_sel_m = self.actual_sel_m.sim(sel_m, is_c_instruction);

        self.pc = self.counter.out();

        let alu_input_x = self.d_register.out();
        let alu_input_y = self.mux_a_m.sim(a_register_out, self.in_m, actual_sel_m);

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
            .sim(alu_out, self.load_d.sim(is_c_instruction, load_d_signal));

        // Figure out A input after evaluating the ALU
        let a_input = self
            .mux_instruction_out
            .sim(self.instruction, alu_out, is_c_instruction);

        // Load A when this is an A instruction or A signal is true
        self.a_register.sim(
            a_input,
            self.load_a.sim(is_a_instruction_out, load_a_signal),
        );

        // Counter logic
        let comp_not_zr = self.comp_not_zr.sim(alu_zr);
        let comp_not_ng = self.comp_not_ng.sim(alu_ng);
        let comp_gt_zr = self.comp_gt_zr.sim(comp_not_zr, comp_not_ng);

        let jump_gt_zr = self.jump_gt_zr.sim(pos_jump, comp_gt_zr);
        let jump_eq_zr = self.jump_eq_zr.sim(zr_jump, alu_zr);
        let jump_lt_zr = self.jump_lt_zr.sim(neg_jump, alu_ng);

        // Jump>0 || Jump==0 || Jump<0
        let jump_or0 = self.jump_or[0].sim(jump_gt_zr, jump_eq_zr);
        let jump_or1 = self.jump_or[1].sim(jump_or0, jump_lt_zr);

        let load_jump = self.load_jump.sim(jump_or1, is_c_instruction);

        // Maybe jump to address in A register
        self.counter
            .sim(a_register_out, self.reset, load_jump, Signal::HI);
    }

    fn eval(&mut self) {
        self.a_register.eval();
        self.d_register.eval();

        self.address_m = self.a_register.out().into();

        self.mux_a_m.set_a(self.a_register.out());
        self.mux_a_m.eval();

        self.alu.set_x(self.d_register.out());
        self.alu.set_y(self.mux_a_m.out());

        self.alu.eval();
        self.out_m = self.alu.out();

        self.counter.eval();
        self.pc = self.counter.out();
    }
}
