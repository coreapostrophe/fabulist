use std::collections::BTreeMap;

use crate::ir::{
    BinaryOperator, Block, Expr, Literal, MemberSegment, QuoteSpec, SelectionSpec, StepSpec, Stmt,
    StoryProgram, UnaryOperator,
};

use super::{
    error::{Result, RuntimeError},
    scope::Scope,
    value::{ClosureValue, ObjectRef, Value},
};

#[cfg(feature = "llvm-backend")]
use std::rc::Rc;

#[cfg(feature = "llvm-backend")]
use super::native::NativeClosureHost;

#[derive(Debug, Clone, PartialEq)]
pub struct NarrationView {
    pub text: String,
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogueView {
    pub speaker: String,
    pub text: String,
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceView {
    pub text: String,
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionView {
    pub choices: Vec<ChoiceView>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoryEvent {
    Narration(NarrationView),
    Dialogue(DialogueView),
    Selection(SelectionView),
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cursor {
    part_index: usize,
    step_index: usize,
}

#[derive(Debug, Clone)]
struct InvocationResult {
    value: Value,
    goto: Option<String>,
}

#[derive(Debug, Clone)]
enum EvalSignal {
    Value(Value),
    Goto(String),
}

#[derive(Debug, Clone)]
enum ExecSignal {
    Continue,
    Return(Value),
    Goto(String),
}

#[derive(Debug, Clone)]
pub struct StoryMachine {
    program: StoryProgram,
    globals: Scope,
    context: ObjectRef,
    cursor: Option<Cursor>,
    #[cfg(feature = "llvm-backend")]
    native_executor: Option<Rc<NativeClosureHost>>,
}

impl StoryMachine {
    pub fn new(program: StoryProgram) -> Result<Self> {
        Self::with_context(program, BTreeMap::new())
    }

    pub fn with_context(program: StoryProgram, context: BTreeMap<String, Value>) -> Result<Self> {
        Self::build(program, context)
    }

    #[cfg(feature = "llvm-backend")]
    pub fn with_native_executor(
        program: StoryProgram,
        context: BTreeMap<String, Value>,
        native_executor: Rc<NativeClosureHost>,
    ) -> Result<Self> {
        Self::build_with_native(program, context, Some(native_executor))
    }

    fn build(program: StoryProgram, context: BTreeMap<String, Value>) -> Result<Self> {
        #[cfg(feature = "llvm-backend")]
        {
            Self::build_with_native(program, context, None)
        }

        #[cfg(not(feature = "llvm-backend"))]
        {
            if program.find_part_index(&program.start_part).is_none() {
                return Err(RuntimeError::UnknownPart(program.start_part.clone()));
            }

            Ok(Self {
                program,
                globals: Scope::new(),
                context: std::rc::Rc::new(std::cell::RefCell::new(context)),
                cursor: None,
            })
        }
    }

    #[cfg(feature = "llvm-backend")]
    fn build_with_native(
        program: StoryProgram,
        context: BTreeMap<String, Value>,
        native_executor: Option<Rc<NativeClosureHost>>,
    ) -> Result<Self> {
        if program.find_part_index(&program.start_part).is_none() {
            return Err(RuntimeError::UnknownPart(program.start_part.clone()));
        }

        Ok(Self {
            program,
            globals: Scope::new(),
            context: std::rc::Rc::new(std::cell::RefCell::new(context)),
            cursor: None,
            native_executor,
        })
    }

    pub fn program(&self) -> &StoryProgram {
        &self.program
    }

    pub fn context_snapshot(&self) -> BTreeMap<String, Value> {
        self.context.borrow().clone()
    }

    pub fn context_value(&self, key: &str) -> Option<Value> {
        self.context.borrow().get(key).cloned()
    }

    pub fn start(&mut self) -> Result<StoryEvent> {
        let Some(start_index) = self.program.find_part_index(&self.program.start_part) else {
            return Err(RuntimeError::UnknownPart(self.program.start_part.clone()));
        };

        self.cursor = Some(Cursor {
            part_index: start_index,
            step_index: 0,
        });

        self.normalize_cursor()?;
        self.render_current()
    }

    pub fn advance(&mut self) -> Result<StoryEvent> {
        let Some(cursor) = self.cursor else {
            return Err(RuntimeError::EndOfStory);
        };

        let step = self.program.parts[cursor.part_index].steps[cursor.step_index].clone();

        let goto_target = match step {
            StepSpec::Narration(quote) => self.execute_quote(&quote)?.goto,
            StepSpec::Dialogue(dialogue) => self.execute_quote(&dialogue.quote)?.goto,
            StepSpec::Selection(_) => return Err(RuntimeError::ChoiceExpected),
        };

        self.move_after_current(goto_target.as_deref())?;
        self.render_current()
    }

    pub fn choose(&mut self, choice_index: usize) -> Result<StoryEvent> {
        let Some(cursor) = self.cursor else {
            return Err(RuntimeError::EndOfStory);
        };

        let selection = match self.program.parts[cursor.part_index].steps[cursor.step_index].clone()
        {
            StepSpec::Selection(selection) => selection,
            _ => return Err(RuntimeError::NotInSelection),
        };

        let Some(choice) = selection.choices.get(choice_index).cloned() else {
            return Err(RuntimeError::InvalidChoice {
                index: choice_index,
                len: selection.choices.len(),
            });
        };

        let goto_target = self.execute_quote(&choice)?.goto;

        self.move_after_current(goto_target.as_deref())?;
        self.render_current()
    }

    fn render_current(&mut self) -> Result<StoryEvent> {
        let Some(cursor) = self.cursor else {
            return Ok(StoryEvent::Finished);
        };

        let step = self.program.parts[cursor.part_index].steps[cursor.step_index].clone();
        match step {
            StepSpec::Narration(quote) => Ok(StoryEvent::Narration(NarrationView {
                text: quote.text.clone(),
                properties: self.evaluate_properties(&quote.properties)?,
            })),
            StepSpec::Dialogue(dialogue) => Ok(StoryEvent::Dialogue(DialogueView {
                speaker: dialogue.speaker.clone(),
                text: dialogue.quote.text.clone(),
                properties: self.evaluate_properties(&dialogue.quote.properties)?,
            })),
            StepSpec::Selection(selection) => {
                Ok(StoryEvent::Selection(self.render_selection(&selection)?))
            }
        }
    }

    fn render_selection(&mut self, selection: &SelectionSpec) -> Result<SelectionView> {
        let mut choices = Vec::with_capacity(selection.choices.len());
        for choice in &selection.choices {
            choices.push(ChoiceView {
                text: choice.text.clone(),
                properties: self.evaluate_properties(&choice.properties)?,
            });
        }
        Ok(SelectionView { choices })
    }

    fn evaluate_properties(
        &mut self,
        properties: &BTreeMap<String, Expr>,
    ) -> Result<BTreeMap<String, Value>> {
        let mut evaluated = BTreeMap::new();
        let globals = self.globals.clone();
        for (key, value) in properties {
            match self.eval_expr(value, &globals)? {
                EvalSignal::Value(value) => {
                    evaluated.insert(key.clone(), value);
                }
                EvalSignal::Goto(_) => return Err(RuntimeError::UnexpectedControlFlow),
            }
        }
        Ok(evaluated)
    }

    fn execute_quote(&mut self, quote: &QuoteSpec) -> Result<InvocationResult> {
        match quote.next_action {
            Some(function_id) => {
                self.invoke_function(function_id, self.globals.clone(), Vec::new())
            }
            None => Ok(InvocationResult {
                value: Value::None,
                goto: None,
            }),
        }
    }

    fn invoke_function(
        &mut self,
        function_id: usize,
        captured: Scope,
        args: Vec<Value>,
    ) -> Result<InvocationResult> {
        #[cfg(feature = "llvm-backend")]
        if let Some(native_executor) = self.native_executor.as_ref() {
            let result = native_executor
                .invoke_function(function_id, captured, self.context.clone(), args)
                .map_err(RuntimeError::NativeExecution);

            return result.map(|result| InvocationResult {
                value: result.value,
                goto: result.goto,
            });
        }

        self.invoke_interpreted_function(function_id, captured, args)
    }

    fn invoke_interpreted_function(
        &mut self,
        function_id: usize,
        captured: Scope,
        args: Vec<Value>,
    ) -> Result<InvocationResult> {
        let function = self
            .program
            .function(function_id)
            .ok_or_else(|| RuntimeError::InvalidCallee(format!("closure#{function_id}")))?
            .clone();

        if function.params.len() != args.len() {
            return Err(RuntimeError::ArityMismatch {
                expected: function.params.len(),
                got: args.len(),
            });
        }

        let frame = captured.child();
        for (param, value) in function.params.iter().zip(args) {
            frame.define(param, value);
        }

        match self.exec_block(&function.body, &frame)? {
            ExecSignal::Continue => Ok(InvocationResult {
                value: Value::None,
                goto: None,
            }),
            ExecSignal::Return(value) => Ok(InvocationResult { value, goto: None }),
            ExecSignal::Goto(target) => Ok(InvocationResult {
                value: Value::None,
                goto: Some(target),
            }),
        }
    }

    fn exec_block(&mut self, block: &Block, scope: &Scope) -> Result<ExecSignal> {
        for statement in &block.statements {
            match self.exec_stmt(statement, scope)? {
                ExecSignal::Continue => continue,
                signal => return Ok(signal),
            }
        }

        Ok(ExecSignal::Continue)
    }

    fn exec_stmt(&mut self, statement: &Stmt, scope: &Scope) -> Result<ExecSignal> {
        match statement {
            Stmt::Expr(expr) => match self.eval_expr(expr, scope)? {
                EvalSignal::Value(_) => Ok(ExecSignal::Continue),
                EvalSignal::Goto(target) => Ok(ExecSignal::Goto(target)),
            },
            Stmt::Block(block) => {
                let child = scope.child();
                self.exec_block(block, &child)
            }
            Stmt::Let { name, initializer } => match self.eval_expr(initializer, scope)? {
                EvalSignal::Value(value) => {
                    scope.define(name, value);
                    Ok(ExecSignal::Continue)
                }
                EvalSignal::Goto(target) => Ok(ExecSignal::Goto(target)),
            },
            Stmt::Goto(target) => match self.eval_expr(target, scope)? {
                EvalSignal::Value(value) => Ok(ExecSignal::Goto(value.to_story_target()?)),
                EvalSignal::Goto(target) => Ok(ExecSignal::Goto(target)),
            },
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = match self.eval_expr(condition, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(ExecSignal::Goto(target)),
                };

                if condition.to_bool()? {
                    self.exec_block(then_branch, &scope.child())
                } else if let Some(else_branch) = else_branch {
                    self.exec_stmt(else_branch, scope)
                } else {
                    Ok(ExecSignal::Continue)
                }
            }
            Stmt::Return(value) => {
                let value = match value {
                    Some(expr) => match self.eval_expr(expr, scope)? {
                        EvalSignal::Value(value) => value,
                        EvalSignal::Goto(target) => return Ok(ExecSignal::Goto(target)),
                    },
                    None => Value::None,
                };

                Ok(ExecSignal::Return(value))
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr, scope: &Scope) -> Result<EvalSignal> {
        Ok(match expr {
            Expr::Literal(literal) => EvalSignal::Value(match literal {
                Literal::Number(value) => Value::Number(*value),
                Literal::Boolean(value) => Value::Boolean(*value),
                Literal::String(value) => Value::String(value.clone()),
                Literal::None => Value::None,
            }),
            Expr::Identifier(name) => EvalSignal::Value(
                scope
                    .get(name)
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?,
            ),
            Expr::StoryReference(name) => EvalSignal::Value(Value::StoryRef(name.clone())),
            Expr::Context => EvalSignal::Value(Value::Object(self.context.clone())),
            Expr::Object(properties) => {
                let mut object = BTreeMap::new();
                for (key, value) in properties {
                    let value = match self.eval_expr(value, scope)? {
                        EvalSignal::Value(value) => value,
                        EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                    };
                    object.insert(key.clone(), value);
                }

                EvalSignal::Value(Value::object(object))
            }
            Expr::Closure(function_id) => EvalSignal::Value(Value::Closure(ClosureValue {
                function_id: *function_id,
                captured: scope.clone(),
            })),
            Expr::Call { callee, arguments } => {
                let callee = match self.eval_expr(callee, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };

                let mut args = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    let value = match self.eval_expr(argument, scope)? {
                        EvalSignal::Value(value) => value,
                        EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                    };
                    args.push(value);
                }

                match callee {
                    Value::Closure(closure) => {
                        let result =
                            self.invoke_function(closure.function_id, closure.captured, args)?;
                        match result.goto {
                            Some(target) => EvalSignal::Goto(target),
                            None => EvalSignal::Value(result.value),
                        }
                    }
                    other => {
                        return Err(RuntimeError::InvalidCallee(other.kind_name().to_string()));
                    }
                }
            }
            Expr::MemberAccess { base, members } => {
                let mut current = match self.eval_expr(base, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };

                for member in members {
                    let key = self.resolve_member_key(member, scope)?;
                    current = match current {
                        Value::Object(object) => {
                            object.borrow().get(&key).cloned().ok_or_else(|| {
                                RuntimeError::InvalidMemberAccess {
                                    target: "Object".to_string(),
                                    member: key.clone(),
                                }
                            })?
                        }
                        other => {
                            return Err(RuntimeError::InvalidMemberAccess {
                                target: other.kind_name().to_string(),
                                member: key,
                            });
                        }
                    };
                }

                EvalSignal::Value(current)
            }
            Expr::Assignment { target, value } => {
                let value = match self.eval_expr(value, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };
                self.assign_target(target, scope, value)?;
                EvalSignal::Value(Value::None)
            }
            Expr::Unary { operator, right } => {
                let right = match self.eval_expr(right, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };
                EvalSignal::Value(match operator {
                    UnaryOperator::Not => Value::Boolean(!right.to_bool()?),
                    UnaryOperator::Negate => Value::Number(-right.to_number()?),
                })
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = match self.eval_expr(left, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };
                let right = match self.eval_expr(right, scope)? {
                    EvalSignal::Value(value) => value,
                    EvalSignal::Goto(target) => return Ok(EvalSignal::Goto(target)),
                };

                let value = match operator {
                    BinaryOperator::Add => left.add(&right)?,
                    BinaryOperator::Subtract => left.subtract(&right)?,
                    BinaryOperator::Multiply => left.multiply(&right)?,
                    BinaryOperator::Divide => left.divide(&right)?,
                    BinaryOperator::EqualEqual => Value::Boolean(left == right),
                    BinaryOperator::NotEqual => Value::Boolean(left != right),
                    BinaryOperator::Greater => {
                        Value::Boolean(left.to_number()? > right.to_number()?)
                    }
                    BinaryOperator::GreaterEqual => {
                        Value::Boolean(left.to_number()? >= right.to_number()?)
                    }
                    BinaryOperator::Less => Value::Boolean(left.to_number()? < right.to_number()?),
                    BinaryOperator::LessEqual => {
                        Value::Boolean(left.to_number()? <= right.to_number()?)
                    }
                    BinaryOperator::And => Value::Boolean(left.to_bool()? && right.to_bool()?),
                    BinaryOperator::Or => Value::Boolean(left.to_bool()? || right.to_bool()?),
                };

                EvalSignal::Value(value)
            }
            Expr::Grouping(expr) => return self.eval_expr(expr, scope),
        })
    }

    fn assign_target(&mut self, target: &Expr, scope: &Scope, value: Value) -> Result<()> {
        match target {
            Expr::Identifier(name) => {
                if scope.assign(name, value) {
                    Ok(())
                } else {
                    Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            }
            Expr::MemberAccess { base, members } => {
                let (object, key) = self.resolve_member_place(base, members, scope)?;
                object.borrow_mut().insert(key, value);
                Ok(())
            }
            _ => Err(RuntimeError::InvalidAssignmentTarget),
        }
    }

    fn resolve_member_place(
        &mut self,
        base: &Expr,
        members: &[MemberSegment],
        scope: &Scope,
    ) -> Result<(ObjectRef, String)> {
        let mut current = match self.eval_expr(base, scope)? {
            EvalSignal::Value(value) => value,
            EvalSignal::Goto(target) => return Err(RuntimeError::InvalidStoryTarget(target)),
        };

        let Some((last, rest)) = members.split_last() else {
            return Err(RuntimeError::InvalidAssignmentTarget);
        };

        for member in rest {
            let key = self.resolve_member_key(member, scope)?;
            current = match current {
                Value::Object(object) => object.borrow().get(&key).cloned().ok_or_else(|| {
                    RuntimeError::InvalidMemberAccess {
                        target: "Object".to_string(),
                        member: key.clone(),
                    }
                })?,
                other => {
                    return Err(RuntimeError::InvalidMemberAccess {
                        target: other.kind_name().to_string(),
                        member: key,
                    });
                }
            };
        }

        let object = match current {
            Value::Object(object) => object,
            other => {
                return Err(RuntimeError::InvalidMemberAccess {
                    target: other.kind_name().to_string(),
                    member: self.resolve_member_key(last, scope)?,
                });
            }
        };

        Ok((object, self.resolve_member_key(last, scope)?))
    }

    fn resolve_member_key(&mut self, member: &MemberSegment, scope: &Scope) -> Result<String> {
        match member {
            MemberSegment::Key(value) => Ok(value.clone()),
            MemberSegment::Expr(expr) => match self.eval_expr(expr, scope)? {
                EvalSignal::Value(value) => value.to_member_key(),
                EvalSignal::Goto(target) => Err(RuntimeError::InvalidStoryTarget(target)),
            },
        }
    }

    fn move_after_current(&mut self, goto_target: Option<&str>) -> Result<()> {
        if let Some(target) = goto_target {
            let Some(part_index) = self.program.find_part_index(target) else {
                return Err(RuntimeError::UnknownPart(target.to_string()));
            };
            self.cursor = Some(Cursor {
                part_index,
                step_index: 0,
            });
            self.normalize_cursor()?;
            return Ok(());
        }

        let Some(cursor) = self.cursor else {
            return Ok(());
        };

        let next_index = cursor.step_index + 1;
        if self.program.parts[cursor.part_index]
            .steps
            .get(next_index)
            .is_some()
        {
            self.cursor = Some(Cursor {
                part_index: cursor.part_index,
                step_index: next_index,
            });
        } else {
            self.cursor = None;
        }

        self.normalize_cursor()
    }

    fn normalize_cursor(&mut self) -> Result<()> {
        while let Some(cursor) = self.cursor {
            let part = &self.program.parts[cursor.part_index];
            if part.steps.is_empty() {
                self.cursor = None;
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compile::StoryCompiler;

    use super::{StoryEvent, StoryMachine, Value};

    const BRANCHING_STORY: &str = r#"
    Story { start: "part_1" }

    # part_1
    [Hero]
    > "Hello there!"
    - "Hi!" {
        next: () => {
            let x = 10;
            let y = 20;
            context.total = x + y;
            goto part_2;
        }
    }
    - "Who are you?" {
        next: () => {
            goto part_3;
        }
    }

    # part_2
    [Villain]
    > "I've been expecting you."

    # part_3
    [Hero]
    > "I don't trust you."
    "#;

    #[test]
    fn machine_pauses_for_selection_and_updates_context() {
        let program = StoryCompiler
            .lower_source(BRANCHING_STORY)
            .expect("program should lower");
        let mut machine = StoryMachine::new(program).expect("machine should build");

        let event = machine.start().expect("story should start");
        assert_eq!(
            event,
            StoryEvent::Dialogue(super::DialogueView {
                speaker: "Hero".to_string(),
                text: "Hello there!".to_string(),
                properties: Default::default(),
            })
        );

        let event = machine.advance().expect("dialogue should advance");
        let StoryEvent::Selection(selection) = event else {
            panic!("expected selection");
        };
        assert_eq!(selection.choices.len(), 2);
        assert_eq!(selection.choices[0].text, "Hi!");

        let event = machine.choose(0).expect("choice should resolve");
        assert_eq!(machine.context_value("total"), Some(Value::Number(30.0)));

        assert_eq!(
            event,
            StoryEvent::Dialogue(super::DialogueView {
                speaker: "Villain".to_string(),
                text: "I've been expecting you.".to_string(),
                properties: Default::default(),
            })
        );
    }

    #[test]
    fn selecting_invalid_choice_reports_error() {
        let program = StoryCompiler
            .lower_source(BRANCHING_STORY)
            .expect("program should lower");
        let mut machine = StoryMachine::new(program).expect("machine should build");

        machine.start().expect("story should start");
        machine.advance().expect("selection should render");

        let error = machine.choose(99).expect_err("invalid choice should fail");
        assert!(matches!(error, super::RuntimeError::InvalidChoice { .. }));
    }

    #[test]
    fn runtime_can_branch_to_second_target() {
        let program = StoryCompiler
            .lower_source(BRANCHING_STORY)
            .expect("program should lower");
        let mut machine = StoryMachine::new(program).expect("machine should build");

        machine.start().expect("story should start");
        machine.advance().expect("selection should render");

        let event = machine.choose(1).expect("choice should resolve");
        assert_eq!(
            event,
            StoryEvent::Dialogue(super::DialogueView {
                speaker: "Hero".to_string(),
                text: "I don't trust you.".to_string(),
                properties: Default::default(),
            })
        );
    }
}
