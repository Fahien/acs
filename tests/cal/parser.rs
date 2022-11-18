// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError,
    expression::{Operator, Term},
    statement::Statement,
    structure::{Module, Type},
};

#[test]
fn hello_void() -> Result<(), CalError> {
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
fn return_zero() -> Result<(), CalError> {
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

#[test]
fn def_local() -> Result<(), CalError> {
    let module: Module = "fn def_local() { let x: i16 = 0; let y: i16 = 1; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "def_local");
    assert!(function.parameters.is_empty());
    assert_eq!(function.body_statements.len(), 2);
    assert_eq!(function.local_count, 2);
    assert_eq!(function.return_type, Type::Void);
    Ok(())
}

#[test]
fn call_function() -> Result<(), CalError> {
    let module: Module = "fn main() { call() }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert!(function.parameters.is_empty());
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Void);
    let statement = &function.body_statements[0];
    if let Statement::Expression(expression) = statement {
        assert!(matches!(expression.term.as_ref(), &Term::Call(_, _)));
    } else {
        panic!()
    }
    Ok(())
}

#[test]
fn one_parameter() -> Result<(), CalError> {
    let module: Module = "fn identity(x: i16) -> i16 { x }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "identity");
    assert_eq!(function.parameters.len(), 1);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::I16);
    Ok(())
}

#[test]
fn multi_parameters() -> Result<(), CalError> {
    let module: Module =
        "fn ignore_y(x: i16, y: i16) -> i16 { x } fn main() -> i16 { ignore_y(2, 3) }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "ignore_y");
    assert_eq!(function.parameters.len(), 2);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::I16);
    Ok(())
}

#[test]
fn add() -> Result<(), CalError> {
    let module: Module = "fn main() { 1 + 2; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::IntLiteral(1));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::Add);
    assert_eq!(*rhs.term.as_ref(), Term::IntLiteral(2));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn if_statement() -> Result<(), CalError> {
    let module: Module = "fn main() -> bool { if true { true } else { false } }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::If(ifstat) = statement else {
        panic!();
    };
    assert_eq!(ifstat.predicate.term.as_ref(), &Term::BoolLiteral(true));
    assert!(ifstat.predicate.op_and_expr.is_none());
    assert_eq!(ifstat.if_branch.len(), 1);
    assert_eq!(ifstat.else_branch.len(), 1);

    Ok(())
}

#[test]
fn while_statement() -> Result<(), CalError> {
    let module: Module = "fn main() -> bool { while true { return true; } false }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 2);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::While(whilestat) = statement else {
        panic!();
    };
    assert_eq!(whilestat.predicate.term.as_ref(), &Term::BoolLiteral(true));
    assert!(whilestat.predicate.op_and_expr.is_none());
    assert_eq!(whilestat.body.len(), 1);

    Ok(())
}

#[test]
fn cmp() -> Result<(), CalError> {
    let module: Module = r#"
        fn main() -> bool {
            1 == 1;
            1 != 2;
            1 < 2;
            2 > 1
        }"#
    .parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 4);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::IntLiteral(1));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Eq);
    assert_eq!(rhs_expr.term.as_ref(), &Term::IntLiteral(1));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[1];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::IntLiteral(1));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Ne);
    assert_eq!(rhs_expr.term.as_ref(), &Term::IntLiteral(2));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[2];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::IntLiteral(1));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Lt);
    assert_eq!(rhs_expr.term.as_ref(), &Term::IntLiteral(2));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[3];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::IntLiteral(2));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Gt);
    assert_eq!(rhs_expr.term.as_ref(), &Term::IntLiteral(1));
    assert!(rhs_expr.op_and_expr.is_none());

    Ok(())
}

#[test]
fn assign_expression() -> Result<(), CalError> {
    let module: Module = r#"
        fn main() {
            a = 0;
        }"#
    .parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expr) = statement else {
        panic!();
    };

    assert_eq!(expr.term.as_ref(), &Term::Variable(String::from("a")));
    let Some((op, rhs)) = expr.op_and_expr.as_ref() else {
        panic!();
    };
    assert_eq!(*op, Operator::Assign);
    assert_eq!(rhs.term.as_ref(), &Term::IntLiteral(0));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn mul() -> Result<(), CalError> {
    let module: Module = "fn main() { 1 * 2; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.local_count, 0);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::IntLiteral(1));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::Mul);
    assert_eq!(*rhs.term.as_ref(), Term::IntLiteral(2));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}
