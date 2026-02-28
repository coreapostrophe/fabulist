use std::rc::Rc;

use fabc_error::{kind::InternalErrorKind as TranslationError, Error};
use fabc_parser::ast::expr::{
    literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary, UnaryOperator,
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
            Expr::Unary {
                operator, right, ..
            } => {
                right.translate_with(translator, buffer);
                let operator_instruction = match operator {
                    UnaryOperator::Negate => Instruction::Neg,
                    UnaryOperator::Not => Instruction::Not,
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
            Primitive::Identifier { info, name } => {
                if let Some(binding) = translator.resolve_binding(info.id) {
                    buffer.push(Instruction::LoadLocal(binding.slot));
                } else {
                    translator.push_error(Error::new(
                        TranslationError::MissingIdentifierBinding {
                            node_id: info.id,
                            name: name.clone(),
                        },
                        info.span.clone(),
                    ));
                    buffer.push(Instruction::LoadConstant(Value::None));
                }
            }
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
    use std::collections::HashMap;

    use fabc_analyzer::types::{
        BindingDetails, BindingKind, DataType, ModuleSymbolType, SymbolAnnotation,
    };
    use fabc_parser::{
        ast::expr::primitive::Primitive,
        ast::expr::{literal::Literal, Expr, Primary},
        Parser,
    };
    use insta::assert_debug_snapshot;

    use crate::translator::AstTranslator;
    use fabc_error::{kind::ErrorKind, kind::InternalErrorKind as TranslationError};

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

    #[test]
    fn translates_unary_expression() {
        let source = "!-42";
        let ast = Parser::parse_ast_str::<Expr>(source).expect("Failed to parse source");
        let instructions = AstTranslator::translate(&ast);
        assert_debug_snapshot!("translates_unary_expression", instructions);
    }

    #[test]
    fn translates_identifier() {
        let ast = Parser::parse_ast_str::<Expr>("foo").expect("Failed to parse source");
        let ident_id = match &ast {
            Expr::Primary {
                value: Primary::Primitive(Primitive::Identifier { info, .. }),
                ..
            } => info.id,
            _ => panic!("unexpected AST for identifier"),
        };

        let mut annotations = HashMap::new();
        annotations.insert(
            ident_id,
            SymbolAnnotation {
                name: Some("foo".to_string()),
                r#type: ModuleSymbolType::Data(DataType::None),
                binding: Some(BindingDetails {
                    slot: 0,
                    depth: 0,
                    distance: 0,
                    kind: BindingKind::Local,
                }),
            },
        );

        let result = AstTranslator::translate_with_annotations_result(&ast, annotations);
        assert!(result.errors.is_empty());
        let instructions = result.instructions;
        assert_debug_snapshot!("translates_identifier", instructions);
    }

    #[test]
    fn reports_missing_identifier_binding() {
        let ast = Parser::parse_ast_str::<Expr>("foo").expect("Failed to parse source");
        let ident_info = match &ast {
            Expr::Primary {
                value: Primary::Primitive(Primitive::Identifier { info, .. }),
                ..
            } => info,
            _ => panic!("unexpected AST for identifier"),
        };

        let result = AstTranslator::translate_with_annotations_result(&ast, HashMap::new());

        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].span, ident_info.span);
        assert_eq!(
            result.errors[0].kind,
            ErrorKind::Internal(TranslationError::MissingIdentifierBinding {
                node_id: ident_info.id,
                name: "foo".to_string(),
            })
        );
    }
}
