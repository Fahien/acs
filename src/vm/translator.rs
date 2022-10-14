// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::{
    asm::instruction::AsmInstruction,
    asm::instruction::{Comp, Dest, Jump},
    segment::Segment,
    vm::instruction::VmInstruction,
    Assembler,
};

/// Translates VM instructions into a sequence of low-level instructions that
/// can execute on the target platform.
/// It represents the VM stack abstraction using the `RAM`. The stack pointer, or
/// `SP`, is stored at `RAM[0]` and defaults to `256`.
#[derive(Default)]
pub struct VmTranslator {
    /// Labels generated so far
    label_count: u32,
}

use AsmInstruction as I;

impl VmTranslator {
    fn next_label(&mut self) -> String {
        let ret = format!("LABEL{}", self.label_count);
        self.label_count += 1;
        ret
    }

    /// Translates a push VM instruction into an equivalent sequence of assembly instructions
    fn gen_push(segment: Segment, value: u16) -> Vec<I> {
        match segment {
            Segment::Stack => {
                vec![
                    // Put value in A, then in D
                    // @value and D=A
                    I::A(value),
                    I::C(Dest::D, Comp::A, Jump::No),
                    // Put stack pointer from RAM[0] in A
                    // Here A is used for addressing the next instruction
                    // @SP -> A=0 and A=M[A]
                    I::A(0),
                    I::C(Dest::A, Comp::M, Jump::No),
                    // Copy value from D to stack memory
                    // *SP=D -> M[A]=D
                    I::C(Dest::M, Comp::D, Jump::No),
                    // Now increment value in RAM[0]
                    // A=0
                    I::A(0),
                    // M[0]=M[0]+1
                    I::C(Dest::M, Comp::MPlusOne, Jump::No),
                ]
            }
            Segment::Local | Segment::Argument | Segment::This | Segment::That => vec![
                I::A(segment.get_base_address() as u16),
                I::C(Dest::D, Comp::M, Jump::No),
                I::A(value),
                I::C(Dest::A, Comp::DPlusA, Jump::No),
                I::C(Dest::D, Comp::M, Jump::No),
                I::A(0),
                I::C(Dest::AM, Comp::MPlusOne, Jump::No),
                I::C(Dest::A, Comp::AMinusOne, Jump::No),
                I::C(Dest::M, Comp::D, Jump::No),
            ],
            Segment::Constant => vec![
                // Put constant value in D
                I::A(value),
                I::C(Dest::D, Comp::A, Jump::No),
                // Get stack pointer
                I::A(0),
                I::C(Dest::AM, Comp::MPlusOne, Jump::No),
                I::C(Dest::A, Comp::AMinusOne, Jump::No),
                // Put D on top of the stack
                I::C(Dest::M, Comp::D, Jump::No),
            ],
            Segment::Static | Segment::Pointer | Segment::Temp => {
                vec![
                    I::A(segment.get_base_address() as u16 + value),
                    I::C(Dest::D, Comp::M, Jump::No),
                    I::A(0),
                    I::C(Dest::AM, Comp::MPlusOne, Jump::No),
                    I::C(Dest::A, Comp::AMinusOne, Jump::No),
                    I::C(Dest::M, Comp::D, Jump::No),
                ]
            }
            _ => unimplemented!(),
        }
    }

    /// Translates a pop VM instruction into an equivalent sequence of assembly instructions
    fn gen_pop(segment: Segment, value: u16) -> Vec<I> {
        match segment {
            Segment::Stack => vec![I::A(0), I::C(Dest::M, Comp::MMinusOne, Jump::No)],
            Segment::Local | Segment::Argument | Segment::This | Segment::That => vec![
                // Put in D address where to store
                I::A(value),
                I::C(Dest::D, Comp::A, Jump::No),
                I::A(segment.get_base_address() as u16),
                I::C(Dest::D, Comp::DPlusM, Jump::No),
                // Store address where to store on stack
                I::A(0),
                I::C(Dest::AM, Comp::MMinusOne, Jump::No),
                I::C(Dest::A, Comp::APlusOne, Jump::No),
                I::C(Dest::M, Comp::D, Jump::No),
                // Put element from stack in D
                I::C(Dest::A, Comp::AMinusOne, Jump::No),
                I::C(Dest::D, Comp::M, Jump::No),
                // Put address to store in A
                I::C(Dest::A, Comp::APlusOne, Jump::No),
                I::C(Dest::A, Comp::M, Jump::No),
                // Store element in segment
                I::C(Dest::M, Comp::D, Jump::No),
            ],
            Segment::Static | Segment::Temp | Segment::Pointer => vec![
                I::A(0),
                I::C(Dest::AM, Comp::MMinusOne, Jump::No),
                I::C(Dest::D, Comp::M, Jump::No),
                I::A(segment.get_base_address() as u16 + value),
                I::C(Dest::M, Comp::D, Jump::No),
            ],
            _ => unimplemented!(),
        }
    }

