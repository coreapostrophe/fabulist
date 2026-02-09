use fabc_error::{kind::InternalErrorKind, Error, Span};
use fabc_parser::ast::expr::{literal::Literal as AstLiteral, primitive::Primitive, Expr, Primary};

use crate::{
    Block, GenerateIR, IRGenerator, IRResult, Literal, Operand, Procedure, Quadruple, TempId,
    Terminator,
};

impl GenerateIR for Expr {
    fn generate_ir(&self, generator: &mut IRGenerator) -> IRResult {
        match self {
            Expr::Primary { value, .. } => value.generate_ir(generator),
            Expr::Grouping { expression, .. } => expression.generate_ir(generator),
            Expr::Unary {
                operator,
                right,
                info,
            } => {
                let mut right_ir = right.generate_ir(generator);
                let Some(arg) = expect_operand(
                    right_ir.operand.clone(),
                    generator,
                    &info.span,
                    InternalErrorKind::MissingOperand,
                ) else {
                    return IRResult {
                        operand: None,
                        quadruples: right_ir.quadruples,
                    };
                };

                let dest = generator.fresh_temp();

                right_ir.quadruples.push(Quadruple::Unary {
                    op: *operator,
                    arg,
                    dest,
                });

                IRResult {
                    operand: Some(Operand::Temp(dest)),
                    quadruples: right_ir.quadruples,
                }
            }
            Expr::Binary {
                operator,
                left,
                right,
                info,
                ..
            } => {
                let mut left_ir = left.generate_ir(generator);
                let mut right_ir = right.generate_ir(generator);

                let Some(lhs) = expect_operand(
                    left_ir.operand.clone(),
                    generator,
                    &info.span,
                    InternalErrorKind::MissingOperand,
                ) else {
                    left_ir.quadruples.append(&mut right_ir.quadruples);
                    return IRResult {
                        operand: None,
                        quadruples: left_ir.quadruples,
                    };
                };

                let Some(rhs) = expect_operand(
                    right_ir.operand.clone(),
                    generator,
                    &info.span,
                    InternalErrorKind::MissingOperand,
                ) else {
                    left_ir.quadruples.append(&mut right_ir.quadruples);
                    return IRResult {
                        operand: None,
                        quadruples: left_ir.quadruples,
                    };
                };

                let dest = generator.fresh_temp();

                let mut quadruples = std::mem::take(&mut left_ir.quadruples);
                quadruples.extend(std::mem::take(&mut right_ir.quadruples));
                quadruples.push(Quadruple::Binary {
                    op: *operator,
                    lhs,
                    rhs,
                    dest,
                });

                IRResult {
                    operand: Some(Operand::Temp(dest)),
                    quadruples,
                }
            }
            Expr::Assignment { name, value, info } => {
                let mut name_ir = name.generate_ir(generator);
                let mut value_ir = value.generate_ir(generator);

                let mut quadruples = std::mem::take(&mut name_ir.quadruples);
                quadruples.append(&mut value_ir.quadruples);

                let dest_temp = extract_temp(&mut name_ir, generator, &info.span);

                if let Some(src) = value_ir.operand.clone() {
                    quadruples.push(Quadruple::Copy {
                        src,
                        dest: dest_temp,
                    });
                }

                IRResult {
                    operand: Some(Operand::Temp(dest_temp)),
                    quadruples,
                }
            }
            Expr::Call {
                callee,
                arguments,
                info,
                ..
            } => {
                let mut callee_ir = callee.generate_ir(generator);
                let mut quadruples = std::mem::take(&mut callee_ir.quadruples);
                let Some(callee_operand) = expect_operand(
                    callee_ir.operand.clone(),
                    generator,
                    &info.span,
                    InternalErrorKind::MissingCallee,
                ) else {
                    return IRResult {
                        operand: None,
                        quadruples,
                    };
                };

                let mut arg_operands = Vec::new();
                for arg in arguments {
                    let mut arg_ir = arg.generate_ir(generator);
                    quadruples.extend(arg_ir.quadruples);
                    let Some(op) = expect_operand(
                        arg_ir.operand,
                        generator,
                        &arg.info().span,
                        InternalErrorKind::MissingArgument,
                    ) else {
                        return IRResult {
                            operand: None,
                            quadruples,
                        };
                    };
                    arg_operands.push(op);
                }

                let dest = generator.fresh_temp();
                quadruples.push(Quadruple::Call {
                    callee: callee_operand,
                    args: arg_operands,
                    dest: Some(dest),
                });

                IRResult {
                    operand: Some(Operand::Temp(dest)),
                    quadruples,
                }
            }
            Expr::MemberAccess {
                left,
                members,
                info,
            } => {
                let mut left_ir = left.generate_ir(generator);
                let mut quadruples = std::mem::take(&mut left_ir.quadruples);
                let Some(mut current) = expect_operand(
                    left_ir.operand,
                    generator,
                    &info.span,
                    InternalErrorKind::MissingMemberBase,
                ) else {
                    return IRResult {
                        operand: None,
                        quadruples,
                    };
                };

                for member in members.iter() {
                    let member_name = extract_member_name(member);

                    let dest = generator.fresh_temp();
                    quadruples.push(Quadruple::MemberAccess {
                        base: current,
                        member: member_name,
                        dest,
                    });

                    current = Operand::Temp(dest);
                }

                IRResult {
                    operand: Some(current),
                    quadruples,
                }
            }
        }
    }
}

