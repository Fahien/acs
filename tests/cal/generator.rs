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
    let vm_instructions = Generator::default().gen_module(&module)?;
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
    let vm_instructions = "fn main() -> i16 { 0 }".generate()?;
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

    let vm_instructions = "fn main() -> i16 { return 0; }".generate()?;
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

#[test]
fn one_parameter() -> Result<(), CalError> {
    let vm_instructions = "fn identity(x: i16) -> i16 { x }".generate()?;
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "identity");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    assert!(matches!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Argument, 0)
    ));
    assert!(matches!(vm_instructions[2], VmInstruction::Return(1)));
    Ok(())
}

#[test]
fn multi_parameters() -> Result<(), CalError> {
    let vm_instructions = "fn ignore_x(x: i16, y: i16) -> i16 { y }".generate()?;
    match &vm_instructions[0] {
        VmInstruction::Function(name, local_count) => {
            assert_eq!(name, "ignore_x");
            assert_eq!(*local_count, 0);
        }
        _ => panic!(),
    }
    assert!(matches!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Argument, 1)
    ));
    assert!(matches!(vm_instructions[2], VmInstruction::Return(1)));
    Ok(())
}

#[test]
fn add() -> Result<(), CalError> {
    let vm_instructions = "fn main() { 1 + 2; }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[3], VmInstruction::Add);
    Ok(())
}

#[test]
fn if_statement() -> Result<(), CalError> {
    let vm_instructions = "fn main() -> bool { if true { true } else { false } }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 0),
    );
    assert_eq!(vm_instructions[2], VmInstruction::Not);
    assert_eq!(vm_instructions[3], VmInstruction::Not);
    assert_eq!(
        vm_instructions[4],
        VmInstruction::IfGoto(String::from("VM_LABEL0"))
    );
    assert_eq!(
        vm_instructions[5],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[6], VmInstruction::Not);
    assert_eq!(
        vm_instructions[7],
        VmInstruction::Goto(String::from("VM_LABEL1"))
    );
    assert_eq!(
        vm_instructions[8],
        VmInstruction::Label(String::from("VM_LABEL0"))
    );
    assert_eq!(
        vm_instructions[9],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(
        vm_instructions[10],
        VmInstruction::Label(String::from("VM_LABEL1"))
    );
    Ok(())
}

#[test]
fn while_statement() -> Result<(), CalError> {
    let vm_instructions = "fn main() -> bool { while true { return true; } false }".generate()?;
    let mut index = 0;
    let VmInstruction::Function(name, 0) = &vm_instructions[index] else {
        panic!();
    };
    assert_eq!(name, "main");
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Label(String::from("VM_LABEL0"))
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 0),
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Not);
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Not);
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::IfGoto(String::from("VM_LABEL1"))
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 0)
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Not);
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Return(1));
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Goto(String::from("VM_LABEL0"))
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Label(String::from("VM_LABEL1"))
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 0)
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Return(1));
    Ok(())
}

#[test]
fn cmp() -> Result<(), CalError> {
    let vm_instructions = r#"
        fn main() -> bool {
            1 == 1;
            1 != 2;
            1 < 2;
            2 > 1
        }"#
    .generate()?;
    let mut index = 0;
    let VmInstruction::Function(name, 0) = &vm_instructions[index] else {
        panic!();
    };
    assert_eq!(name, "main");

    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1),
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Eq);

    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 2),
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Eq);
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Not);

    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 2),
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Lt);

    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 2)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1),
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Gt);

    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Return(1));
    Ok(())
}

#[test]
fn assign_expression() -> Result<(), CalError> {
    let vm_instructions = r#"
    fn main() {
        let a: i16 = 0;
        a = 1;
    }"#
    .generate()?;

    let mut index = 0;
    let VmInstruction::Function(name, 1) = &vm_instructions[index] else {
        panic!();
    };
    assert_eq!(name, "main");

    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 0)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Pop(Segment::Local, 0)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Push(Segment::Constant, 1)
    );
    index += 1;
    assert_eq!(
        vm_instructions[index],
        VmInstruction::Pop(Segment::Local, 0)
    );
    index += 1;
    assert_eq!(vm_instructions[index], VmInstruction::Return(0));

    Ok(())
}

#[test]
fn mul() -> Result<(), CalError> {
    let vm_instructions = "fn main() { 1 * 2; }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(
        vm_instructions[3],
        VmInstruction::Call(String::from("mul"), 2)
    );
    Ok(())
}

#[test]
fn and() -> Result<(), CalError> {
    let vm_instructions = "fn main() { 1 & 2; }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[3], VmInstruction::And);
    Ok(())
}

#[test]
fn or() -> Result<(), CalError> {
    let vm_instructions = "fn main() { 1 | 2; }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[3], VmInstruction::Or);
    Ok(())
}

#[test]
fn modulo() -> Result<(), CalError> {
    let vm_instructions = "fn main() { 2 % 1; }".generate()?;
    let VmInstruction::Function(name, 0) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[3],
        VmInstruction::Call(String::from("mod"), 2)
    );
    Ok(())
}

