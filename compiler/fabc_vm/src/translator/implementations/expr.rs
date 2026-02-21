use std::rc::Rc;

use fabc_parser::ast::expr::{
    literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary,
};

use crate::{instructions::Instruction, translator::Translatable, value::Value};

impl Translatable for Expr {
    fn translate_with(
        &self,
        translator: &mut crate::translator::AstTranslator,
        buffer: &mut Vec<Instruction>,
    ) {
        match &self {
            Expr::Binary {
                left,
                operator,
                right,
                ..
            } => {
                left.translate_with(translator, buffer);
                right.translate_with(translator, buffer);
                let operator_instruction = match operator {
                    BinaryOperator::Add => Instruction::Add,
                    BinaryOperator::Subtraction => Instruction::Sub,
                    BinaryOperator::Multiply => Instruction::Mul,
                    BinaryOperator::Divide => Instruction::Div,
                    BinaryOperator::EqualEqual => Instruction::Eq,
                    BinaryOperator::NotEqual => Instruction::Neq,
                    BinaryOperator::Less => Instruction::Le,
                    BinaryOperator::Greater => Instruction::Gr,
                    BinaryOperator::LessEqual => Instruction::Leq,
                    BinaryOperator::GreaterEqual => Instruction::Geq,
                    BinaryOperator::And => Instruction::And,
                    BinaryOperator::Or => Instruction::Or,
                };
                buffer.push(operator_instruction);
            }
            Expr::Primary { value, .. } => value.translate_with(translator, buffer),
            _ => todo!(),
        }
    }
}

impl Translatable for Primary {
    fn translate_with(
        &self,
        translator: &mut crate::translator::AstTranslator,
        buffer: &mut Vec<Instruction>,
    ) {
        match &self {
            Primary::Literal(literal) => literal.translate_with(translator, buffer),
            Primary::Primitive(primitive) => primitive.translate_with(translator, buffer),
        }
    }
}

impl Translatable for Literal {
    fn translate_with(
        &self,
        _translator: &mut crate::translator::AstTranslator,
        buffer: &mut Vec<Instruction>,
    ) {
        let instruction = match self {
            Literal::Boolean { value, .. } => Instruction::LoadConstant(Value::Boolean(*value)),
            Literal::String { value, .. } => {
                Instruction::LoadConstant(Value::String(Rc::new(value.clone())))
            }
            Literal::Number { value, .. } => {
                Instruction::LoadConstant(Value::Number(*value as i64))
            }
            Literal::None { .. } => Instruction::LoadConstant(Value::None),
        };
        buffer.push(instruction);
    }
}

impl Translatable for Primitive {
    fn translate_with(
        &self,
        translator: &mut crate::translator::AstTranslator,
        buffer: &mut Vec<Instruction>,
    ) {
        match self {
            Primitive::Identifier { .. } => todo!(),
            Primitive::Grouping { expr, .. } => expr.translate_with(translator, buffer),
            Primitive::Object { .. } => todo!(),
            Primitive::Closure { .. } => todo!(),
            Primitive::StoryIdentifier { .. } => todo!(),
            Primitive::Context { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use fabc_parser::{
        ast::expr::{literal::Literal, Expr},
        Parser,
    };
    use insta::assert_debug_snapshot;

    use crate::translator::AstTranslator;

    #[test]
    fn translates_literals() {
        let ast = Parser::parse_ast_str::<Literal>("\"test\"").expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_string_literal", instructions);

        let ast = Parser::parse_ast_str::<Literal>("42").expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_number_literal", instructions);

        let ast = Parser::parse_ast_str::<Literal>("true").expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_boolean_literal", instructions);

        let ast = Parser::parse_ast_str::<Literal>("none").expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_none_literal", instructions);
    }

    #[test]
    fn translates_binary_expression() {
        let source = "1 + 2 * 3 - 4 / 5 == 6 != 7 < 8 > 9 <= 10 >= 11 and true or false";
        let ast = Parser::parse_ast_str::<Expr>(source).expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_binary_expression", instructions);
    }
}
