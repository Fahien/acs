// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{segment::Segment, vm::instruction::VmInstruction};

/// The first part of the preamble is responsible of calling the main function
/// and going into and endless loop when returning
fn sys() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Call("main".into(), 0),
        VmInstruction::Label("END".into()),
        VmInstruction::Goto("END".into()),
    ]
}

/// The peek function is added at the beginning of the program and it can be
/// used to read a word (2 bytes) from an address in memory
fn peek() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Function("peek".into(), 0),
        VmInstruction::Push(Segment::Argument, 0),
        VmInstruction::Pop(Segment::Pointer, 0),
        VmInstruction::Push(Segment::This, 0),
        VmInstruction::Return(1),
    ]
}

/// The poke function is added at the beginning of the program and it can be
/// used to write a word (2 bytes) at an address in memory
fn poke() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Function("poke".into(), 0),
        VmInstruction::Push(Segment::Argument, 0),
        VmInstruction::Pop(Segment::Pointer, 0),
        VmInstruction::Push(Segment::Argument, 1),
        VmInstruction::Pop(Segment::This, 0),
        VmInstruction::Return(0),
    ]
}

/// Generates a bunch of instructions for the built-in multiplication function
fn mul() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Function("mul".into(), 2),
        VmInstruction::Push(Segment::Constant, 0),
        VmInstruction::Pop(Segment::Local, 0), // sum
        VmInstruction::Push(Segment::Constant, 0),
        VmInstruction::Pop(Segment::Local, 1), // i
        VmInstruction::Label("MUL_WHILE".into()),
        VmInstruction::Push(Segment::Local, 1),
        VmInstruction::Push(Segment::Argument, 1), // y
        VmInstruction::Lt,
        VmInstruction::Not,
        VmInstruction::IfGoto("MUL_END".into()),
        VmInstruction::Push(Segment::Local, 0),
        VmInstruction::Push(Segment::Argument, 0), // x
        VmInstruction::Add,
        VmInstruction::Pop(Segment::Local, 0),
        VmInstruction::Push(Segment::Local, 1),
        VmInstruction::Push(Segment::Constant, 1),
        VmInstruction::Add,
        VmInstruction::Pop(Segment::Local, 1),
        VmInstruction::Goto("MUL_WHILE".into()),
        VmInstruction::Label("MUL_END".into()),
        VmInstruction::Push(Segment::Local, 0),
        VmInstruction::Return(1),
    ]
}

/// Generates a bunch of instructions for the built-in division function
fn div() -> Vec<VmInstruction> {
    vec![
        VmInstruction::Function("div".into(), 2),
        VmInstruction::Push(Segment::Constant, u16::MAX),
        VmInstruction::Pop(Segment::Local, 1), // quotient
        VmInstruction::Push(Segment::Argument, 1),
        VmInstruction::Push(Segment::Constant, 0),
        VmInstruction::Eq,
        VmInstruction::IfGoto("DIV_END".into()),
        VmInstruction::Push(Segment::Argument, 0),
        VmInstruction::Pop(Segment::Local, 0), // remainder
        VmInstruction::Push(Segment::Constant, 0),
        VmInstruction::Pop(Segment::Local, 1), // quotient
        VmInstruction::Label("DIV_WHILE".into()),
        VmInstruction::Push(Segment::Local, 0), // end if remainder < divisor
        VmInstruction::Push(Segment::Argument, 1),
        VmInstruction::Lt,
        VmInstruction::IfGoto("DIV_END".into()),
        VmInstruction::Push(Segment::Local, 0), // r - divisor
        VmInstruction::Push(Segment::Argument, 1),
        VmInstruction::Sub,
        VmInstruction::Pop(Segment::Local, 0),
        VmInstruction::Push(Segment::Local, 1), // increment quotient
        VmInstruction::Push(Segment::Constant, 1),
        VmInstruction::Add,
        VmInstruction::Pop(Segment::Local, 1),
        VmInstruction::Goto("DIV_WHILE".into()),
        VmInstruction::Label("DIV_END".into()),
        VmInstruction::Push(Segment::Local, 1),
        VmInstruction::Return(1),
    ]
}

/// The preable is added at the beginning of the program
pub fn preamble() -> Vec<VmInstruction> {
    let mut ret = sys();
    ret.extend(peek());
    ret.extend(poke());
    ret.extend(mul());
    ret.extend(div());
    ret
}
