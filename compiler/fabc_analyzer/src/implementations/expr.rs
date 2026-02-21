use fabc_error::{
    kind::{CompileErrorKind, InternalErrorKind},
    Error,
};
use fabc_parser::ast::expr::{literal::Literal, primitive::Primitive, Expr, Primary};

use crate::{
    types::{DataType, ModuleSymbolType, SymbolAnnotation},
    AnalysisResult, Analyzable,
};

impl Analyzable for Expr {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
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

                if left_sym_type != right_sym_type {
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
                        r#type: left_sym_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(left_sym_type),
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

                if name_sym_type != value_sym_type {
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
                        r#type: value_sym_type.clone(),
                        binding: None,
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(value_sym_type),
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

                            if arg_sym_type != parameters[i] {
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

                    if let ModuleSymbolType::Data(DataType::Record { .. }) = left_type {
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

                for member in members.iter() {
                    let Some(member_name_type) = member.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };

                    if let ModuleSymbolType::Data(DataType::Record { fields }) = &current_type {
                        if let Some(field) = fields
                            .iter()
                            .find(|f| f.name == member_name_type.to_string())
                        {
                            current_type = (*field.r#type).clone();
                        } else {
                            analyzer.push_error(Error::new(
                                CompileErrorKind::InvalidMemberAccess {
                                    member: member_name_type.to_string(),
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
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
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
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
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
                let ident_sym = {
                    let Some(ident_sym) = analyzer.mut_story_sym_table().lookup_symbol(name) else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::UninitializedVariable,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    ident_sym.clone()
                };

                analyzer.annotate_story_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: Some(name.clone()),
                        r#type: ident_sym.r#type.clone(),
                        binding: Some(crate::types::BindingDetails {
                            slot: ident_sym.slot,
                            depth: ident_sym.depth,
                            kind: crate::types::BindingKind::Local,
                        }),
                    },
                );

                AnalysisResult {
                    story_sym_type: Some(ident_sym.r#type),
                    ..Default::default()
                }
            }
            Primitive::Identifier { info, name } => {
                let ident_sym = {
                    let Some(ident_sym) = analyzer.mut_mod_sym_table().lookup_symbol(name) else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::UninitializedVariable,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    ident_sym.clone()
                };

                analyzer.annotate_mod_symbol(self.info().id, ident_sym.clone().into());

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
                    let Some(param_sym_type) = param.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        analyzer.mut_mod_sym_table().exit_scope();
                        return AnalysisResult::default();
                    };
                    param_types.push(param_sym_type.clone());

                    if let Primitive::Identifier { name, .. } = param {
                        if analyzer
                            .mut_mod_sym_table()
                            .assign_symbol(name, param_sym_type)
                            .is_none()
                        {
                            analyzer.push_error(Error::new(
                                InternalErrorKind::InvalidAssignment,
                                info.span.clone(),
                            ));
                            analyzer.mut_mod_sym_table().exit_scope();
                            return AnalysisResult::default();
                        }
                    }
                }

                let body_sym_type = {
                    let Some(sym_type) = body.analyze(analyzer).mod_sym_type else {
                        analyzer.push_error(Error::new(
                            CompileErrorKind::TypeInference,
                            info.span.clone(),
                        ));
                        analyzer.mut_mod_sym_table().exit_scope();
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
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
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        match self {
            Primary::Literal(lit) => lit.analyze(analyzer),
            Primary::Primitive(prim) => prim.analyze(analyzer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::{identifier_expr, info, number_expr, string_expr},
        Analyzer,
    };
    use fabc_error::kind::{CompileErrorKind, ErrorKind};
    use fabc_parser::ast::expr::BinaryOperator;

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
        let mut fields = std::collections::BTreeMap::new();
        fields.insert("foo".to_string(), number_expr(30, 2.0));
        let record = Primitive::Object {
            info: info(31),
            value: fabc_parser::ast::decl::object::ObjectDecl {
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
}
