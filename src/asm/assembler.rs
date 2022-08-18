// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::asm::instruction::AsmInstruction;

/// The assembler translates programs written in asm language (text) to binary code
pub struct Assembler {
    symbol_table: HashMap<String, u16>,
    next_variable_address: u16,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            symbol_table: Self::default_symbol_table(),
            next_variable_address: 16,
        }
    }
}

impl Assembler {
    /// Returns a symbol table with predefined symbols
    fn default_symbol_table() -> HashMap<String, u16> {
        HashMap::<String, u16>::from([
            ("R0".into(), 0),
            ("R1".into(), 1),
            ("R2".into(), 2),
            ("R3".into(), 3),
            ("R4".into(), 4),
            ("R5".into(), 5),
            ("R6".into(), 6),
            ("R7".into(), 7),
            ("R8".into(), 8),
            ("R9".into(), 9),
            ("R10".into(), 10),
            ("R11".into(), 11),
            ("R12".into(), 12),
            ("R13".into(), 13),
            ("R14".into(), 14),
            ("R15".into(), 15),
            ("SP".into(), 0),
            ("LCL".into(), 1),
            ("ARG".into(), 2),
            ("THIS".into(), 3),
            ("THAT".into(), 4),
            ("SCREEN".into(), 16384),
            ("KBD".into(), 24576),
        ])
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Removes comments and trims lines
    fn preprocess(asm: &str) -> Vec<&str> {
        let mut lines = vec![];
        for line in asm.lines() {
            // Ignore comments at the end of the line
            let line = if let Some(index) = line.find("//") {
                &line[0..index]
            } else {
                line
            };
            // Remove space characters
            let line = line.trim_matches(|c| c == ' ' || c == '\n' || c == '\r');

            if !line.is_empty() {
                lines.push(line);
            }
        }
        lines
    }

    pub fn translate(lines: Vec<&str>) -> Vec<AsmInstruction> {
        lines
            .into_iter()
            .map(|line| line.parse().unwrap())
            .collect()
    }

    /// Resolves symbols in the assembly to physical memory addresses.
    pub fn resolve(&mut self, mut asms: Vec<AsmInstruction>) -> Vec<AsmInstruction> {
        let mut no_label_instructions = vec![];

        // First pass, collects labels and empty out those lines
        let mut skipped_lines = 0;
        for (n, asm) in asms.iter_mut().enumerate() {
            if let AsmInstruction::Label(symbol) = asm {
                self.symbol_table
                    .insert(symbol.clone(), (n - skipped_lines) as u16);
                skipped_lines += 1;
            } else {
                no_label_instructions.push(asm.clone());
            }
        }

        let mut instructions = vec![];

        // Second pass
        for asm in &no_label_instructions {
            match asm {
                AsmInstruction::Symbol(symbol) => {
                    // Check whether we already encountered this symbol
                    if let Some(&literal) = self.symbol_table.get(symbol) {
                        instructions.push(AsmInstruction::A(literal));
                    } else {
                        // New variable symbol
                        self.symbol_table
                            .insert(symbol.clone(), self.next_variable_address);
                        instructions.push(AsmInstruction::A(self.next_variable_address));
                        self.next_variable_address += 1;
                    }
                }
                _ => instructions.push(asm.clone()),
            }
        }

        instructions
    }

    pub fn assemble(&mut self, asm: impl AsRef<str>) -> Vec<AsmInstruction> {
        let lines = Self::preprocess(asm.as_ref());
        let asms = Self::translate(lines);
        self.resolve(asms)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        const CODE: &str = "@2\nD=A\n@3\nD=D+A\n@0\nM=D\n";
        let mut assembler = Assembler::default();
        let instructions = assembler.assemble(CODE);
        assert!(!instructions.is_empty());
        assert_eq!(u16::from(&instructions[0]), 2);
        assert_eq!(u16::from(&instructions[1]), 0b1110110000010000);
        assert_eq!(u16::from(&instructions[2]), 3);
        assert_eq!(u16::from(&instructions[3]), 0b1110000010010000);
        assert_eq!(u16::from(&instructions[4]), 0);
        assert_eq!(u16::from(&instructions[5]), 0b1110001100001000);
    }

    #[test]
    fn max() {
        const CODE: &str = r#"
           // Computes R2 = max(R0, R1)  (R0,R1,R2 refer to RAM[0],RAM[1],RAM[2])
           @R0
           D=M              // D = first number
           @R1
           D=D-M            // D = first number - second number
           @OUTPUT_FIRST
           D;JGT            // if D>0 (first is greater) goto output_first
           @R1
           D=M              // D = second number
           @OUTPUT_D
           0;JMP            // goto output_d
        (OUTPUT_FIRST)
           @R0             
           D=M              // D = first number
        (OUTPUT_D)
           @R2
           M=D              // M[2] = D (greatest number)
        (INFINITE_LOOP)
           @INFINITE_LOOP
           0;JMP            // infinite loop
        "#;
        let mut assembler = Assembler::default();
        let instructions = assembler.assemble(CODE);
        assert!(!instructions.is_empty());
        assert_eq!(instructions[0], 0.into());
    }

    #[test]
    fn rect() {
        const CODE: &str = r#"
           @0
           D=M
           @INFINITE_LOOP
           D;JLE
           @counter
           M=D
           @SCREEN
           D=A
           @address
           M=D
        (LOOP)
           @address
           A=M
           M=-1
           @address
           D=M
           @32
           D=D+A
           @address
           M=D
           @counter
           MD=M-1
           @LOOP
           D;JGT
        (INFINITE_LOOP)
           @INFINITE_LOOP
           0;JMP
        "#;
        let mut assembler = Assembler::default();
        let instructions = assembler.assemble(CODE);
        assert!(!instructions.is_empty());
        assert_eq!(instructions[0], 0.into());
    }
}
