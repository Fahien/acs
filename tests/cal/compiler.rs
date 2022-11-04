// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{compiler::Compile, Computer};

#[test]
fn hello_void() -> Result<(), String> {
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
fn return_zero() -> Result<(), String> {
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
