use std::error::Error;

use acs::{
    asm::instruction::AsmInstruction, code::VmCode, preprocessor::VmPreprocessedCode,
    segment::Segment, Computer, Signal16, VmTranslator,
};

const CYCLE_COUNT: u16 = 512;

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

#[test]
fn basic_loop() -> Result<(), Box<dyn Error>> {
    let instructions = AsmInstruction::parse(
        r#"
        // Computes the sum 1 + 2 + ... + argument[0] and pushes the
        // result onto the stack. Argument[0] is initialized by the test
        // script before this code starts running.
        push constant 0    
        pop local 0         // initializes sum = 0
        label LOOP_START
        push argument 0    
        push local 0
        add
        pop local 0	        // sum = sum + counter
        push argument 0
        push constant 1
        sub
        pop argument 0      // counter--
        push argument 0
        if-goto LOOP_START  // If counter != 0, goto LOOP_START
        push local 0
        "#,
    );

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    computer.get_memory_mut().ram[Segment::Local.get_base_address()] = 300;
    computer.get_memory_mut().ram[Segment::Argument.get_base_address()] = 400;
    computer.get_memory_mut().ram[400] = 3;

    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory_mut().ram[300], 6);
    Ok(())
}

#[test]
fn fibonacci_series() -> Result<(), Box<dyn Error>> {
    let instructions = AsmInstruction::parse(
        r#"
        // Puts the first argument[0] elements of the Fibonacci series
        // in the memory, starting in the address given in argument[1].
        // Argument[0] and argument[1] are initialized by the test script 
        // before this code starts running.
        
        push argument 1
        pop pointer 1           // that = argument[1]
        
        push constant 0
        pop that 0              // first element in the series = 0
        push constant 1
        pop that 1              // second element in the series = 1
        
        push argument 0
        push constant 2
        sub
        pop argument 0          // num_of_elements -= 2 (first 2 elements are set)
        
        label MAIN_LOOP_START
        
        push argument 0
        if-goto COMPUTE_ELEMENT // if num_of_elements > 0, goto COMPUTE_ELEMENT
        goto END_PROGRAM        // otherwise, goto END_PROGRAM
        
        label COMPUTE_ELEMENT
        
        push that 0
        push that 1
        add
        pop that 2              // that[2] = that[0] + that[1]
        
        push pointer 1
        push constant 1
        add
        pop pointer 1           // that += 1
        
        push argument 0
        push constant 1
        sub
        pop argument 0          // num_of_elements--
        
        goto MAIN_LOOP_START
        
        label END_PROGRAM
        "#,
    );

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    computer.get_memory_mut().ram[Segment::Local.get_base_address()] = 300;
    computer.get_memory_mut().ram[Segment::Argument.get_base_address()] = 400;
    computer.get_memory_mut().ram[400] = 6;
    computer.get_memory_mut().ram[401] = 3000;

    for _ in 0..CYCLE_COUNT {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory_mut().ram[3000], 0);
    assert_eq!(computer.get_memory_mut().ram[3001], 1);
    assert_eq!(computer.get_memory_mut().ram[3002], 1);
    assert_eq!(computer.get_memory_mut().ram[3003], 2);
    assert_eq!(computer.get_memory_mut().ram[3004], 3);
    assert_eq!(computer.get_memory_mut().ram[3005], 5);
    Ok(())
}

#[test]
fn simple_function() -> Result<(), Box<dyn Error>> {
    let instructions = AsmInstruction::parse(
        r#"
            // Performs a simple calculation and returns the result.
            function SimpleFunction.test 2
            push local 0
            push local 1
            add
            not
            push argument 0
            add
            push argument 1
            sub
            return
        "#,
    );

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    computer.get_memory_mut().ram[0] = 317;
    computer.get_memory_mut().ram[1] = 317;
    computer.get_memory_mut().ram[2] = 310;
    computer.get_memory_mut().ram[3] = 3000;
    computer.get_memory_mut().ram[4] = 4000;
    computer.get_memory_mut().ram[310] = 1234;
    computer.get_memory_mut().ram[311] = 37;
    computer.get_memory_mut().ram[312] = 1000;
    computer.get_memory_mut().ram[313] = 305;
    computer.get_memory_mut().ram[314] = 300;
    computer.get_memory_mut().ram[315] = 3010;
    computer.get_memory_mut().ram[316] = 4010;

    for _ in 0..512 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory_mut().ram[0], 311);
    assert_eq!(computer.get_memory_mut().ram[1], 305);
    assert_eq!(computer.get_memory_mut().ram[2], 300);
    assert_eq!(computer.get_memory_mut().ram[3], 3010);
    assert_eq!(computer.get_memory_mut().ram[4], 4010);
    assert_eq!(computer.get_memory_mut().ram[310], 1196);
    Ok(())
}

#[test]
fn fibonacci_element() -> Result<(), Box<dyn Error>> {
    let sys_code = VmCode::new(
        "sys",
        r#"
        // Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
        // function, which computes the n'th element of the Fibonacci series.
        // Note that by convention, the Sys.init function is called "automatically" 
        // by the bootstrap code.

        function Sys.init 0
        push constant 4
        call Main.fibonacci 1   // computes the 4'th fibonacci element
        label WHILE
        goto WHILE              // loops infinitely
        "#,
    );

    let main_code = VmCode::new(
        "main",
        r#"
        // Computes the n'th element of the Fibonacci series, recursively.
        // n is given in argument[0].  Called by the Sys.init function 
        // (part of the Sys.vm file), which also pushes the argument[0] 
        // parameter before this code starts running.

        function Main.fibonacci 0
        push argument 0
        push constant 2
        lt                     // checks if n<2
        if-goto IF_TRUE
        goto IF_FALSE
        label IF_TRUE          // if n<2, return n
        push argument 0        
        return
        label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
        push argument 0
        push constant 2
        sub
        call Main.fibonacci 1  // computes fib(n-2)
        push argument 0
        push constant 1
        sub
        call Main.fibonacci 1  // computes fib(n-1)
        add                    // returns fib(n-1) + fib(n-2)
        return
        "#,
    );

    let preprocessed_code = VmPreprocessedCode::builder()
        .include(sys_code)
        .include(main_code)
        .build();

    let vm_instructions = preprocessed_code.into();

    let instructions = VmTranslator::default().translate(vm_instructions);

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    // Ignore set SP=256
    for _ in 0..4 {
        computer.ticktock();
    }
    computer.get_memory_mut().ram[0] = 261;

    for _ in 0..6000 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 262);
    assert_eq!(computer.get_memory().ram[261], 3);
    Ok(())
}

