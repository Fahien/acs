// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError, generator::Generator, segment::Segment, vm::instruction::VmInstruction,
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