    fn gen_and() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // AM=M[A]-1
            I::C(Dest::AM, Comp::MMinusOne, Jump::No),
            // Pop first element from stack
            // D=M[A]
            I::C(Dest::D, Comp::M, Jump::No),
            // Move temp stack pointer backwards
            // A=A-1
            I::C(Dest::A, Comp::AMinusOne, Jump::No),
            // M[A]=D & M[A]
            I::C(Dest::M, Comp::DAndM, Jump::No),
        ]
    }

    fn gen_or() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // AM=M[A]-1
            I::C(Dest::AM, Comp::MMinusOne, Jump::No),
            // Pop first element from stack
            // D=M[A]
            I::C(Dest::D, Comp::M, Jump::No),
            // Move temp stack pointer backwards
            // A=A-1
            I::C(Dest::A, Comp::AMinusOne, Jump::No),
            // M[A]=D & M[A]
            I::C(Dest::M, Comp::DOrM, Jump::No),
        ]
    }

    pub fn gen_add() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // AM=M[A]-1
            I::C(Dest::AM, Comp::MMinusOne, Jump::No),
            // Pop first element from stack
            // D=M[A]
            I::C(Dest::D, Comp::M, Jump::No),
            // Move temp stack pointer backwards
            // A=A-1
            I::C(Dest::A, Comp::AMinusOne, Jump::No),
            // M[A]=D+M[A]
            I::C(Dest::M, Comp::DPlusM, Jump::No),
        ]
    }

    pub fn gen_sub() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // AM=M[A]-1
            I::C(Dest::AM, Comp::MMinusOne, Jump::No),
            // Pop first element from stack
            // D=M[A]
            I::C(Dest::D, Comp::M, Jump::No),
            // Move temp stack pointer backwards
            // A=A-1
            I::C(Dest::A, Comp::AMinusOne, Jump::No),
            // M[A]=D+M[A]
            I::C(Dest::M, Comp::MMinusD, Jump::No),
        ]
    }

    pub fn gen_neg() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // A=M[A]
            I::C(Dest::A, Comp::MMinusOne, Jump::No),
            // M[A]=-M[A]
            I::C(Dest::M, Comp::MinusM, Jump::No),
        ]
    }

    pub fn gen_not() -> Vec<I> {
        vec![
            // Get stack pointer
            // A=0
            I::A(0),
            // A=M[A]
            I::C(Dest::A, Comp::MMinusOne, Jump::No),
            // M[A]=-M[A]
            I::C(Dest::M, Comp::NotM, Jump::No),
        ]
    }

    /// Pops two elements from the stack and pushes whether they _compare_ (encoded in jump)
    fn gen_compare(&mut self, jump: Jump) -> Vec<I> {
        let next_label = self.next_label();
        vec![
            // Get stack pointer and decrements it (pop)
            I::A(0),
            // A,M[0]=M[0]-1
            I::C(Dest::AM, Comp::MMinusOne, Jump::No),
            // Get topmost element of stack
            // D=M[A]
            I::C(Dest::D, Comp::M, Jump::No),
            // Move stack pointer backwards
            // A=A-1
            I::C(Dest::A, Comp::AMinusOne, Jump::No),
            // D = top() - previously_popped()
            // D=M[A]-D
            I::C(Dest::D, Comp::MMinusD, Jump::No),
            // Set element on stack (result) to TRUE
            I::C(Dest::M, Comp::MinusOne, Jump::No),
            // Put label address into A
            I::Symbol(next_label.clone()),
            // Jump if D `jump-comparison` 0
            I::C(Dest::D, Comp::D, jump),
            // Symbol instruction wrote onto A, need to get SP again
            I::A(0),
            I::C(Dest::A, Comp::MMinusOne, Jump::No),
            // Set element on stack (result) to FALSE
            I::C(Dest::M, Comp::Zero, Jump::No),
            I::Label(next_label),
        ]
    }

    /// Translates a VM instruction into a sequence of assembly instructions
    fn vm_to_asm(&mut self, vm_instruction: VmInstruction) -> Vec<I> {
        match vm_instruction {
            VmInstruction::Push(segment, value) => Self::gen_push(segment, value),
            VmInstruction::Pop(segment, value) => Self::gen_pop(segment, value),
            VmInstruction::Add => Self::gen_add(),
            VmInstruction::Sub => Self::gen_sub(),
            VmInstruction::Eq => self.gen_compare(Jump::Eq),
            VmInstruction::Lt => self.gen_compare(Jump::Lt),
            VmInstruction::Gt => self.gen_compare(Jump::Gt),
            VmInstruction::Neg => Self::gen_neg(),
            VmInstruction::And => Self::gen_and(),
            VmInstruction::Or => Self::gen_or(),
            VmInstruction::Not => Self::gen_not(),
            VmInstruction::Label(label) => vec![I::Label(label)],
            VmInstruction::IfGoto(_label) => todo!(),
            VmInstruction::Goto(_label) => todo!(),
            VmInstruction::Function(_function, _param_count) => todo!(),
            VmInstruction::Return => todo!(),
            VmInstruction::Call(_function, _arg_count) => todo!(),
        }
    }

    /// Translates a VM program into a sequence of assembly instructions
    pub fn translate(&mut self, vm_instructions: Vec<VmInstruction>) -> Vec<I> {
        let mut asm_instructions = vec![
            // Set stack pointer to 256
            I::A(256),
            I::C(Dest::D, Comp::A, Jump::No), // D=256
            I::A(0),
            I::C(Dest::M, Comp::D, Jump::No), // M[0]=D
        ];
        for instruction in vm_instructions {
            let new_asm_instructions = self.vm_to_asm(instruction);
            asm_instructions.extend(new_asm_instructions)
        }

        let mut assembler = Assembler::default();
        assembler.resolve(asm_instructions)
    }
}
