// Copyright © 2022
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use acs::{
    error::CalError,
    expression::{Expression, Literal, Operator, Term, UnaryOperator},
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
    assert_eq!(function.return_type, Type::I16);

    let module: Module = "fn main() -> i16 { return 0; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert!(function.parameters.is_empty());
    assert_eq!(function.body_statements.len(), 1);
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
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::Literal(Literal::I16(1)));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::Add);
    assert_eq!(*rhs.term.as_ref(), Term::Literal(Literal::I16(2)));
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
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::If(ifstat) = statement else {
        panic!();
    };
    assert_eq!(
        ifstat.predicate.term.as_ref(),
        &Term::Literal(Literal::Bool(true))
    );
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
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::While(whilestat) = statement else {
        panic!();
    };
    assert_eq!(
        whilestat.predicate.term.as_ref(),
        &Term::Literal(Literal::Bool(true))
    );
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
    assert_eq!(function.return_type, Type::Bool);

    let statement = &function.body_statements[0];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Eq);
    assert_eq!(rhs_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[1];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Ne);
    assert_eq!(rhs_expr.term.as_ref(), &Term::Literal(Literal::I16(2)));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[2];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Lt);
    assert_eq!(rhs_expr.term.as_ref(), &Term::Literal(Literal::I16(2)));
    assert!(rhs_expr.op_and_expr.is_none());

    let statement = &function.body_statements[3];
    let Statement::Expression(eq_expr) = statement else {
        panic!();
    };
    assert_eq!(eq_expr.term.as_ref(), &Term::Literal(Literal::I16(2)));
    let Some((op, rhs_expr)) = &eq_expr.op_and_expr else {
        panic!();
    };
    assert_eq!(*op, Operator::Gt);
    assert_eq!(rhs_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
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
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::I16(0)));
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
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::Literal(Literal::I16(1)));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::Mul);
    assert_eq!(*rhs.term.as_ref(), Term::Literal(Literal::I16(2)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn and() -> Result<(), CalError> {
    let module: Module = "fn main() { 1 & 2; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::Literal(Literal::I16(1)));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::And);
    assert_eq!(*rhs.term.as_ref(), Term::Literal(Literal::I16(2)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn or() -> Result<(), CalError> {
    let module: Module = "fn main() { 1 | 2; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expression) = statement else {
        panic!();
    };
    assert_eq!(*expression.term.as_ref(), Term::Literal(Literal::I16(1)));
    let (op, rhs) = expression.op_and_expr.as_ref().unwrap();
    assert_eq!(*op, Operator::Or);
    assert_eq!(*rhs.term.as_ref(), Term::Literal(Literal::I16(2)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn array() -> Result<(), CalError> {
    let module: Module = r#"fn main() {
        let a: [i16; 2] = [1, 2];
        a[1] = 3;
    }"#
    .parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 2);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Let(variable, rhs) = statement else {
        panic!();
    };
    assert_eq!(variable.name, "a");
    assert_eq!(variable.typ, Type::Array(Box::new(Type::I16), 2));
    let Term::Literal(Literal::Array(array)) = rhs.term.as_ref() else {
        panic!();
    };
    assert!(rhs.op_and_expr.is_none());

    assert_eq!(array.len(), 2);
    assert_eq!(array[0], Literal::I16(1));
    assert_eq!(array[1], Literal::I16(2));

    let statement = &function.body_statements[1];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    let Term::Index(var_name, index_expr) = expr.term.as_ref() else {
        panic!();
    };
    assert_eq!(var_name, "a");
    assert_eq!(index_expr.term.as_ref(), &Term::Literal(Literal::I16(1)));
    assert!(index_expr.op_and_expr.is_none());

    let Some((op, rhs)) = expr.op_and_expr.as_ref() else {
        panic!();
    };
    assert_eq!(*op, Operator::Assign);
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::I16(3)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn character() -> Result<(), CalError> {
    let module: Module = "fn main() { let a: char = 'a'; }".parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Let(variable, rhs) = statement else {
        panic!();
    };
    assert_eq!(variable.name, "a");
    assert_eq!(variable.typ, Type::Char);
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::Char('a')));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn reference() -> Result<(), CalError> {
    let module: Module = r#"
    fn main() -> i16 {
        let a: i16 = 1;
        pass(&a);
        a
    }
    fn pass(a: &i16) {
        a = 2;
    }
    "#
    .parse()?;
    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 3);
    assert_eq!(function.return_type, Type::I16);

    let statement = &function.body_statements[0];
    let Statement::Let(variable, rhs) = statement else {
        panic!();
    };
    assert_eq!(variable.name, "a");
    assert_eq!(variable.typ, Type::I16);
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::I16(1)));
    assert!(rhs.op_and_expr.is_none());

    let statement = &function.body_statements[1];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    let Term::Call(function_name, args) = expr.term.as_ref() else {
        panic!();
    };
    assert!(expr.op_and_expr.is_none());
    assert_eq!(function_name, "pass");
    assert_eq!(args.len(), 1);
    let Term::UnaryOp(UnaryOperator::Ref, rhs) = args[0].term.as_ref() else {
        panic!();
    };
    assert!(args[0].op_and_expr.is_none());
    assert_eq!(rhs.as_ref(), &Term::Variable(String::from("a")));

    let statement = &function.body_statements[2];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    assert_eq!(expr.term.as_ref(), &Term::Variable(String::from("a")));
    assert!(expr.op_and_expr.is_none());

    let function = &module.functions[1];
    assert_eq!(function.name, "pass");
    assert_eq!(function.parameters.len(), 1);
    assert_eq!(function.body_statements.len(), 1);
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
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::I16(2)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}

#[test]
fn array_of_array_reference() -> Result<(), CalError> {
    let module: Module = r#"fn main() -> [i16; 2] {
            let a: [[i16; 2]; 2] = [[1, 2], [3, 4]];
            pass(&a[1]);
            a[1]
        }

        fn pass(a: &[i16; 2]) {
            a[1] = 5;
        }"#
    .parse()?;

    let function = &module.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.parameters.len(), 0);
    assert_eq!(function.body_statements.len(), 3);
    let array_i16_2 = Type::Array(Box::new(Type::I16), 2);
    assert_eq!(function.return_type, array_i16_2);

    let statement = &function.body_statements[0];
    let Statement::Let(variable, rhs) = statement else {
        panic!();
    };
    assert_eq!(variable.name, "a");
    assert_eq!(variable.typ, Type::Array(Box::new(array_i16_2), 2));
    assert_eq!(
        rhs.term.as_ref(),
        &Term::Literal(Literal::Array(vec![
            Literal::Array(vec![Literal::I16(1), Literal::I16(2)]),
            Literal::Array(vec![Literal::I16(3), Literal::I16(4)]),
        ]))
    );
    assert!(rhs.op_and_expr.is_none());

    let statement = &function.body_statements[1];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    let Term::Call(function_name, args) = expr.term.as_ref() else {
        panic!();
    };
    assert!(expr.op_and_expr.is_none());
    assert_eq!(function_name, "pass");
    assert_eq!(args.len(), 1);
    let Term::UnaryOp(UnaryOperator::Ref, rhs) = args[0].term.as_ref() else {
        panic!();
    };
    assert!(args[0].op_and_expr.is_none());
    let a_index_1 = Term::Index(
        "a".into(),
        Expression::new(Box::new(Term::Literal(Literal::I16(1))), None),
    );
    assert_eq!(rhs.as_ref(), &a_index_1);

    let statement = &function.body_statements[2];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    assert_eq!(expr.term.as_ref(), &a_index_1);
    assert!(expr.op_and_expr.is_none());

    let function = &module.functions[1];
    assert_eq!(function.name, "pass");
    assert_eq!(function.parameters.len(), 1);
    assert_eq!(function.body_statements.len(), 1);
    assert_eq!(function.return_type, Type::Void);

    let statement = &function.body_statements[0];
    let Statement::Expression(expr) = statement else {
        panic!();
    };
    assert_eq!(expr.term.as_ref(), &a_index_1);
    let Some((op, rhs)) = expr.op_and_expr.as_ref() else {
        panic!();
    };
    assert_eq!(*op, Operator::Assign);
    assert_eq!(rhs.term.as_ref(), &Term::Literal(Literal::I16(5)));
    assert!(rhs.op_and_expr.is_none());

    Ok(())
}