#[test]
fn array() -> Result<(), CalError> {
    let vm_instructions = "fn main() -> [i16; 2] { let a: [i16; 2] = [1, 2]; a }".generate()?;
    let VmInstruction::Function(name, 2) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[3], VmInstruction::Pop(Segment::Local, 1));
    assert_eq!(vm_instructions[4], VmInstruction::Pop(Segment::Local, 0));
    assert_eq!(vm_instructions[5], VmInstruction::Push(Segment::Local, 0));
    assert_eq!(vm_instructions[6], VmInstruction::Push(Segment::Local, 1));
    assert_eq!(vm_instructions[7], VmInstruction::Return(2));
    Ok(())
}

#[test]
fn character() -> Result<(), CalError> {
    let vm_instructions = "fn main() -> char { let a: char = 'a'; a }".generate()?;
    let VmInstruction::Function(name, 1) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 'a' as u16)
    );
    assert_eq!(vm_instructions[2], VmInstruction::Pop(Segment::Local, 0));
    assert_eq!(vm_instructions[3], VmInstruction::Push(Segment::Local, 0));
    assert_eq!(vm_instructions[4], VmInstruction::Return(1));
    Ok(())
}

#[test]
fn reference() -> Result<(), CalError> {
    let vm_instructions = r#"
    fn main() -> i16 {
        let a: i16 = 1;
        pass(&a);
        a
    }
    fn pass(a: &i16) {
        a = 2;
    }
    "#
    .generate()?;
    assert_eq!(
        vm_instructions[0],
        VmInstruction::Function(String::from("main"), 1)
    );
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(vm_instructions[2], VmInstruction::Pop(Segment::Local, 0));
    assert_eq!(
        vm_instructions[3],
        VmInstruction::Push(Segment::Constant, Segment::Local.get_base_address() as u16)
    );
    assert_eq!(vm_instructions[4], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[5], VmInstruction::Push(Segment::This, 0));
    assert_eq!(
        vm_instructions[6],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[7], VmInstruction::Add);
    assert_eq!(
        vm_instructions[8],
        VmInstruction::Call(String::from("pass"), 1)
    );
    assert_eq!(vm_instructions[9], VmInstruction::Push(Segment::Local, 0));
    assert_eq!(vm_instructions[10], VmInstruction::Return(1));

    assert_eq!(
        vm_instructions[11],
        VmInstruction::Function(String::from("pass"), 0)
    );
    assert_eq!(
        vm_instructions[12],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(
        vm_instructions[13],
        VmInstruction::Push(Segment::Argument, 0)
    );
    assert_eq!(vm_instructions[14], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[15], VmInstruction::Pop(Segment::This, 0));
    assert_eq!(vm_instructions[16], VmInstruction::Return(0));

    let vm_instructions = r#"
    fn main() -> i16 {
        let a: [i16; 2] = [1, 2];
        pass(&a);
        a[1]
    }
    fn pass(a: &[i16; 2]) {
        a[1] = 3;
    }
    "#
    .generate()?;
    assert_eq!(
        vm_instructions[0],
        VmInstruction::Function(String::from("main"), 2)
    );
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[3], VmInstruction::Pop(Segment::Local, 1));
    assert_eq!(vm_instructions[4], VmInstruction::Pop(Segment::Local, 0));
    assert_eq!(
        vm_instructions[5],
        VmInstruction::Push(Segment::Constant, Segment::Local.get_base_address() as u16)
    );
    assert_eq!(vm_instructions[6], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[7], VmInstruction::Push(Segment::This, 0));
    assert_eq!(
        vm_instructions[8],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[9], VmInstruction::Add);
    assert_eq!(
        vm_instructions[10],
        VmInstruction::Call(String::from("pass"), 1)
    );
    assert_eq!(
        vm_instructions[11],
        VmInstruction::Push(Segment::Constant, 1)
    ); // index
    assert_eq!(
        vm_instructions[12],
        VmInstruction::Push(Segment::Constant, 0)
    ); // offset
    assert_eq!(vm_instructions[13], VmInstruction::Add);
    assert_eq!(
        vm_instructions[14],
        VmInstruction::Push(Segment::Constant, Segment::Local.get_base_address() as u16)
    );
    assert_eq!(vm_instructions[15], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[16], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[17], VmInstruction::Add);
    assert_eq!(vm_instructions[18], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[19], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[20], VmInstruction::Return(1));

    assert_eq!(
        vm_instructions[21],
        VmInstruction::Function(String::from("pass"), 0)
    );
    assert_eq!(
        vm_instructions[22],
        VmInstruction::Push(Segment::Constant, 3) // rhs
    );
    assert_eq!(
        vm_instructions[23],
        VmInstruction::Push(Segment::Constant, 1) // index
    );
    assert_eq!(
        vm_instructions[24],
        VmInstruction::Push(Segment::Argument, 0)
    );
    assert_eq!(vm_instructions[25], VmInstruction::Add);
    assert_eq!(vm_instructions[26], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[27], VmInstruction::Pop(Segment::This, 0));
    assert_eq!(vm_instructions[28], VmInstruction::Return(0));
    Ok(())
}

