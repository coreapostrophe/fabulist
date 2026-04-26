use fabc_error::{
    kind::{CompileErrorKind, InternalErrorKind},
    Error,
};
use fabc_parser::ast::{
    expr::{literal::Literal, primitive::Primitive, Expr, Primary},
    stmt::{block::BlockStmt, Stmt as ParserStmt},
};

use crate::{
    types::{
        BindingDetails, BindingKind, DataType, ModuleSymbolType, StorySymbolType, SymbolAnnotation,
    },
    AnalysisResult, Analyzable, Analyzer,
};

fn analyze_block_in_current_scope(block: &BlockStmt, analyzer: &mut Analyzer) -> AnalysisResult {
    let mut return_type: Option<ModuleSymbolType> = None;

    for statement in &block.statements {
        match statement {
            ParserStmt::Return(return_statement) => {
                let analyzed_return = return_statement.analyze(analyzer);
                if let Some(ret_type) = analyzed_return.mod_sym_type {
                    return_type = Some(ret_type);
                }
            }
            _ => {
                statement.analyze(analyzer);
            }
        }
    }

    AnalysisResult {
        mod_sym_type: return_type,
        ..Default::default()
    }
}

fn is_unknown_mod_type(sym_type: &ModuleSymbolType) -> bool {
    matches!(sym_type, ModuleSymbolType::Data(DataType::Unknown))
}

fn mod_types_compatible(left: &ModuleSymbolType, right: &ModuleSymbolType) -> bool {
    left == right || is_unknown_mod_type(left) || is_unknown_mod_type(right)
}

fn merge_mod_types(left: &ModuleSymbolType, right: &ModuleSymbolType) -> ModuleSymbolType {
    match (is_unknown_mod_type(left), is_unknown_mod_type(right)) {
        (true, false) => right.clone(),
        _ => left.clone(),
    }
}

fn static_member_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Primary {
            value: Primary::Primitive(Primitive::Identifier { name, .. }),
            ..
        }
        | Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => Some(name.clone()),
        Expr::Primary {
            value: Primary::Literal(Literal::String { value, .. }),
            ..
        } => Some(value.clone()),
        _ => None,
    }
}

fn supports_dynamic_members(sym_type: &ModuleSymbolType) -> bool {
    matches!(
        sym_type,
        ModuleSymbolType::Data(DataType::Context | DataType::Unknown)
    )
}

impl Analyzable for Expr {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        match self {
            Expr::Primary { value, .. } => {
                let result = value.analyze(analyzer);

                let primary_type = {
                    let Some(sym_type) = result.mod_sym_type.clone() else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            self.info().span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: primary_type.clone(),
                        binding: None,
                    },
                );

