use fabc_parser::{ast::expr::Expr, Parser};
use fabc_vm::{
    instructions::Instruction, program::Program, translator::AstTranslator, value::Value,
    VirtualMachine,
};

fn interpret_expr(source: &str) -> Value {
    let ast = Parser::parse_ast_str::<Expr>(source).expect("failed to parse expression");

    let mut instructions = AstTranslator::translate(&ast);
    instructions.push(Instruction::Halt);

    let program = Program::new(instructions);
    let vm = VirtualMachine::interpret(&program).expect("failed to interpret program");

    let value = vm
        .last_value()
        .cloned()
        .expect("expected value on the VM stack");

    assert_eq!(vm.stack().len(), 1, "expected a single value on the stack");

    value
}

#[test]
fn interprets_arithmetic_and_logic_expression() {
    let value = interpret_expr("1 + 2 * 3 == 7 and false or true");
    assert_eq!(value, Value::Boolean(true));
}

#[test]
fn respects_arithmetic_precedence() {
    let value = interpret_expr("10 - 2 * 3 + 1");
    assert_eq!(value, Value::Number(5));
}
