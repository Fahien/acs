use std::error::Error;

use acs::{segment::Segment, vm::instruction::VmInstruction, VmEmulator};

#[test]
fn push_constant() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 42");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_local() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Local.get_base_address()] = 320;
    emulator.ram[320] = 42;
    let instructions = VmInstruction::parse("push local 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_this() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Pointer.get_base_address()] = 320;
    emulator.ram[320] = 42;
    let instructions = VmInstruction::parse("push this 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_that() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Pointer.get_base_address() + 1] = 320;
    emulator.ram[320] = 42;
    let instructions = VmInstruction::parse("push that 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_argument() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Argument.get_base_address()] = 320;
    emulator.ram[320] = 42;
    let instructions = VmInstruction::parse("push argument 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_temp() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Temp.get_base_address()] = 42;
    let instructions = VmInstruction::parse("push temp 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn push_static() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Static.get_base_address()] = 42;
    let instructions = VmInstruction::parse("push static 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn pop_local() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Local.get_base_address()] = 320;
    let instructions = VmInstruction::parse("push constant 42\npop local 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[320], 42);
    Ok(())
}

#[test]
fn pop_argument() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Argument.get_base_address()] = 320;
    let instructions = VmInstruction::parse("push constant 42\npop argument 1");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[321], 42);
    Ok(())
}

#[test]
fn pop_this() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Pointer.get_base_address()] = 320;
    let instructions = VmInstruction::parse("push constant 42\npop this 2");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[322], 42);
    Ok(())
}

#[test]
fn pop_that() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    emulator.ram[Segment::Pointer.get_base_address() + 1] = 320;
    let instructions = VmInstruction::parse("push constant 42\npop that 3");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[323], 42);
    Ok(())
}

#[test]
fn pop_temp() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 42\npop temp 4");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[Segment::Temp.get_base_address() + 4], 42);
    Ok(())
}

#[test]
fn pop_static() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 42\npop static 0");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    emulator.step();
    assert_eq!(emulator.ram[0], 256);
    assert_eq!(emulator.ram[Segment::Static.get_base_address()], 42);
    Ok(())
}

#[test]
fn add() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 38\npush constant 4\nadd");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 38);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 42);
    Ok(())
}

#[test]
fn sub() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 38\npush constant 4\nsub");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 38);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 34);
    Ok(())
}

#[test]
fn eq() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse(
        r#"push constant 4
        push constant 4
        eq
        push constant 5
        push constant 4
        eq"#,
    );
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], -1);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 5);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 0);
    Ok(())
}

#[test]
fn lt() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse(
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
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 0);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 5);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 0);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 260);
    assert_eq!(emulator.ram[259], 5);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], -1);
    Ok(())
}

#[test]
fn gt() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse(
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
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 0);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 5);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], -1);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 260);
    assert_eq!(emulator.ram[259], 5);
    emulator.step();
    assert_eq!(emulator.ram[0], 259);
    assert_eq!(emulator.ram[258], 0);
    Ok(())
}

#[test]
fn neg() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse(
        r#"push constant 4
        neg
        "#,
    );
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 4);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], -4);
    Ok(())
}

#[test]
fn and() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 7\npush constant 2\nand");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 7);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 2);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 2);
    Ok(())
}

#[test]
fn or() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 1\npush constant 2\nor");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 1);
    emulator.step();
    assert_eq!(emulator.ram[0], 258);
    assert_eq!(emulator.ram[257], 2);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 3);
    Ok(())
}

#[test]
fn not() -> Result<(), Box<dyn Error>> {
    let mut emulator = VmEmulator::default();
    let instructions = VmInstruction::parse("push constant 0\nnot");
    emulator.load(instructions);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], 0);
    emulator.step();
    assert_eq!(emulator.ram[0], 257);
    assert_eq!(emulator.ram[256], -1);
    Ok(())
}
