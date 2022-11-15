// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{compiler::Compile, error::CalError, Computer};

#[test]
fn hello_void() -> Result<(), CalError> {
    let asm_instructions = "fn main() {}".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 256);
    Ok(())
}

#[test]
fn return_zero() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 1 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 1);

    let asm_instructions = "fn main() -> i16 { return 2; }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 2);
    Ok(())
}

#[test]
fn def_local() -> Result<(), CalError> {
    let asm_instructions = "fn main() { let x: i16 = 1; let y: i16 = 2; }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..256 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 256);
    // 5 elements were pushed on the stack when calling main for saving previous stack frame
    assert_eq!(computer.get_memory().ram[261], 1);
    assert_eq!(computer.get_memory().ram[262], 2);
    Ok(())
}

#[test]
fn call_function() -> Result<(), CalError> {
    let asm_instructions = "fn main() { call() } fn call() {}".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..256 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 256);
    Ok(())
}

#[test]
fn one_parameter() -> Result<(), CalError> {
    let asm_instructions =
        "fn identity(x: i16) -> i16 { x } fn main() -> i16 { identity(42) }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 42);
    Ok(())
}

#[test]
fn multi_parameters() -> Result<(), CalError> {
    let asm_instructions =
        "fn ignore_x(x: i16, y: i16) -> i16 { y } fn main() -> i16 { ignore_x(4, 5) }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 5);
    Ok(())
}

#[test]
fn add() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 1 + 2 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 3);
    Ok(())
}

#[test]
fn sub() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 1 - 2 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);
    Ok(())
}
