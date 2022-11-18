// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
};

use acs::{compiler::compile, error::CalError, Assembler};

fn to_bytes(uint: &u16) -> &[u8] {
    unsafe { std::slice::from_raw_parts(uint as *const u16 as *const u8, 2) }
}

fn main() -> Result<(), CalError> {
    let args: Vec<String> = env::args().collect();
    let cal_path = args.get(1).expect("Expected one cli argument: cal_path");

    let code = read_to_string(cal_path).expect("Failed to read string from asm");
    let asm_instructions = compile(&code)?;

    let mut assembler = Assembler::new();
    let asm_instructions = assembler.resolve(asm_instructions);

    let mut out = File::create("out.asm").unwrap();
    for asmi in asm_instructions {
        out.write_all(to_bytes(&u16::from(&asmi))).unwrap();
    }
    Ok(())
}
