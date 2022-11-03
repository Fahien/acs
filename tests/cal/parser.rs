// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::structure::{Module, Type};

#[test]
fn hello_void() -> Result<(), String> {
    let module: Module = "fn main() {}".parse()?;
    assert_eq!(module.functions.len(), 1);
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert!(function.parameters.is_empty());
    assert!(function.body_statements.is_empty());
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Void);
    Ok(())
}

#[test]
fn return_zero() -> Result<(), String> {
    let module: Module = "fn main() -> i16 { 0 }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert!(function.parameters.is_empty());
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::I16);

    let module: Module = "fn main() -> i16 { return 0; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert!(function.parameters.is_empty());
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::I16);
    Ok(())
}
