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
fn return_integer() -> Result<(), CalError> {
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

    let asm_instructions = "fn main() -> i16 { 0b11 }".compile()?;
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

#[test]
fn if_statement() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> bool { if true { true } else { false } }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = "fn main() -> bool { if true { return true; } false }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);
    Ok(())
}

#[test]
fn while_statement() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> bool { while true { return true; } false }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = "fn main() -> bool { while false { return false; } true }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);
    Ok(())
}

#[test]
fn cmp() -> Result<(), CalError> {
    let asm_instructions = r#"fn main() -> bool { 1 == 1 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = r#"fn main() -> bool { 1 == 2 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 0);

    let asm_instructions = r#"fn main() -> bool { 1 != 2 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = r#"fn main() -> bool { 1 != 1 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 0);

    let asm_instructions = r#"fn main() -> bool { 1 < 2 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = r#"fn main() -> bool { 1 < 1 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 0);

    let asm_instructions = r#"fn main() -> bool { 2 > 1 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], -1);

    let asm_instructions = r#"fn main() -> bool { 1 > 1 }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..128 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 0);
    Ok(())
}

#[test]
fn assign_expression() -> Result<(), CalError> {
    let asm_instructions = r#"fn main() -> i16 { let a: i16 = 0; a = 1; a }"#.compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..256 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 1);
    Ok(())
}

#[test]
fn mul() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 3 * 2 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 6);
    Ok(())
}

#[test]
fn div() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 7 / 2 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 3);
    Ok(())
}

#[test]
fn and() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 6 & 3 }".compile()?;
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
fn or() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 1 | 2 }".compile()?;
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
fn modulo() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> i16 { 5 % 2 }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 1);
    Ok(())
}

#[test]
fn array() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> [i16; 2] { [1, 2] }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 258);
    assert_eq!(computer.get_memory().ram[256], 1);
    assert_eq!(computer.get_memory().ram[257], 2);

    let asm_instructions = "fn main() -> i16 { let a: [i16; 2] = [1, 2]; a[1] }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..4096 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 2);

    let asm_instructions =
        "fn main() -> i16 { let a: [i16; 2] = [1, 2]; a[1] = 3; a[1] }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 3);
    Ok(())
}

#[test]
fn character() -> Result<(), CalError> {
    let asm_instructions = "fn main() -> char { 'a' }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 97);

    Ok(())
}

#[test]
fn reference() -> Result<(), CalError> {
    let asm_instructions = r#"
    fn main() -> i16 {
        let a: i16 = 1;
        pass(&a);
        a
    }
    fn pass(a: &i16) {
        a = 2;
    }
    "#
    .compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 2);

    let asm_instructions = r#"
    fn main() -> i16 {
        let a: [i16; 2] = [1, 2];
        pass(&a);
        a[1]
    }
    fn pass(a: &[i16; 2]) {
        a[1] = 3;
    }
    "#
    .compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..2048 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 257);
    assert_eq!(computer.get_memory().ram[256], 3);

    Ok(())
}

#[test]
fn array_of_array() -> Result<(), CalError> {
    let asm_instructions =
        "fn main() -> [[i16; 2]; 2] { let a: [[i16; 2]; 2] = [[1, 2], [3,4]]; a }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 260);
    assert_eq!(computer.get_memory().ram[256], 1);
    assert_eq!(computer.get_memory().ram[257], 2);
    assert_eq!(computer.get_memory().ram[258], 3);
    assert_eq!(computer.get_memory().ram[259], 4);

    let asm_instructions =
        "fn main() -> [i16; 2] { let a: [[i16; 2]; 2] = [[1, 2], [3, 4]]; a[1] }".compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..1024 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 258);
    assert_eq!(computer.get_memory().ram[256], 3);
    assert_eq!(computer.get_memory().ram[257], 4);
    Ok(())
}

#[test]
fn array_of_array_reference() -> Result<(), CalError> {
    let asm_instructions = r#"fn edit(e: &[i16; 2]) {
            e[1] = 5;
        }

        fn main() -> [i16; 2] {
            let a: [[i16; 2]; 2] = [[1, 2], [3, 4]];
            edit(&a[1]);
            a[1]
        }"#
    .compile()?;
    let mut computer = Computer::default();
    computer.set_instructions(asm_instructions);
    for _ in 0..2048 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 258);
    assert_eq!(computer.get_memory().ram[256], 3);
    assert_eq!(computer.get_memory().ram[257], 5);
    Ok(())
}