#[test]
fn statics() -> Result<(), Box<dyn Error>> {
    let sys_code = VmCode::new(
        "sys",
        r#"
        // Tests that different functions, stored in two different 
        // class files, manipulate the static segment correctly. 
        function Sys.init 0
        push constant 6
        push constant 8
        call Class1.set 2
        pop temp 0 // Dumps the return value
        push constant 23
        push constant 15
        call Class2.set 2
        pop temp 0 // Dumps the return value
        call Class1.get 0
        call Class2.get 0
        label WHILE
        goto WHILE
        "#,
    );

    let class1_code = VmCode::new(
        "class1",
        r#"
        // Stores two supplied arguments in static[0] and static[1].
        function Class1.set 0
        push argument 0
        pop static 0
        push argument 1
        pop static 1
        push constant 0
        return

        // Returns static[0] - static[1].
        function Class1.get 0
        push static 0
        push static 1
        sub
        return
        "#,
    );

    let class2_code = VmCode::new(
        "class2",
        r#"
        // Stores two supplied arguments in static[0] and static[1].
        function Class2.set 0
        push argument 0
        pop static 0
        push argument 1
        pop static 1
        push constant 0
        return

        // Returns static[0] - static[1].
        function Class2.get 0
        push static 0
        push static 1
        sub
        return
        "#,
    );

    let preprocessed = VmPreprocessedCode::builder()
        .include(sys_code)
        .include(class1_code)
        .include(class2_code)
        .build();
    let vm_instructions = preprocessed.into();

    let instructions = VmTranslator::default().translate(vm_instructions);

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    for _ in 0..4 {
        computer.ticktock();
    }
    computer.get_memory_mut().ram[0] = 261; // stack pointer

    for _ in 0..2500 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 263);
    assert_eq!(computer.get_memory().ram[261], -2);
    assert_eq!(computer.get_memory().ram[262], 8);
    Ok(())
}

#[test]
fn nested_call() -> Result<(), Box<dyn Error>> {
    let instructions = AsmInstruction::parse(
        r#"
        // Sys.init()
        //
        // Calls Sys.main() and stores return value in temp 1.
        // Does not return.  (Enters infinite loop.)

        function Sys.init 0
        push constant 4000	// test THIS and THAT context save
        pop pointer 0
        push constant 5000
        pop pointer 1
        call Sys.main 0
        pop temp 1
        label LOOP
        goto LOOP

        // Sys.main()
        //
        // Sets locals 1, 2 and 3, leaving locals 0 and 4 unchanged to test
        // default local initialization to 0.  (RAM set to -1 by test setup.)
        // Calls Sys.add12(123) and stores return value (135) in temp 0.
        // Returns local 0 + local 1 + local 2 + local 3 + local 4 (456) to confirm
        // that locals were not mangled by function call.

        function Sys.main 5
        push constant 4001
        pop pointer 0
        push constant 5001
        pop pointer 1
        push constant 200
        pop local 1
        push constant 40
        pop local 2
        push constant 6
        pop local 3
        push constant 123
        call Sys.add12 1
        pop temp 0
        push local 0
        push local 1
        push local 2
        push local 3
        push local 4
        add
        add
        add
        add
        return

        // Sys.add12(int n)
        //
        // Returns n+12.

        function Sys.add12 0
        push constant 4002
        pop pointer 0
        push constant 5002
        pop pointer 1
        push argument 0
        push constant 12
        add
        return
        "#,
    );

    let mut computer = Computer::default();
    computer.set_instructions(instructions);

    // Ignore set SP=256
    for _ in 0..4 {
        computer.ticktock();
    }

    computer.get_memory_mut().ram[0] = 261;
    computer.get_memory_mut().ram[1] = 261;
    computer.get_memory_mut().ram[2] = 256;
    computer.get_memory_mut().ram[3] = -3;
    computer.get_memory_mut().ram[4] = -4;
    computer.get_memory_mut().ram[5] = -1;
    computer.get_memory_mut().ram[6] = -1;
    computer.get_memory_mut().ram[256] = 1234;
    computer.get_memory_mut().ram[257] = -1;
    computer.get_memory_mut().ram[256] = -2;
    computer.get_memory_mut().ram[259] = -3;
    computer.get_memory_mut().ram[260] = -4;

    for i in 261..300 {
        computer.get_memory_mut().ram[i] = -1;
    }

    for _ in 0..4000 {
        computer.ticktock();
    }
    assert_eq!(computer.get_memory().ram[0], 261);
    assert_eq!(computer.get_memory().ram[1], 261);
    assert_eq!(computer.get_memory().ram[2], 256);
    assert_eq!(computer.get_memory().ram[3], 4000);
    assert_eq!(computer.get_memory().ram[4], 5000);
    assert_eq!(computer.get_memory().ram[5], 135);
    assert_eq!(computer.get_memory().ram[6], 246);
    Ok(())
}