                result
            }
            Expr::Binary {
                info, left, right, ..
            } => {
                let left_sym_type = {
                    let Some(sym_type) = left.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                let right_sym_type = {
                    let Some(sym_type) = right.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                if !mod_types_compatible(&left_sym_type, &right_sym_type) {
                    analyzer.push_error(Error::new(
                        CompileErrorKind::ExpectedType {
                            expected: left_sym_type.to_string(),
                            found: right_sym_type.to_string(),
                        },
                        right.info().span.clone(),
                    ));
                    return AnalysisResult::default();
                }

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: merge_mod_types(&left_sym_type, &right_sym_type),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(merge_mod_types(&left_sym_type, &right_sym_type)),
                    ..Default::default()
                }
            }
            Expr::Assignment { info, name, value } => {
                let name_sym_type = {
                    let Some(sym_type) = name.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                let value_sym_type = {
                    let Some(sym_type) = value.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                if !mod_types_compatible(&name_sym_type, &value_sym_type) {
                    analyzer.push_error(Error::new(
                        CompileErrorKind::ExpectedType {
                            expected: name_sym_type.to_string(),
                            found: value_sym_type.to_string(),
                        },
                        value.info().span.clone(),
                    ));
                    return AnalysisResult::default();
                }

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: merge_mod_types(&name_sym_type, &value_sym_type),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(merge_mod_types(&name_sym_type, &value_sym_type)),
                    ..Default::default()
                }
            }
            Expr::Call {
                info,
                callee,
                arguments,
            } => {
                let callee_sym_type = {
                    let Some(sym_type) = callee.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                match callee_sym_type {
                    ModuleSymbolType::Function {
                        return_type,
                        parameters,
                        arity,
                    } => {
                        if arguments.len() != arity {
                            analyzer.push_error(Error::new(
                                CompileErrorKind::ArityMismatch {
                                    expected: arity,
                                    found: arguments.len(),
                                },
                                info.span.clone(),
                            ));
                            return AnalysisResult::default();
                        }

                        for (i, argument) in arguments.iter().enumerate() {
                            let arg_sym_type = {
                                let Some(sym_type) = argument.analyze(analyzer).mod_sym_type else {
                                    analyzer.push_error(Error::new(
                                        CompileErrorKind::TypeInference,
                                        info.span.clone(),
                                    ));
                                    return AnalysisResult::default();
                                };
                                sym_type.clone()
                            };

                            if !mod_types_compatible(&arg_sym_type, &parameters[i]) {
                                analyzer.push_error(Error::new(
                                    CompileErrorKind::ExpectedType {
                                        expected: parameters[i].to_string(),
                                        found: arg_sym_type.to_string(),
                                    },
                                    argument.info().span.clone(),
                                ));
                                return AnalysisResult::default();
                            }
                        }

                        analyzer.annotate_mod_symbol(
                            self.info().id,
                            SymbolAnnotation {
                                name: None,
                                r#type: (*return_type).clone(),
                                binding: None,
                            },
                        );

                        AnalysisResult {
                            mod_sym_type: Some(*return_type),
                            ..Default::default()
                        }
                    }
                    _ => {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::NotCallable,
                            info.span.clone(),
                        ));
                        AnalysisResult::default()
                    }
                }
            }
            Expr::Grouping { info, expression } => {
                let result = expression.analyze(analyzer);

                let group_sym_type = {
                    let Some(sym_type) = result.mod_sym_type.clone() else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: group_sym_type.clone(),
                        binding: None,
                    },
                );

                result
            }
            Expr::Unary { info, right, .. } => {
                let result = right.analyze(analyzer);

                let unary_sym_type = {
                    let Some(sym_type) = result.mod_sym_type.clone() else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    sym_type
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: unary_sym_type.clone(),
                        binding: None,
                    },
                );

                result
            }
            Expr::MemberAccess {
                info,
                left,
                members,
            } => {
                let mut current_type = {
                    let Some(left_type) = left.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };

                    if supports_dynamic_members(&left_type) {
                        ModuleSymbolType::Data(DataType::Unknown)
                    } else if let ModuleSymbolType::Data(DataType::Record { .. }) = left_type {
                        left_type
                    } else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::ExpectedType {
                                expected: "Record".to_string(),
                                found: format!("{left_type}"),
                            },
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    }
                };

                if is_unknown_mod_type(&current_type) {
                    analyzer.annotate_mod_symbol(
                        self.info().id,
                        SymbolAnnotation {
                            name: None,
                            r#type: ModuleSymbolType::Data(DataType::Unknown),
                            binding: None,
                        },
                    );

                    return AnalysisResult {
                        mod_sym_type: Some(ModuleSymbolType::Data(DataType::Unknown)),
                        ..Default::default()
                    };
                }

                for member in members.iter() {
                    let member_name = if let Some(member_name) = static_member_name(member) {
                        member_name
                    } else {
                        let Some(member_name_type) = member.analyze(analyzer).mod_sym_type else {
                            analyzer.push_error(Error::new(
                                CompileErrorKind::TypeInference,
                                info.span.clone(),
                            ));
                            return AnalysisResult::default();
                        };
                        member_name_type.to_string()
                    };

                    if let ModuleSymbolType::Data(DataType::Record { fields }) = &current_type {
                        if let Some(field) = fields.iter().find(|f| f.name == member_name) {
                            current_type = (*field.r#type).clone();
                        } else {
                            analyzer.push_error(Error::new(
                                CompileErrorKind::InvalidMemberAccess {
                                    member: member_name,
                                },
                                info.span.clone(),
                            ));
                            return AnalysisResult::default();
                        }
                    } else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::ExpectedType {
                                expected: "Record".to_string(),
                                found: format!("{current_type}"),
                            },
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    }
                }

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: current_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(current_type),
                    ..Default::default()
                }
            }
        }
    }
}

