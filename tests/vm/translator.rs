use std::error::Error;

use acs::{asm::instruction::AsmInstruction, segment::Segment, Computer, Signal16};

const CYCLE_COUNT: u16 = 128;

#[test]
fn push_constant() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 42");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_local() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Local.get_base_address()] = 320.into();
    computer.get_memory_mut().ram.data[320] = 42.into();
    let instructions = AsmInstruction::parse("push local 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_this() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Pointer.get_base_address()] = 320.into();
    computer.get_memory_mut().ram.data[320] = 42.into();
    let instructions = AsmInstruction::parse("push this 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_that() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Pointer.get_base_address() + 1] = 320.into();
    computer.get_memory_mut().ram.data[320] = 42.into();
    let instructions = AsmInstruction::parse("push that 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_argument() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Argument.get_base_address()] = 320.into();
    computer.get_memory_mut().ram.data[320] = 42.into();
    let instructions = AsmInstruction::parse("push argument 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_temp() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Temp.get_base_address()] = 42.into();
    let instructions = AsmInstruction::parse("push temp 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn push_static() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Static.get_base_address()] = 42.into();
    let instructions = AsmInstruction::parse("push static 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn pop_local() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Local.get_base_address()] = 320.into();
    let instructions = AsmInstruction::parse("push constant 42\npop local 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(computer.get_memory().ram.data[320], 42);
    Ok(())
}

#[test]
fn pop_argument() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Argument.get_base_address()] = 320.into();
    let instructions = AsmInstruction::parse("push constant 42\npop argument 1");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(computer.get_memory().ram.data[321], 42);
    Ok(())
}

#[test]
fn pop_this() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Pointer.get_base_address()] = 320.into();
    let instructions = AsmInstruction::parse("push constant 42\npop this 2");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(computer.get_memory().ram.data[322], 42);
    Ok(())
}

#[test]
fn pop_that() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    computer.get_memory_mut().ram.data[Segment::Pointer.get_base_address() + 1] = 320.into();
    let instructions = AsmInstruction::parse("push constant 42\npop that 3");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(computer.get_memory().ram.data[323], 42);
    Ok(())
}

#[test]
fn pop_temp() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 42\npop temp 4");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(
        computer.get_memory().ram.data[Segment::Temp.get_base_address() + 4],
        42
    );
    Ok(())
}

#[test]
fn pop_static() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 42\npop static 0");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 256);
    assert_eq!(
        computer.get_memory().ram.data[Segment::Static.get_base_address()],
        42
    );
    Ok(())
}

#[test]
fn add() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 38\npush constant 4\nadd");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 42);
    Ok(())
}

#[test]
fn sub() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 38\npush constant 4\nsub");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 34);
    Ok(())
}

#[test]
fn eq() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse(
        r#"push constant 4
        push constant 4
        eq
        push constant 5
        push constant 4
        eq"#,
    );
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 258);
    assert_eq!(computer.get_memory().ram.data[257], Signal16::FALSE);
    Ok(())
}

#[test]
fn lt() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse(
        r#"push constant 4
        push constant 4
        lt
        push constant 5
        push constant 4
        lt
        push constant 4
        push constant 5
        lt
        "#,
    );
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 259);
    assert_eq!(computer.get_memory().ram.data[258], Signal16::TRUE);
    Ok(())
}

#[test]
fn gt() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse(
        r#"push constant 4
        push constant 4
        gt
        push constant 5
        push constant 4
        gt
        push constant 4
        push constant 5
        gt
        "#,
    );
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 259);
    assert_eq!(computer.get_memory().ram.data[258], Signal16::FALSE);
    Ok(())
}

#[test]
fn neg() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse(
        r#"push constant 4
        neg
        "#,
    );
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], -4);
    Ok(())
}

#[test]
fn and() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 7\npush constant 2\nand");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 2);
    Ok(())
}

#[test]
fn or() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 1\npush constant 2\nor");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], 3);
    Ok(())
}

#[test]
fn not() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::default();
    let instructions = AsmInstruction::parse("push constant 0\nnot");
    computer.set_instructions(instructions);
    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram.data[0], 257);
    assert_eq!(computer.get_memory().ram.data[256], Signal16::TRUE);
    Ok(())
}
