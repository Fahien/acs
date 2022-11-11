// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError,
    generator::{Generate, Generator},
    segment::Segment,
    vm::instruction::VmInstruction,
};

#[test]
fn hello_void() -> Result<(), CalError> {
    let module = "fn main() {}".parse()?;
    let vm_instructions = Generator::default().gen_module(&module);
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "main");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    match &vm_instructions[1] {
        VmInstruction::Return(return_size_in_words) => {
            assert_eq!(*return_size_in_words, 0);
        }
        _ => panic!(),
    }
    Ok(())
}

#[test]
fn return_zero() -> Result<(), CalError> {
    let module = "fn main() -> i16 { 0 }".parse()?;
    let vm_instructions = Generator::default().gen_module(&module);
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "main");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    assert!(matches!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 0)
    ));
    assert!(matches!(vm_instructions[2], VmInstruction::Return(1)));

    let module = "fn main() -> i16 { return 0; }".parse()?;
    let vm_instructions = Generator::default().gen_module(&module);
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "main");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    assert!(matches!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 0)
    ));
    assert!(matches!(vm_instructions[2], VmInstruction::Return(1)));

    Ok(())
}

#[test]
fn def_local() -> Result<(), CalError> {
    let vm_instructions = "fn def_local() { let x: i16 = 0; let y: i16 = 1; }".generate()?;
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "def_local");
            assert_eq!(*local_count, 2);
        }
        _ => panic!(),
    }
    assert!(matches!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 0)
    ));
    assert!(matches!(
        vm_instructions[2],
        VmInstruction::Pop(Segment::Local, 0)
    ));
    assert!(matches!(
        vm_instructions[3],
        VmInstruction::Push(Segment::Constant, 1)
    ));
    assert!(matches!(
        vm_instructions[4],
        VmInstruction::Pop(Segment::Local, 1)
    ));
    assert!(matches!(vm_instructions[5], VmInstruction::Return(0)));
    Ok(())
}

#[test]
fn call_function() -> Result<(), CalError> {
    let vm_instructions = "fn main() { call() }".generate()?;
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "main");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    match &vm_instructions[1] {
        VmInstruction::Call(name, param_count) => {
            assert_eq!(name, "call");
            assert_eq!(*param_count, 0);
        }
        _ => panic!(),
    }
    assert!(matches!(vm_instructions[2], VmInstruction::Return(0)));
    Ok(())
}