impl Analyzable for Literal {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        let data_type = match self {
            Literal::Number { .. } => DataType::Number,
            Literal::String { .. } => DataType::String,
            Literal::Boolean { .. } => DataType::Boolean,
            Literal::None { .. } => DataType::None,
        };

        analyzer.annotate_mod_symbol(
            self.info().id,
            SymbolAnnotation {
                name: None,
                r#type: ModuleSymbolType::Data(data_type.clone()),
                binding: None,
            },
        );

        AnalysisResult {
            mod_sym_type: Some(ModuleSymbolType::Data(data_type)),
            ..Default::default()
        }
    }
}

impl Analyzable for Primitive {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        match self {
            Primitive::Object { info, value } => {
                let Some(obj_type) = value.analyze(analyzer).mod_sym_type else {
                    analyzer.push_error(Error::new(
                        CompileErrorKind::TypeInference,
                        info.span.clone(),
                    ));
                    return AnalysisResult::default();
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: obj_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(obj_type),
                    ..Default::default()
                }
            }
            Primitive::StoryIdentifier { info, name } => {
                let story_table = analyzer.mut_story_sym_table();
                let current_level = story_table.current_level();
                let ident_sym = {
                    let Some(ident_sym) = story_table.lookup_symbol(name) else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::UninitializedVariable,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    ident_sym.clone()
                };

                let binding_kind = if ident_sym.depth == 0 {
                    BindingKind::Global
                } else if ident_sym.depth == current_level {
                    BindingKind::Local
                } else {
                    BindingKind::Upvalue
                };
                let distance = current_level.saturating_sub(ident_sym.depth);
                let mod_sym_type = matches!(ident_sym.r#type, StorySymbolType::Part)
                    .then_some(ModuleSymbolType::Data(DataType::String));

                analyzer.annotate_story_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: Some(name.clone()),
                        r#type: ident_sym.r#type.clone(),
                        binding: Some(BindingDetails {
                            slot: ident_sym.slot,
                            depth: ident_sym.depth,
                            distance,
                            kind: binding_kind,
                        }),
                    },
                );

                if let Some(mod_sym_type) = mod_sym_type.clone() {
                    analyzer.annotate_mod_symbol(
                        self.info().id,
                        SymbolAnnotation {
                            name: Some(name.clone()),
                            r#type: mod_sym_type,
                            binding: None,
                        },
                    );
                }

                AnalysisResult {
                    mod_sym_type,
                    story_sym_type: Some(ident_sym.r#type),
                }
            }
            Primitive::Identifier { info, name } => {
                let mod_table = analyzer.mut_mod_sym_table();
                let current_level = mod_table.current_level();
                let ident_sym = {
                    let Some(ident_sym) = mod_table.lookup_symbol(name) else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::UninitializedVariable,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    ident_sym.clone()
                };

                let binding_kind = if ident_sym.depth == 0 {
                    BindingKind::Global
                } else if ident_sym.depth == current_level {
                    BindingKind::Local
                } else {
                    BindingKind::Upvalue
                };
                let distance = current_level.saturating_sub(ident_sym.depth);

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: Some(ident_sym.name.clone()),
                        r#type: ident_sym.r#type.clone(),
                        binding: Some(BindingDetails {
                            slot: ident_sym.slot,
                            depth: ident_sym.depth,
                            distance,
                            kind: binding_kind,
                        }),
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(ident_sym.r#type),
                    ..Default::default()
                }
            }
            Primitive::Grouping { info, expr } => {
                let Some(group_type) = expr.analyze(analyzer).mod_sym_type else {
                    analyzer.push_error(Error::new(
                        CompileErrorKind::TypeInference,
                        info.span.clone(),
                    ));
                    return AnalysisResult::default();
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: group_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(group_type),
                    ..Default::default()
                }
            }
            Primitive::Context { .. } => {
                let context_type = ModuleSymbolType::Data(DataType::Context);

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: context_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(context_type),
                    ..Default::default()
                }
            }
            Primitive::Closure { info, params, body } => {
                analyzer.mut_mod_sym_table().enter_scope();

                let mut param_types = Vec::new();
                for param in params {
                    let Primitive::Identifier {
                        info: param_info,
                        name,
                    } = param
                    else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        analyzer.mut_mod_sym_table().exit_scope();
                        return AnalysisResult::default();
                    };

                    let param_sym_type = ModuleSymbolType::Data(DataType::Unknown);
                    let symbol = {
                        let Some(symbol) = analyzer
                            .mut_mod_sym_table()
                            .assign_symbol(name, param_sym_type.clone())
                        else {
                            analyzer.push_error(Error::new(
                                InternalErrorKind::InvalidAssignment,
                                info.span.clone(),
                            ));
                            analyzer.mut_mod_sym_table().exit_scope();
                            return AnalysisResult::default();
                        };
                        symbol.clone()
                    };

                    analyzer.annotate_mod_symbol(param_info.id, symbol.into());
                    param_types.push(param_sym_type);
                }