impl GenerateIR for Primary {
    fn generate_ir(&self, generator: &mut IRGenerator) -> IRResult {
        match self {
            Primary::Literal(lit) => lit.generate_ir(generator),
            Primary::Primitive(prim) => prim.generate_ir(generator),
        }
    }
}

impl GenerateIR for AstLiteral {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        let literal = match self {
            AstLiteral::Boolean { value, .. } => Literal::Boolean(*value),
            AstLiteral::String { value, .. } => Literal::String(value.clone()),
            AstLiteral::Number { value, .. } => Literal::Number(*value),
            AstLiteral::None { .. } => Literal::None,
        };

        IRResult::with_operand(Operand::Literal(literal))
    }
}

impl GenerateIR for Primitive {
    fn generate_ir(&self, generator: &mut IRGenerator) -> IRResult {
        match self {
            Primitive::Identifier { name, .. } => {
                let temp = generator.temp_for_symbol(name.clone());
                IRResult::with_operand(Operand::Temp(temp))
            }
            Primitive::StoryIdentifier { name, .. } => {
                let temp = generator.temp_for_symbol(name.clone());
                IRResult::with_operand(Operand::Temp(temp))
            }
            Primitive::Context { .. } => IRResult::with_operand(Operand::Context),
            Primitive::Grouping { expr, .. } => expr.generate_ir(generator),
            Primitive::Object { value, .. } => value.generate_ir(generator),
            Primitive::Closure { params, body, .. } => {
                let entry_label = generator.fresh_label();
                let mut procedure = Procedure::new(None);

                let mut param_defs = Vec::new();
                for param in params {
                    let hint = match param {
                        Primitive::Identifier { name, .. }
                        | Primitive::StoryIdentifier { name, .. } => Some(name.clone()),
                        _ => None,
                    };

                    let param_def = generator.make_param(hint.clone());

                    if let Some(name) = hint {
                        generator.temp_for_symbol(name);
                    }

                    param_defs.push(param_def);
                }

                let mut entry_block = Block::with_name(entry_label, "closure_entry");
                let body_ir = body.generate_ir(generator);
                entry_block.add_quadruples(body_ir.quadruples);
                entry_block.set_terminator(Terminator::Return {
                    value: body_ir.operand,
                });

                procedure
                    .add_params(param_defs.clone())
                    .add_block(entry_block)
                    .set_entry(entry_label);

                generator.add_procedure(procedure);

                let dest = generator.fresh_temp();
                let quadruples = vec![Quadruple::MakeClosure {
                    target: entry_label,
                    params: param_defs,
                    dest,
                }];

                IRResult {
                    operand: Some(Operand::Temp(dest)),
                    quadruples,
                }
            }
        }
    }
}

fn extract_temp(result: &mut IRResult, generator: &mut IRGenerator, span: &Span) -> TempId {
    match result.operand {
        Some(Operand::Temp(id)) => id,
        Some(Operand::Literal(_)) | Some(Operand::Context) | None => {
            generator.push_error(Error::new(
                InternalErrorKind::InvalidAssignmentTarget,
                span.clone(),
            ));
            generator.fresh_temp()
        }
    }
}

fn expect_operand(
    operand: Option<Operand>,
    generator: &mut IRGenerator,
    span: &Span,
    err_kind: InternalErrorKind,
) -> Option<Operand> {
    if let Some(op) = operand {
        Some(op)
    } else {
        generator.push_error(Error::new(err_kind, span.clone()));
        None
    }
}

fn extract_member_name(expr: &Expr) -> String {
    match expr {
        Expr::Primary {
            value: Primary::Primitive(Primitive::Identifier { name, .. }),
            ..
        }
        | Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => name.clone(),
        _ => expr.info().id.to_string(),
    }
}