#[test]
fn array_of_array() -> Result<(), CalError> {
    let vm_instructions = r#"
    fn main() -> [i16; 2] {
        let a: [[i16; 2]; 2] = [[1, 2], [3, 4]];
        a[1]
    }"#
    .generate()?;
    let VmInstruction::Function(name, 4) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(
        vm_instructions[3],
        VmInstruction::Push(Segment::Constant, 3)
    );
    assert_eq!(
        vm_instructions[4],
        VmInstruction::Push(Segment::Constant, 4)
    );
    assert_eq!(vm_instructions[5], VmInstruction::Pop(Segment::Local, 3));
    assert_eq!(vm_instructions[6], VmInstruction::Pop(Segment::Local, 2));
    assert_eq!(vm_instructions[7], VmInstruction::Pop(Segment::Local, 1));
    assert_eq!(vm_instructions[8], VmInstruction::Pop(Segment::Local, 0));
    // Index
    assert_eq!(
        vm_instructions[9],
        VmInstruction::Push(Segment::Constant, 1)
    );
    // Size of element
    assert_eq!(
        vm_instructions[10],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[11], VmInstruction::Call("mul".into(), 2));
    // Variable offset
    assert_eq!(
        vm_instructions[12],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[13], VmInstruction::Add);
    // Local segment base address
    assert_eq!(
        vm_instructions[14],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(vm_instructions[15], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[16], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[17], VmInstruction::Add); // *segment + offset + index expression
    assert_eq!(vm_instructions[18], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[19], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[20], VmInstruction::Push(Segment::This, 1));
    assert_eq!(vm_instructions[21], VmInstruction::Return(2));
    Ok(())
}

#[test]
fn array_of_array_reference() -> Result<(), CalError> {
    let vm_instructions = r#"
    fn main() -> [i16; 2] {
        let a: [[i16; 2]; 2] = [[1, 2], [3, 4]];
        pass(&a[1]);
        a[1]
    }
    fn pass(a: &[i16; 2]) {
        a[1] = 5;
    }"#
    .generate()?;
    let VmInstruction::Function(name, 4) = &vm_instructions[0] else {
        panic!();
    };
    assert_eq!(name, "main");
    assert_eq!(
        vm_instructions[1],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(
        vm_instructions[2],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(
        vm_instructions[3],
        VmInstruction::Push(Segment::Constant, 3)
    );
    assert_eq!(
        vm_instructions[4],
        VmInstruction::Push(Segment::Constant, 4)
    );
    assert_eq!(vm_instructions[5], VmInstruction::Pop(Segment::Local, 3));
    assert_eq!(vm_instructions[6], VmInstruction::Pop(Segment::Local, 2));
    assert_eq!(vm_instructions[7], VmInstruction::Pop(Segment::Local, 1));
    assert_eq!(vm_instructions[8], VmInstruction::Pop(Segment::Local, 0));

    // &a[1]
    // Index
    assert_eq!(
        vm_instructions[9],
        VmInstruction::Push(Segment::Constant, 1)
    );
    // Size of element
    assert_eq!(
        vm_instructions[10],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[11], VmInstruction::Call("mul".into(), 2));

    // Variable offset
    assert_eq!(
        vm_instructions[12],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[13], VmInstruction::Add);
    // Local segment base address
    assert_eq!(
        vm_instructions[14],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(vm_instructions[15], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[16], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[17], VmInstruction::Add); // *segment + offset + index expression
    assert_eq!(vm_instructions[18], VmInstruction::Call("pass".into(), 1));

    // a[1]
    // Index
    assert_eq!(
        vm_instructions[19],
        VmInstruction::Push(Segment::Constant, 1)
    );
    // Size of element
    assert_eq!(
        vm_instructions[20],
        VmInstruction::Push(Segment::Constant, 2)
    );
    assert_eq!(vm_instructions[21], VmInstruction::Call("mul".into(), 2));

    // Variable offset
    assert_eq!(
        vm_instructions[22],
        VmInstruction::Push(Segment::Constant, 0)
    );
    assert_eq!(vm_instructions[23], VmInstruction::Add);
    // Local segment base address
    assert_eq!(
        vm_instructions[24],
        VmInstruction::Push(Segment::Constant, 1)
    );
    assert_eq!(vm_instructions[25], VmInstruction::Pop(Segment::Pointer, 0));
    assert_eq!(vm_instructions[26], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[27], VmInstruction::Add); // *segment + offset + index expressio
    assert_eq!(vm_instructions[29], VmInstruction::Push(Segment::This, 0));
    assert_eq!(vm_instructions[30], VmInstruction::Push(Segment::This, 1));
    assert_eq!(vm_instructions[31], VmInstruction::Return(2));
    Ok(())
}