                let previous_error_count = analyzer.errors.len();
                let body_sym_type =
                    match analyze_block_in_current_scope(body, analyzer).mod_sym_type {
                        Some(sym_type) => sym_type,
                        None if analyzer.errors.len() == previous_error_count => {
                            ModuleSymbolType::Data(DataType::None)
                        }
                        None => {
                            analyzer.mut_mod_sym_table().exit_scope();
                            return AnalysisResult::default();
                        }
                    };

                analyzer.mut_mod_sym_table().exit_scope();

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: ModuleSymbolType::Function {
                            return_type: Box::new(body_sym_type.clone()),
                            parameters: param_types.clone(),
                            arity: params.len(),
                        },
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(ModuleSymbolType::Function {
                        return_type: Box::new(body_sym_type),
                        parameters: param_types,
                        arity: params.len(),
                    }),
                    ..Default::default()
                }
            }
        }
    }
}

impl Analyzable for Primary {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        match self {
            Primary::Literal(lit) => lit.analyze(analyzer),
            Primary::Primitive(prim) => prim.analyze(analyzer),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::test_utils::{identifier_expr, info, number_expr, string_expr};
    use crate::types::BindingKind;
    use fabc_error::kind::{CompileErrorKind, ErrorKind};
    use fabc_parser::ast::{
        decl::object::ObjectDecl,
        expr::BinaryOperator,
        stmt::{block::BlockStmt, expr::ExprStmt, r#let::LetStmt, r#return::ReturnStmt, Stmt},
    };

    #[test]
    fn binary_mismatch_reports_error() {
        let expr = Expr::Binary {
            info: info(20),
            left: Box::new(number_expr(21, 1.0)),
            operator: BinaryOperator::Add,
            right: Box::new(string_expr(22, "oops")),
        };

        let analyzer = Analyzer::analyze_ast(&expr).expect("analyze failed");

        assert!(analyzer.errors.iter().any(|e| matches!(
            e.kind,
            ErrorKind::Compile(CompileErrorKind::ExpectedType { .. })
        )));
    }

    #[test]
    fn invalid_member_access_is_reported() {
        let mut fields = BTreeMap::new();
        fields.insert("foo".to_string(), number_expr(30, 2.0));
        let record = Primitive::Object {
            info: info(31),
            value: ObjectDecl {
                info: info(32),
                map: fields,
            },
        };

        let access = Expr::MemberAccess {
            info: info(40),
            left: Box::new(Expr::Primary {
                info: info(33),
                value: Primary::Primitive(record),
            }),
            members: vec![identifier_expr(34, "bar")],
        };

        let mut analyzer = Analyzer::default();
        analyzer.mut_mod_sym_table().assign_symbol(
            "bar",
            ModuleSymbolType::Module {
                name: "bar".to_string(),
            },
        );

        access.analyze(&mut analyzer);

        assert!(analyzer.errors.iter().any(|e| matches!(
            e.kind,
            ErrorKind::Compile(CompileErrorKind::InvalidMemberAccess { .. })
        )));
    }

    #[test]
    fn identifier_inside_closure_marks_upvalue_binding() {
        let mut analyzer = Analyzer::default();

        let outer_let = Stmt::Let(LetStmt {
            info: info(200),
            name: "x".to_string(),
            initializer: number_expr(201, 1.0),
        });

        let captured_ident_expr = identifier_expr(210, "x");
        let captured_node_id = 1210;

        let closure_body = BlockStmt {
            info: info(211),
            first_return: None,
            statements: vec![Stmt::Expr(ExprStmt {
                info: info(212),
                expr: captured_ident_expr,
            })],
        };

        let closure_stmt = Stmt::Expr(ExprStmt {
            info: info(213),
            expr: Expr::Primary {
                info: info(214),
                value: Primary::Primitive(Primitive::Closure {
                    info: info(215),
                    params: Vec::new(),
                    body: closure_body,
                }),
            },
        });

        let block = BlockStmt {
            info: info(216),
            first_return: None,
            statements: vec![outer_let, closure_stmt],
        };

        block.analyze(&mut analyzer);

        let annotation = analyzer
            .mod_sym_annotations
            .get(&captured_node_id)
            .expect("identifier annotation missing");
        let binding = annotation.binding.as_ref().expect("binding missing");

        assert_eq!(binding.kind, BindingKind::Upvalue);
        assert_eq!(binding.distance, 1);
        assert_eq!(binding.slot, 0);
    }

    #[test]
    fn closure_parameters_bind_as_local_unknowns() {
        let mut analyzer = Analyzer::default();

        let closure = Expr::Primary {
            info: info(300),
            value: Primary::Primitive(Primitive::Closure {
                info: info(301),
                params: vec![
                    Primitive::Identifier {
                        info: info(302),
                        name: "x".to_string(),
                    },
                    Primitive::Identifier {
                        info: info(303),
                        name: "y".to_string(),
                    },
                ],
                body: BlockStmt {
                    info: info(304),
                    first_return: None,
                    statements: vec![Stmt::Expr(ExprStmt {
                        info: info(305),
                        expr: Expr::Binary {
                            info: info(306),
                            left: Box::new(identifier_expr(307, "x")),
                            operator: BinaryOperator::Add,
                            right: Box::new(identifier_expr(308, "y")),
                        },
                    })],
                },
            }),
        };

        closure.analyze(&mut analyzer);

        assert!(analyzer.errors.is_empty());

        for node_id in [1307, 1308] {
            let annotation = analyzer
                .mod_sym_annotations
                .get(&node_id)
                .expect("identifier annotation missing");
            let binding = annotation.binding.as_ref().expect("binding missing");

            assert_eq!(binding.kind, BindingKind::Local);
            assert_eq!(binding.distance, 0);
        }
    }

    #[test]
    fn closure_without_explicit_return_defaults_to_none() {
        let mut analyzer = Analyzer::default();

        let closure = Expr::Primary {
            info: info(400),
            value: Primary::Primitive(Primitive::Closure {
                info: info(401),
                params: Vec::new(),
                body: BlockStmt {
                    info: info(402),
                    first_return: None,
                    statements: vec![Stmt::Expr(ExprStmt {
                        info: info(403),
                        expr: number_expr(404, 1.0),
                    })],
                },
            }),
        };

        let result = closure.analyze(&mut analyzer);

        assert!(analyzer.errors.is_empty());
        assert_eq!(
            result.mod_sym_type,
            Some(ModuleSymbolType::Function {
                return_type: Box::new(ModuleSymbolType::Data(DataType::None)),
                parameters: Vec::new(),
                arity: 0,
            })
        );
    }

    #[test]
    fn call_accepts_known_arguments_for_unknown_params() {
        let mut analyzer = Analyzer::default();

        let call = Expr::Call {
            info: info(500),
            callee: Box::new(Expr::Primary {
                info: info(501),
                value: Primary::Primitive(Primitive::Closure {
                    info: info(502),
                    params: vec![Primitive::Identifier {
                        info: info(503),
                        name: "x".to_string(),
                    }],
                    body: BlockStmt {
                        info: info(504),
                        first_return: Some(0),
                        statements: vec![Stmt::Return(ReturnStmt {
                            info: info(505),
                            value: Some(Expr::Binary {
                                info: info(506),
                                left: Box::new(identifier_expr(507, "x")),
                                operator: BinaryOperator::Add,
                                right: Box::new(number_expr(508, 1.0)),
                            }),
                        })],
                    },
                }),
            }),
            arguments: vec![number_expr(509, 5.0)],
        };

        let result = call.analyze(&mut analyzer);

        assert!(analyzer.errors.is_empty());
        assert_eq!(
            result.mod_sym_type,
            Some(ModuleSymbolType::Data(DataType::Number))
        );
    }
}
