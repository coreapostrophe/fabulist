use std::collections::BTreeMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue},
    IntPredicate,
};

use crate::{
    error::{Error, Result},
    ir::{
        BinaryOperator, Block, Expr, FunctionId, FunctionSpec, Literal, MemberSegment, Stmt,
        StoryProgram, UnaryOperator,
    },
};

use super::abi::{RuntimeAbi, OUTCOME_KIND_GOTO};

pub struct LlvmArtifact<'ctx> {
    pub module: Module<'ctx>,
    pub function_symbols: BTreeMap<FunctionId, String>,
}

pub struct LlvmEmitter<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    abi: RuntimeAbi<'ctx>,
    function_symbols: BTreeMap<FunctionId, String>,
    string_counter: usize,
}

impl<'ctx> LlvmEmitter<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let abi = RuntimeAbi::declare(context, &module);

        Ok(Self {
            context,
            module,
            builder,
            abi,
            function_symbols: BTreeMap::new(),
            string_counter: 0,
        })
    }

    pub fn emit(mut self, program: &StoryProgram) -> Result<LlvmArtifact<'ctx>> {
        for function in &program.functions {
            self.emit_function(function)?;
        }

        self.emit_start_function(program)?;

        Ok(LlvmArtifact {
            module: self.module,
            function_symbols: self.function_symbols,
        })
    }

    fn emit_start_function(&mut self, program: &StoryProgram) -> Result<()> {
        let function = self.module.add_function(
            "fabc_story_start",
            self.abi.value_ptr_type.fn_type(&[], false),
            None,
        );
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        let start_value = self.emit_story_ref(&program.start_part, "story.start")?;
        self.builder
            .build_return(Some(&start_value))
            .map_err(|err| Error::Codegen(err.to_string()))?;

        Ok(())
    }

    fn emit_function(&mut self, function: &FunctionSpec) -> Result<()> {
        let symbol = format!("fabc_fn_{}", function.id);
        let fn_type = self.abi.outcome_ptr_type.fn_type(
            &[
                self.abi.value_ptr_type.into(),
                self.abi.value_ptr_type.into(),
            ],
            false,
        );
        let llvm_fn = self.module.add_function(&symbol, fn_type, None);
        self.function_symbols.insert(function.id, symbol);

        let entry = self.context.append_basic_block(llvm_fn, "entry");
        self.builder.position_at_end(entry);

        let frame = llvm_fn
            .get_nth_param(0)
            .ok_or_else(|| Error::Codegen("missing frame parameter".to_string()))?
            .into_pointer_value();
        let context = llvm_fn
            .get_nth_param(1)
            .ok_or_else(|| Error::Codegen("missing context parameter".to_string()))?
            .into_pointer_value();

        self.emit_block(&function.body, frame, context, llvm_fn)?;

        if self.current_block_has_terminator() {
            return Ok(());
        }

        let continue_outcome =
            self.call_value(self.abi.outcome_continue, &[], "outcome.continue")?;
        self.builder
            .build_return(Some(&continue_outcome))
            .map_err(|err| Error::Codegen(err.to_string()))?;

        Ok(())
    }

    fn emit_block(
        &mut self,
        block: &Block,
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
        llvm_fn: FunctionValue<'ctx>,
    ) -> Result<()> {
        for statement in &block.statements {
            self.emit_stmt(statement, frame, context, llvm_fn)?;
            if self.current_block_has_terminator() {
                break;
            }
        }

        Ok(())
    }

    fn emit_stmt(
        &mut self,
        statement: &Stmt,
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
        llvm_fn: FunctionValue<'ctx>,
    ) -> Result<()> {
        match statement {
            Stmt::Expr(expr) => {
                let _ = self.emit_expr(expr, frame, context)?;
            }
            Stmt::Block(block) => {
                let child = self.call_value(self.abi.env_child, &[frame.into()], "env.child")?;
                self.emit_block(block, child, context, llvm_fn)?;
            }
            Stmt::Let { name, initializer } => {
                let value = self.emit_expr(initializer, frame, context)?;
                let (name_ptr, name_len) = self.build_string_constant(name, "let.name")?;
                self.builder
                    .build_call(
                        self.abi.env_define,
                        &[frame.into(), name_ptr.into(), name_len.into(), value.into()],
                        "env.define",
                    )
                    .map_err(|err| Error::Codegen(err.to_string()))?;
            }
            Stmt::Goto(target) => {
                let target = self.emit_expr(target, frame, context)?;
                let outcome =
                    self.call_value(self.abi.outcome_goto, &[target.into()], "outcome.goto")?;
                self.builder
                    .build_return(Some(&outcome))
                    .map_err(|err| Error::Codegen(err.to_string()))?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.emit_expr(condition, frame, context)?;
                let condition_bool = self
                    .builder
                    .build_call(self.abi.is_truthy, &[condition_value.into()], "cond.bool")
                    .map_err(|err| Error::Codegen(err.to_string()))?
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Error::Codegen("truthiness helper did not return a value".to_string())
                    })?
                    .into_int_value();

                let then_block = self.context.append_basic_block(llvm_fn, "if.then");
                let else_block = self.context.append_basic_block(llvm_fn, "if.else");
                let cont_block = self.context.append_basic_block(llvm_fn, "if.cont");

                self.builder
                    .build_conditional_branch(condition_bool, then_block, else_block)
                    .map_err(|err| Error::Codegen(err.to_string()))?;

                self.builder.position_at_end(then_block);
                let then_frame =
                    self.call_value(self.abi.env_child, &[frame.into()], "if.then.frame")?;
                self.emit_block(then_branch, then_frame, context, llvm_fn)?;
                if !self.current_block_has_terminator() {
                    self.builder
                        .build_unconditional_branch(cont_block)
                        .map_err(|err| Error::Codegen(err.to_string()))?;
                }

                self.builder.position_at_end(else_block);
                if let Some(else_branch) = else_branch {
                    self.emit_stmt(else_branch, frame, context, llvm_fn)?;
                }
                if !self.current_block_has_terminator() {
                    self.builder
                        .build_unconditional_branch(cont_block)
                        .map_err(|err| Error::Codegen(err.to_string()))?;
                }

                self.builder.position_at_end(cont_block);
            }
            Stmt::Return(value) => {
                let return_value = match value {
                    Some(expr) => self.emit_expr(expr, frame, context)?,
                    None => self.call_value(self.abi.value_none, &[], "none")?,
                };
                let outcome = self.call_value(
                    self.abi.outcome_return,
                    &[return_value.into()],
                    "outcome.return",
                )?;
                self.builder
                    .build_return(Some(&outcome))
                    .map_err(|err| Error::Codegen(err.to_string()))?;
            }
        }

        Ok(())
    }

    fn emit_expr(
        &mut self,
        expr: &Expr,
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>> {
        match expr {
            Expr::Literal(literal) => self.emit_literal(literal),
            Expr::Identifier(name) => {
                let (name_ptr, name_len) = self.build_string_constant(name, "ident")?;
                self.call_value(
                    self.abi.env_load,
                    &[frame.into(), name_ptr.into(), name_len.into()],
                    "env.load",
                )
            }
            Expr::StoryReference(name) => self.emit_story_ref(name, "story.ref"),
            Expr::Context => {
                self.call_value(self.abi.context_value, &[context.into()], "context.value")
            }
            Expr::Object(properties) => {
                let object = self.call_value(self.abi.object_new, &[], "object.new")?;
                for (key, value) in properties {
                    let (key_ptr, key_len) = self.build_string_constant(key, "object.key")?;
                    let value = self.emit_expr(value, frame, context)?;
                    self.builder
                        .build_call(
                            self.abi.object_insert,
                            &[object.into(), key_ptr.into(), key_len.into(), value.into()],
                            "object.insert",
                        )
                        .map_err(|err| Error::Codegen(err.to_string()))?;
                }
                Ok(object)
            }
            Expr::Closure(function_id) => {
                let symbol = format!("fabc_fn_{function_id}");
                let (symbol_ptr, symbol_len) =
                    self.build_string_constant(&symbol, "closure.symbol")?;
                self.call_value(
                    self.abi.closure_new,
                    &[symbol_ptr.into(), symbol_len.into(), frame.into()],
                    "closure.new",
                )
            }
            Expr::Call { callee, arguments } => {
                let callee = self.emit_expr(callee, frame, context)?;
                let mut argument_values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    argument_values.push(self.emit_expr(argument, frame, context)?);
                }

                let args_ptr = self.build_argument_buffer(&argument_values)?;
                let args_len = self
                    .abi
                    .size_type
                    .const_int(argument_values.len() as u64, false);

                let outcome = self.call_outcome(
                    self.abi.call,
                    &[
                        frame.into(),
                        context.into(),
                        callee.into(),
                        args_ptr.into(),
                        args_len.into(),
                    ],
                    "call",
                )?;

                self.unwrap_call_outcome(outcome, "call")
            }
            Expr::MemberAccess { base, members } => {
                let mut current = self.emit_expr(base, frame, context)?;
                for member in members {
                    let key = self.emit_member_key(member, frame, context)?;
                    current = self.call_value(
                        self.abi.member_get,
                        &[current.into(), key.into()],
                        "member.get",
                    )?;
                }
                Ok(current)
            }
            Expr::Assignment { target, value } => {
                let value = self.emit_expr(value, frame, context)?;
                self.emit_assignment(target, value, frame, context)?;
                self.call_value(self.abi.value_none, &[], "none")
            }
            Expr::Unary { operator, right } => {
                let right = self.emit_expr(right, frame, context)?;
                let helper = match operator {
                    UnaryOperator::Not => self.abi.unary_not,
                    UnaryOperator::Negate => self.abi.unary_negate,
                };
                self.call_value(helper, &[right.into()], "unary")
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.emit_expr(left, frame, context)?;
                let right = self.emit_expr(right, frame, context)?;
                let helper = match operator {
                    BinaryOperator::Add => self.abi.binary_add,
                    BinaryOperator::Subtract => self.abi.binary_subtract,
                    BinaryOperator::Multiply => self.abi.binary_multiply,
                    BinaryOperator::Divide => self.abi.binary_divide,
                    BinaryOperator::EqualEqual => self.abi.binary_equal,
                    BinaryOperator::NotEqual => self.abi.binary_not_equal,
                    BinaryOperator::Greater => self.abi.binary_greater,
                    BinaryOperator::GreaterEqual => self.abi.binary_greater_equal,
                    BinaryOperator::Less => self.abi.binary_less,
                    BinaryOperator::LessEqual => self.abi.binary_less_equal,
                    BinaryOperator::And => self.abi.binary_and,
                    BinaryOperator::Or => self.abi.binary_or,
                };
                self.call_value(helper, &[left.into(), right.into()], "binary")
            }
            Expr::Grouping(inner) => self.emit_expr(inner, frame, context),
        }
    }

    fn emit_assignment(
        &mut self,
        target: &Expr,
        value: PointerValue<'ctx>,
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
    ) -> Result<()> {
        match target {
            Expr::Identifier(name) => {
                let (name_ptr, name_len) = self.build_string_constant(name, "assign.ident")?;
                self.builder
                    .build_call(
                        self.abi.env_assign,
                        &[frame.into(), name_ptr.into(), name_len.into(), value.into()],
                        "env.assign",
                    )
                    .map_err(|err| Error::Codegen(err.to_string()))?;
                Ok(())
            }
            Expr::MemberAccess { base, members } => {
                let (container, key) = self.emit_member_place(base, members, frame, context)?;
                self.builder
                    .build_call(
                        self.abi.member_assign,
                        &[container.into(), key.into(), value.into()],
                        "member.assign",
                    )
                    .map_err(|err| Error::Codegen(err.to_string()))?;
                Ok(())
            }
            _ => Err(Error::Codegen(
                "unsupported assignment target in LLVM emitter".to_string(),
            )),
        }
    }

    fn emit_member_place(
        &mut self,
        base: &Expr,
        members: &[MemberSegment],
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
    ) -> Result<(PointerValue<'ctx>, PointerValue<'ctx>)> {
        let mut current = self.emit_expr(base, frame, context)?;
        let Some((last, rest)) = members.split_last() else {
            return Err(Error::Codegen(
                "member assignment requires at least one member".to_string(),
            ));
        };

        for member in rest {
            let key = self.emit_member_key(member, frame, context)?;
            current = self.call_value(
                self.abi.member_get,
                &[current.into(), key.into()],
                "member.place",
            )?;
        }

        let key = self.emit_member_key(last, frame, context)?;
        Ok((current, key))
    }

    fn emit_member_key(
        &mut self,
        member: &MemberSegment,
        frame: PointerValue<'ctx>,
        context: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>> {
        match member {
            MemberSegment::Key(key) => self.emit_string_value(key, "member.key"),
            MemberSegment::Expr(expr) => self.emit_expr(expr, frame, context),
        }
    }

    fn emit_literal(&mut self, literal: &Literal) -> Result<PointerValue<'ctx>> {
        match literal {
            Literal::Number(value) => self.call_value(
                self.abi.value_number,
                &[self.abi.f64_type.const_float(*value).into()],
                "number",
            ),
            Literal::Boolean(value) => self.call_value(
                self.abi.value_boolean,
                &[self
                    .abi
                    .bool_type
                    .const_int(u64::from(*value), false)
                    .into()],
                "boolean",
            ),
            Literal::String(value) => self.emit_string_value(value, "string"),
            Literal::None => self.call_value(self.abi.value_none, &[], "none"),
        }
    }

    fn emit_string_value(&mut self, value: &str, label: &str) -> Result<PointerValue<'ctx>> {
        let (ptr, len) = self.build_string_constant(value, label)?;
        self.call_value(
            self.abi.value_string,
            &[ptr.into(), len.into()],
            "string.value",
        )
    }

    fn emit_story_ref(&mut self, value: &str, label: &str) -> Result<PointerValue<'ctx>> {
        let (ptr, len) = self.build_string_constant(value, label)?;
        self.call_value(
            self.abi.value_story_ref,
            &[ptr.into(), len.into()],
            "story.value",
        )
    }

    fn build_argument_buffer(
        &mut self,
        arguments: &[PointerValue<'ctx>],
    ) -> Result<PointerValue<'ctx>> {
        if arguments.is_empty() {
            return Ok(self.abi.value_ptr_ptr_type.const_null());
        }

        let len = self.abi.size_type.const_int(arguments.len() as u64, false);
        let buffer = self
            .builder
            .build_array_alloca(self.abi.value_ptr_type, len, "call.args")
            .map_err(|err| Error::Codegen(err.to_string()))?;

        for (index, value) in arguments.iter().enumerate() {
            let offset = self.context.i32_type().const_int(index as u64, false);
            // SAFETY: `buffer` was allocated with exactly `arguments.len()` elements, and the
            // loop index comes from enumerating that same slice, so this GEP stays in bounds.
            let slot = unsafe {
                self.builder.build_gep(
                    self.abi.value_ptr_type,
                    buffer,
                    &[offset],
                    &format!("call.arg.{index}"),
                )
            }
            .map_err(|err| Error::Codegen(err.to_string()))?;
            self.builder
                .build_store(slot, *value)
                .map_err(|err| Error::Codegen(err.to_string()))?;
        }

        Ok(buffer)
    }

    fn build_string_constant(
        &mut self,
        value: &str,
        label: &str,
    ) -> Result<(PointerValue<'ctx>, inkwell::values::IntValue<'ctx>)> {
        let global = self
            .builder
            .build_global_string_ptr(value, &format!("{label}.{}", self.string_counter))
            .map_err(|err| Error::Codegen(err.to_string()))?;
        self.string_counter += 1;

        Ok((
            global.as_pointer_value(),
            self.abi.size_type.const_int(value.len() as u64, false),
        ))
    }

    fn call_value(
        &mut self,
        function: FunctionValue<'ctx>,
        arguments: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<PointerValue<'ctx>> {
        let call = self
            .builder
            .build_call(function, arguments, name)
            .map_err(|err| Error::Codegen(err.to_string()))?;

        let value = call
            .try_as_basic_value()
            .left()
            .ok_or_else(|| Error::Codegen(format!("call `{name}` did not produce a value")))?;

        match value {
            BasicValueEnum::PointerValue(value) => Ok(value),
            other => Err(Error::Codegen(format!(
                "call `{name}` produced a non-pointer value: {other:?}"
            ))),
        }
    }

    fn call_outcome(
        &mut self,
        function: FunctionValue<'ctx>,
        arguments: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<PointerValue<'ctx>> {
        let call = self
            .builder
            .build_call(function, arguments, name)
            .map_err(|err| Error::Codegen(err.to_string()))?;

        let value = call
            .try_as_basic_value()
            .left()
            .ok_or_else(|| Error::Codegen(format!("call `{name}` did not produce a value")))?;

        match value {
            BasicValueEnum::PointerValue(value) => Ok(value),
            other => Err(Error::Codegen(format!(
                "call `{name}` produced a non-pointer value: {other:?}"
            ))),
        }
    }

    fn unwrap_call_outcome(
        &mut self,
        outcome: PointerValue<'ctx>,
        label: &str,
    ) -> Result<PointerValue<'ctx>> {
        let kind = self
            .builder
            .build_call(
                self.abi.outcome_kind,
                &[outcome.into()],
                &format!("{label}.kind"),
            )
            .map_err(|err| Error::Codegen(err.to_string()))?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| Error::Codegen(format!("call `{label}.kind` did not produce a value")))?
            .into_int_value();

        let llvm_fn = self
            .builder
            .get_insert_block()
            .and_then(|block| block.get_parent())
            .ok_or_else(|| Error::Codegen("cannot resolve current LLVM function".to_string()))?;
        let goto_block = self
            .context
            .append_basic_block(llvm_fn, &format!("{label}.goto"));
        let continue_block = self
            .context
            .append_basic_block(llvm_fn, &format!("{label}.cont"));
        let is_goto = self
            .builder
            .build_int_compare(
                IntPredicate::EQ,
                kind,
                self.abi
                    .outcome_kind_type
                    .const_int(OUTCOME_KIND_GOTO, false),
                &format!("{label}.is_goto"),
            )
            .map_err(|err| Error::Codegen(err.to_string()))?;

        self.builder
            .build_conditional_branch(is_goto, goto_block, continue_block)
            .map_err(|err| Error::Codegen(err.to_string()))?;

        self.builder.position_at_end(goto_block);
        self.builder
            .build_return(Some(&outcome))
            .map_err(|err| Error::Codegen(err.to_string()))?;

        self.builder.position_at_end(continue_block);
        self.call_value(
            self.abi.outcome_into_value,
            &[outcome.into()],
            &format!("{label}.value"),
        )
    }

    fn current_block_has_terminator(&self) -> bool {
        self.builder
            .get_insert_block()
            .and_then(|block| block.get_terminator())
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use inkwell::context::Context;

    use super::LlvmEmitter;
    use crate::ir::{
        BinaryOperator, Block, Expr, FunctionSpec, Literal, MemberSegment, PartSpec, Stmt,
        StoryProgram,
    };

    #[test]
    fn emitter_generates_story_start_function_and_runtime_calls() {
        let context = Context::create();
        let artifact = LlvmEmitter::new(&context, "emitter_test")
            .expect("create emitter")
            .emit(&program_with_addition_and_goto())
            .expect("emit llvm artifact");

        let ir = artifact.module.print_to_string().to_string();
        assert!(ir.contains("fabc_story_start"));
        assert!(ir.contains("fabc_fn_0"));
        assert!(ir.contains("fabc_rt_binary_add"));
        assert_eq!(
            artifact.function_symbols.get(&0).map(String::as_str),
            Some("fabc_fn_0")
        );
    }

    fn program_with_addition_and_goto() -> StoryProgram {
        StoryProgram {
            start_part: "part_1".to_string(),
            metadata: BTreeMap::new(),
            parts: vec![
                PartSpec {
                    id: "part_1".to_string(),
                    steps: Vec::new(),
                },
                PartSpec {
                    id: "part_2".to_string(),
                    steps: Vec::new(),
                },
            ],
            functions: vec![FunctionSpec {
                id: 0,
                node_id: 0,
                params: Vec::new(),
                body: Block {
                    statements: vec![
                        Stmt::Expr(Expr::Assignment {
                            target: Box::new(Expr::MemberAccess {
                                base: Box::new(Expr::Context),
                                members: vec![MemberSegment::Key("total".to_string())],
                            }),
                            value: Box::new(Expr::Binary {
                                left: Box::new(Expr::Literal(Literal::Number(10.0))),
                                operator: BinaryOperator::Add,
                                right: Box::new(Expr::Literal(Literal::Number(20.0))),
                            }),
                        }),
                        Stmt::Goto(Expr::StoryReference("part_2".to_string())),
                    ],
                },
            }],
        }
    }
}
