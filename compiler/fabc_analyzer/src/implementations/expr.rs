#![allow(unused)]
use fabc_error::{kind::ErrorKind, Error};
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
                            ErrorKind::TypeInference,
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
                    },
                );

                result
            }
            Expr::Binary {
                info,
                left,
                operator,
                right,
            } => {
                let left_sym_type = {
                    let Some(sym_type) = left.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                let right_sym_type = {
                    let Some(sym_type) = right.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                if left_sym_type != right_sym_type {
                    analyzer.push_error(Error::new(
                        ErrorKind::ExpectedType {
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
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(left_sym_type),
                }
            }
            Expr::Assignment { info, name, value } => {
                let name_sym_type = {
                    let Some(sym_type) = value.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                let value_sym_type = {
                    let Some(sym_type) = value.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                if name_sym_type != value_sym_type {
                    analyzer.push_error(Error::new(
                        ErrorKind::ExpectedType {
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
                    },
                );

                AnalysisResult {
                    mod_sym_type: Some(value_sym_type),
                }
            }
            Expr::Call {
                info,
                callee,
                arguments,
            } => {
                let callee_sym_type = {
                    let Some(sym_type) = callee.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
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
                                ErrorKind::ArityMismatch {
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
                                        ErrorKind::TypeInference,
                                        info.span.clone(),
                                    ));
                                    return AnalysisResult::default();
                                };
                                sym_type.clone()
                            };

                            if arg_sym_type != parameters[i] {
                                analyzer.push_error(Error::new(
                                    ErrorKind::ExpectedType {
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
                            },
                        );

                        AnalysisResult {
                            mod_sym_type: Some(*return_type),
                        }
                    }
                    _ => {
                        analyzer.push_error(Error::new(ErrorKind::NotCallable, info.span.clone()));
                        AnalysisResult::default()
                    }
                }
            }
            Expr::Grouping { info, expression } => {
                let result = expression.analyze(analyzer);

                let group_sym_type = {
                    let Some(sym_type) = result.mod_sym_type.clone() else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: group_sym_type.clone(),
                    },
                );

                result
            }
            Expr::Unary {
                info,
                operator,
                right,
            } => {
                let result = right.analyze(analyzer);

                let unary_sym_type = {
                    let Some(sym_type) = result.mod_sym_type.clone() else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: unary_sym_type.clone(),
                    },
                );

                result
            }
            Expr::MemberAccess {
                info,
                left,
                members,
            } => {
                let left_type = {
                    let Some(left_type) = left.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };

                    if let ModuleSymbolType::Data(DataType::Record { .. }) = left_type {
                        left_type
                    } else {
                        analyzer.push_error(Error::new(
                            ErrorKind::ExpectedType {
                                expected: "Record".to_string(),
                                found: format!("{left_type}"),
                            },
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    }
                };

                let mut resolved_type = None;
                for (idx, member) in members.iter().enumerate() {
                    let is_last = idx == members.len() - 1;

                    if !is_last {
                        let Some(member_type) = member.analyze(analyzer).mod_sym_type else {
                            analyzer.push_error(Error::new(
                                ErrorKind::TypeInference,
                                info.span.clone(),
                            ));
                            break;
                        };

                        if let ModuleSymbolType::Data(DataType::Record { .. }) = member_type {
                            resolved_type = Some(member_type);
                        } else {
                            analyzer.push_error(Error::new(
                                ErrorKind::ExpectedType {
                                    expected: "Record".to_string(),
                                    found: format!("{member_type}"),
                                },
                                info.span.clone(),
                            ));
                        }
                    } else {
                        let Some(member_type) = member.analyze(analyzer).mod_sym_type else {
                            analyzer.push_error(Error::new(
                                ErrorKind::TypeInference,
                                info.span.clone(),
                            ));
                            break;
                        };
                        resolved_type = Some(member_type);
                    };
                }

                if let Some(resolved_type) = resolved_type {
                    analyzer.annotate_mod_symbol(
                        self.info().id,
                        SymbolAnnotation {
                            name: None,
                            r#type: resolved_type.clone(),
                        },
                    );

                    AnalysisResult {
                        mod_sym_type: Some(resolved_type),
                    }
                } else {
                    AnalysisResult::default()
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
            },
        );

        AnalysisResult {
            mod_sym_type: Some(ModuleSymbolType::Data(data_type)),
        }
    }
}

impl Analyzable for Primitive {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        let data_type = match self {
            Primitive::Object { info, value } => {
                let Some(obj_type) = value.analyze(analyzer).mod_sym_type else {
                    analyzer.push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                    return AnalysisResult::default();
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: obj_type.clone(),
                    },
                );

                obj_type
            }
            Primitive::Identifier { info, name } => {
                let ident_sym = {
                    let Some(ident_sym) = analyzer.mut_mod_sym_table().lookup_symbol(name) else {
                        analyzer.push_error(Error::new(
                            ErrorKind::UninitializedVariable,
                            info.span.clone(),
                        ));
                        return AnalysisResult::default();
                    };
                    ident_sym.clone()
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: Some(name.clone()),
                        r#type: ident_sym.r#type.clone(),
                    },
                );

                ident_sym.r#type
            }
            Primitive::Grouping { info, expr } => {
                let Some(group_type) = expr.analyze(analyzer).mod_sym_type else {
                    analyzer.push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                    return AnalysisResult::default();
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: group_type.clone(),
                    },
                );

                group_type
            }
            Primitive::Context { info } => {
                let context_type = ModuleSymbolType::Data(DataType::Context);

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: context_type.clone(),
                    },
                );

                context_type
            }
            Primitive::Closure { info, params, body } => {
                let mut param_types = Vec::new();
                for param in params {
                    let Some(param_sym_type) = param.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    param_types.push(param_sym_type.clone());
                }

                let body_sym_type = {
                    let Some(sym_type) = body.analyze(analyzer).mod_sym_type else {
                        analyzer
                            .push_error(Error::new(ErrorKind::TypeInference, info.span.clone()));
                        return AnalysisResult::default();
                    };
                    sym_type.clone()
                };

                analyzer.annotate_mod_symbol(
                    self.info().id,
                    SymbolAnnotation {
                        name: None,
                        r#type: ModuleSymbolType::Function {
                            return_type: Box::new(body_sym_type.clone()),
                            parameters: param_types.clone(),
                            arity: params.len(),
                        },
                    },
                );

                ModuleSymbolType::Function {
                    return_type: Box::new(body_sym_type),
                    parameters: param_types,
                    arity: params.len(),
                }
            }
        };

        AnalysisResult {
            mod_sym_type: Some(data_type),
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
