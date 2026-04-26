use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    ffi::c_void,
    ptr,
    rc::Rc,
    slice, str,
};

use inkwell::{
    context::Context,
    execution_engine::ExecutionEngine,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{InitializationConfig, Target},
    OptimizationLevel,
};

use crate::{
    error::{Error, Result},
    ir::{FunctionId, StoryProgram},
};

use super::{ClosureValue, ObjectRef, Scope, Value};

type RawPtr = *mut c_void;
type NativeClosureFn = unsafe extern "C" fn(RawPtr, RawPtr) -> RawPtr;

#[derive(Debug, Clone)]
pub struct NativeInvocationResult {
    pub value: Value,
    pub goto: Option<String>,
}

#[derive(Clone)]
struct NativeFunctionMetadata {
    params: Vec<String>,
    function: NativeClosureFn,
}

enum NativeOutcome {
    Continue,
    Return(Value),
    Goto(String),
}

impl NativeOutcome {
    fn kind(&self) -> u64 {
        match self {
            Self::Continue => 0,
            Self::Return(_) => 1,
            Self::Goto(_) => 2,
        }
    }
}

pub struct NativeClosureHost {
    _context: &'static Context,
    _engine: ExecutionEngine<'static>,
    functions: BTreeMap<FunctionId, NativeFunctionMetadata>,
    symbols: BTreeMap<String, FunctionId>,
}

impl std::fmt::Debug for NativeClosureHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NativeClosureHost")
            .field("functions", &self.functions.keys().collect::<Vec<_>>())
            .finish()
    }
}

thread_local! {
    static ACTIVE_HOST: Cell<*const NativeClosureHost> = const { Cell::new(ptr::null()) };
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

impl NativeClosureHost {
    pub fn from_llvm_ir(
        llvm_ir: &str,
        program: &StoryProgram,
        function_symbols: &BTreeMap<FunctionId, String>,
        module_name: &str,
    ) -> Result<Rc<Self>> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|error| Error::NativeJit(error.to_string()))?;
        ExecutionEngine::link_in_mc_jit();

        let context = Box::leak(Box::new(Context::create()));
        let memory_buffer =
            MemoryBuffer::create_from_memory_range_copy(llvm_ir.as_bytes(), module_name);
        let module = context
            .create_module_from_ir(memory_buffer)
            .map_err(|error| Error::NativeJit(error.to_string()))?;
        let runtime_declarations = runtime_declarations(&module);
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|error| Error::NativeJit(error.to_string()))?;

        register_runtime_symbols(&execution_engine, &runtime_declarations);

        let mut functions = BTreeMap::new();
        let mut symbols = BTreeMap::new();

        for function in &program.functions {
            let Some(symbol) = function_symbols.get(&function.id) else {
                return Err(Error::NativeJit(format!(
                    "missing compiled symbol for closure {}",
                    function.id
                )));
            };

            let address = execution_engine
                .get_function_address(symbol)
                .map_err(|error| {
                    Error::NativeJit(format!("failed to resolve `{symbol}`: {error}"))
                })?;
            let function_ptr = unsafe { std::mem::transmute::<usize, NativeClosureFn>(address) };

            symbols.insert(symbol.clone(), function.id);
            functions.insert(
                function.id,
                NativeFunctionMetadata {
                    params: function.params.clone(),
                    function: function_ptr,
                },
            );
        }

        Ok(Rc::new(Self {
            _context: context,
            _engine: execution_engine,
            functions,
            symbols,
        }))
    }

    pub fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> std::result::Result<NativeInvocationResult, String> {
        let Some(metadata) = self.functions.get(&function_id) else {
            return Err(format!("unknown native closure {function_id}"));
        };

        if metadata.params.len() != args.len() {
            return Err(format!(
                "closure expected {} arguments but received {}",
                metadata.params.len(),
                args.len()
            ));
        }

        let frame = captured.child();
        for (param, value) in metadata.params.iter().zip(args) {
            frame.define(param, value);
        }

        self.invoke_compiled(metadata.function, frame, context)
    }

    fn invoke_compiled(
        &self,
        function: NativeClosureFn,
        frame: Scope,
        context: ObjectRef,
    ) -> std::result::Result<NativeInvocationResult, String> {
        let frame_ptr = Box::into_raw(Box::new(frame)) as RawPtr;
        let context_ptr = Box::into_raw(Box::new(context)) as RawPtr;

        let previous_host = ACTIVE_HOST.with(|slot| {
            let previous = slot.get();
            slot.set(self as *const _);
            previous
        });
        LAST_ERROR.with(|slot| {
            slot.borrow_mut().take();
        });

        let outcome_ptr = unsafe { function(frame_ptr, context_ptr) };

        ACTIVE_HOST.with(|slot| slot.set(previous_host));
        unsafe {
            drop(Box::from_raw(frame_ptr as *mut Scope));
            drop(Box::from_raw(context_ptr as *mut ObjectRef));
        }

        if let Some(error) = LAST_ERROR.with(|slot| slot.borrow_mut().take()) {
            return Err(error);
        }

        let outcome = unsafe { take_outcome(outcome_ptr) };
        Ok(match outcome {
            NativeOutcome::Continue => NativeInvocationResult {
                value: Value::None,
                goto: None,
            },
            NativeOutcome::Return(value) => NativeInvocationResult { value, goto: None },
            NativeOutcome::Goto(target) => NativeInvocationResult {
                value: Value::None,
                goto: Some(target),
            },
        })
    }
}

fn runtime_declarations(
    module: &Module<'static>,
) -> Vec<(inkwell::values::FunctionValue<'static>, usize)> {
    let mut declarations = Vec::new();

    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_value_none",
        runtime_addr(fabc_rt_value_none as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_value_number",
        runtime_addr(fabc_rt_value_number as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_value_boolean",
        runtime_addr(fabc_rt_value_boolean as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_value_string",
        runtime_addr(fabc_rt_value_string as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_value_story_ref",
        runtime_addr(fabc_rt_value_story_ref as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_context_value",
        runtime_addr(fabc_rt_context_value as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_object_new",
        runtime_addr(fabc_rt_object_new as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_object_insert",
        runtime_addr(fabc_rt_object_insert as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_env_child",
        runtime_addr(fabc_rt_env_child as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_env_load",
        runtime_addr(fabc_rt_env_load as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_env_define",
        runtime_addr(fabc_rt_env_define as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_env_assign",
        runtime_addr(fabc_rt_env_assign as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_member_get",
        runtime_addr(fabc_rt_member_get as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_member_assign",
        runtime_addr(fabc_rt_member_assign as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_call",
        runtime_addr(fabc_rt_call as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_closure_new",
        runtime_addr(fabc_rt_closure_new as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_unary_not",
        runtime_addr(fabc_rt_unary_not as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_unary_negate",
        runtime_addr(fabc_rt_unary_negate as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_add",
        runtime_addr(fabc_rt_binary_add as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_subtract",
        runtime_addr(fabc_rt_binary_subtract as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_multiply",
        runtime_addr(fabc_rt_binary_multiply as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_divide",
        runtime_addr(fabc_rt_binary_divide as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_equal",
        runtime_addr(fabc_rt_binary_equal as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_not_equal",
        runtime_addr(fabc_rt_binary_not_equal as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_greater",
        runtime_addr(fabc_rt_binary_greater as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_greater_equal",
        runtime_addr(fabc_rt_binary_greater_equal as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_less",
        runtime_addr(fabc_rt_binary_less as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_less_equal",
        runtime_addr(fabc_rt_binary_less_equal as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_and",
        runtime_addr(fabc_rt_binary_and as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_binary_or",
        runtime_addr(fabc_rt_binary_or as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_is_truthy",
        runtime_addr(fabc_rt_is_truthy as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_outcome_continue",
        runtime_addr(fabc_rt_outcome_continue as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_outcome_return",
        runtime_addr(fabc_rt_outcome_return as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_outcome_goto",
        runtime_addr(fabc_rt_outcome_goto as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_outcome_kind",
        runtime_addr(fabc_rt_outcome_kind as *const ()),
    );
    map_runtime_declaration(
        module,
        &mut declarations,
        "fabc_rt_outcome_into_value",
        runtime_addr(fabc_rt_outcome_into_value as *const ()),
    );

    declarations
}

fn runtime_addr(function: *const ()) -> usize {
    function as usize
}

fn map_runtime_declaration(
    module: &Module<'static>,
    declarations: &mut Vec<(inkwell::values::FunctionValue<'static>, usize)>,
    name: &str,
    address: usize,
) {
    if let Some(function) = module.get_function(name) {
        declarations.push((function, address));
    }
}

fn register_runtime_symbols(
    execution_engine: &ExecutionEngine<'static>,
    declarations: &[(inkwell::values::FunctionValue<'static>, usize)],
) {
    for (function, address) in declarations {
        execution_engine.add_global_mapping(function, *address);
    }
}

fn set_last_error(error: String) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(error);
    });
}

fn with_active_host<T>(
    f: impl FnOnce(&NativeClosureHost) -> std::result::Result<T, String>,
) -> std::result::Result<T, String> {
    ACTIVE_HOST.with(|slot| {
        let host_ptr = slot.get();
        if host_ptr.is_null() {
            Err("native closure host is not active".to_string())
        } else {
            unsafe { f(&*host_ptr) }
        }
    })
}

fn box_value(value: Value) -> RawPtr {
    Box::into_raw(Box::new(value)) as RawPtr
}

unsafe fn take_value(ptr: RawPtr) -> Value {
    *Box::from_raw(ptr as *mut Value)
}

unsafe fn clone_scope(ptr: RawPtr) -> Scope {
    (*(ptr as *mut Scope)).clone()
}

unsafe fn clone_context(ptr: RawPtr) -> ObjectRef {
    (*(ptr as *mut ObjectRef)).clone()
}

fn box_outcome(outcome: NativeOutcome) -> RawPtr {
    Box::into_raw(Box::new(outcome)) as RawPtr
}

unsafe fn take_outcome(ptr: RawPtr) -> NativeOutcome {
    *Box::from_raw(ptr as *mut NativeOutcome)
}

fn read_string(bytes: *const i8, len: u64) -> std::result::Result<String, String> {
    let bytes = unsafe { slice::from_raw_parts(bytes as *const u8, len as usize) };
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|error| error.to_string())
}

unsafe extern "C" fn fabc_rt_value_none() -> RawPtr {
    box_value(Value::None)
}

unsafe extern "C" fn fabc_rt_value_number(value: f64) -> RawPtr {
    box_value(Value::Number(value))
}

unsafe extern "C" fn fabc_rt_value_boolean(value: bool) -> RawPtr {
    box_value(Value::Boolean(value))
}

unsafe extern "C" fn fabc_rt_value_string(bytes: *const i8, len: u64) -> RawPtr {
    match read_string(bytes, len) {
        Ok(value) => box_value(Value::String(value)),
        Err(error) => {
            set_last_error(error);
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_value_story_ref(bytes: *const i8, len: u64) -> RawPtr {
    match read_string(bytes, len) {
        Ok(value) => box_value(Value::StoryRef(value)),
        Err(error) => {
            set_last_error(error);
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_context_value(context: RawPtr) -> RawPtr {
    box_value(Value::Object(clone_context(context)))
}

unsafe extern "C" fn fabc_rt_object_new() -> RawPtr {
    box_value(Value::object(BTreeMap::new()))
}

unsafe extern "C" fn fabc_rt_object_insert(
    object: RawPtr,
    key: *const i8,
    len: u64,
    value: RawPtr,
) {
    let key = match read_string(key, len) {
        Ok(key) => key,
        Err(error) => {
            set_last_error(error);
            return;
        }
    };

    let value = take_value(value);
    match &mut *(object as *mut Value) {
        Value::Object(object) => {
            object.borrow_mut().insert(key, value);
        }
        other => set_last_error(format!("cannot insert member into `{}`", other.kind_name())),
    }
}

unsafe extern "C" fn fabc_rt_env_child(parent: RawPtr) -> RawPtr {
    Box::into_raw(Box::new(clone_scope(parent).child())) as RawPtr
}

unsafe extern "C" fn fabc_rt_env_load(frame: RawPtr, name: *const i8, len: u64) -> RawPtr {
    let name = match read_string(name, len) {
        Ok(name) => name,
        Err(error) => {
            set_last_error(error);
            return box_value(Value::None);
        }
    };

    match clone_scope(frame).get(&name) {
        Some(value) => box_value(value),
        None => {
            set_last_error(format!("undefined variable `{name}`"));
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_env_define(frame: RawPtr, name: *const i8, len: u64, value: RawPtr) {
    let name = match read_string(name, len) {
        Ok(name) => name,
        Err(error) => {
            set_last_error(error);
            return;
        }
    };

    let value = take_value(value);
    clone_scope(frame).define(name, value);
}

unsafe extern "C" fn fabc_rt_env_assign(frame: RawPtr, name: *const i8, len: u64, value: RawPtr) {
    let name = match read_string(name, len) {
        Ok(name) => name,
        Err(error) => {
            set_last_error(error);
            return;
        }
    };

    let value = take_value(value);
    if !clone_scope(frame).assign(&name, value) {
        set_last_error(format!("undefined variable `{name}`"));
    }
}

unsafe extern "C" fn fabc_rt_member_get(base: RawPtr, key: RawPtr) -> RawPtr {
    let base = take_value(base);
    let key = take_value(key);
    let key = match key.to_member_key() {
        Ok(key) => key,
        Err(error) => {
            set_last_error(error.to_string());
            return box_value(Value::None);
        }
    };

    match base {
        Value::Object(object) => match object.borrow().get(&key).cloned() {
            Some(value) => box_value(value),
            None => {
                set_last_error(format!("cannot read member `{key}` from `Object`"));
                box_value(Value::None)
            }
        },
        other => {
            set_last_error(format!(
                "cannot read member `{key}` from `{}`",
                other.kind_name()
            ));
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_member_assign(container: RawPtr, key: RawPtr, value: RawPtr) {
    let key = match take_value(key).to_member_key() {
        Ok(key) => key,
        Err(error) => {
            set_last_error(error.to_string());
            return;
        }
    };
    let value = take_value(value);

    match &mut *(container as *mut Value) {
        Value::Object(object) => {
            object.borrow_mut().insert(key, value);
        }
        other => set_last_error(format!(
            "cannot assign member `{key}` on `{}`",
            other.kind_name()
        )),
    }
}

unsafe extern "C" fn fabc_rt_call(
    _frame: RawPtr,
    context: RawPtr,
    callee: RawPtr,
    args: *mut RawPtr,
    len: u64,
) -> RawPtr {
    let callee = take_value(callee);
    let args = slice::from_raw_parts(args, len as usize)
        .iter()
        .map(|argument| take_value(*argument))
        .collect::<Vec<_>>();
    let context = clone_context(context);

    let result = match callee {
        Value::Closure(ClosureValue {
            function_id,
            captured,
        }) => with_active_host(|host| host.invoke_function(function_id, captured, context, args)),
        other => Err(format!("invalid callable value `{}`", other.kind_name())),
    };

    match result {
        Ok(result) => match result.goto {
            Some(target) => box_outcome(NativeOutcome::Goto(target)),
            None => box_outcome(NativeOutcome::Return(result.value)),
        },
        Err(error) => {
            set_last_error(error);
            box_outcome(NativeOutcome::Return(Value::None))
        }
    }
}

unsafe extern "C" fn fabc_rt_closure_new(symbol: *const i8, len: u64, frame: RawPtr) -> RawPtr {
    let symbol = match read_string(symbol, len) {
        Ok(symbol) => symbol,
        Err(error) => {
            set_last_error(error);
            return box_value(Value::None);
        }
    };

    let result = with_active_host(|host| {
        host.symbols
            .get(&symbol)
            .copied()
            .ok_or_else(|| format!("unknown native closure symbol `{symbol}`"))
    });

    match result {
        Ok(function_id) => box_value(Value::Closure(ClosureValue {
            function_id,
            captured: clone_scope(frame),
        })),
        Err(error) => {
            set_last_error(error);
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_unary_not(value: RawPtr) -> RawPtr {
    match take_value(value).to_bool() {
        Ok(value) => box_value(Value::Boolean(!value)),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_unary_negate(value: RawPtr) -> RawPtr {
    match take_value(value).to_number() {
        Ok(value) => box_value(Value::Number(-value)),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

unsafe extern "C" fn fabc_rt_binary_add(left: RawPtr, right: RawPtr) -> RawPtr {
    binary_value(left, right, |left, right| left.add(&right))
}

unsafe extern "C" fn fabc_rt_binary_subtract(left: RawPtr, right: RawPtr) -> RawPtr {
    binary_value(left, right, |left, right| left.subtract(&right))
}

unsafe extern "C" fn fabc_rt_binary_multiply(left: RawPtr, right: RawPtr) -> RawPtr {
    binary_value(left, right, |left, right| left.multiply(&right))
}

unsafe extern "C" fn fabc_rt_binary_divide(left: RawPtr, right: RawPtr) -> RawPtr {
    binary_value(left, right, |left, right| left.divide(&right))
}

unsafe extern "C" fn fabc_rt_binary_equal(left: RawPtr, right: RawPtr) -> RawPtr {
    box_value(Value::Boolean(take_value(left) == take_value(right)))
}

unsafe extern "C" fn fabc_rt_binary_not_equal(left: RawPtr, right: RawPtr) -> RawPtr {
    box_value(Value::Boolean(take_value(left) != take_value(right)))
}

unsafe extern "C" fn fabc_rt_binary_greater(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_numbers(left, right, |left, right| left > right)
}

unsafe extern "C" fn fabc_rt_binary_greater_equal(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_numbers(left, right, |left, right| left >= right)
}

unsafe extern "C" fn fabc_rt_binary_less(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_numbers(left, right, |left, right| left < right)
}

unsafe extern "C" fn fabc_rt_binary_less_equal(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_numbers(left, right, |left, right| left <= right)
}

unsafe extern "C" fn fabc_rt_binary_and(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_bools(left, right, |left, right| left && right)
}

unsafe extern "C" fn fabc_rt_binary_or(left: RawPtr, right: RawPtr) -> RawPtr {
    compare_bools(left, right, |left, right| left || right)
}

unsafe extern "C" fn fabc_rt_is_truthy(value: RawPtr) -> bool {
    match take_value(value).to_bool() {
        Ok(value) => value,
        Err(error) => {
            set_last_error(error.to_string());
            false
        }
    }
}

unsafe extern "C" fn fabc_rt_outcome_continue() -> RawPtr {
    box_outcome(NativeOutcome::Continue)
}

unsafe extern "C" fn fabc_rt_outcome_return(value: RawPtr) -> RawPtr {
    box_outcome(NativeOutcome::Return(take_value(value)))
}

unsafe extern "C" fn fabc_rt_outcome_goto(value: RawPtr) -> RawPtr {
    match take_value(value).to_story_target() {
        Ok(target) => box_outcome(NativeOutcome::Goto(target)),
        Err(error) => {
            set_last_error(error.to_string());
            box_outcome(NativeOutcome::Continue)
        }
    }
}

unsafe extern "C" fn fabc_rt_outcome_kind(outcome: RawPtr) -> u64 {
    (&*(outcome as *mut NativeOutcome)).kind()
}

unsafe extern "C" fn fabc_rt_outcome_into_value(outcome: RawPtr) -> RawPtr {
    match take_outcome(outcome) {
        NativeOutcome::Continue => box_value(Value::None),
        NativeOutcome::Return(value) => box_value(value),
        NativeOutcome::Goto(target) => {
            set_last_error(format!(
                "cannot convert goto outcome `{target}` into a value"
            ));
            box_value(Value::None)
        }
    }
}

unsafe fn binary_value(
    left: RawPtr,
    right: RawPtr,
    op: impl FnOnce(Value, Value) -> std::result::Result<Value, super::RuntimeError>,
) -> RawPtr {
    match op(take_value(left), take_value(right)) {
        Ok(value) => box_value(value),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

unsafe fn compare_numbers(
    left: RawPtr,
    right: RawPtr,
    op: impl FnOnce(f64, f64) -> bool,
) -> RawPtr {
    let left = take_value(left);
    let right = take_value(right);
    match (left.to_number(), right.to_number()) {
        (Ok(left), Ok(right)) => box_value(Value::Boolean(op(left, right))),
        (Err(error), _) | (_, Err(error)) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

unsafe fn compare_bools(
    left: RawPtr,
    right: RawPtr,
    op: impl FnOnce(bool, bool) -> bool,
) -> RawPtr {
    let left = take_value(left);
    let right = take_value(right);
    match (left.to_bool(), right.to_bool()) {
        (Ok(left), Ok(right)) => box_value(Value::Boolean(op(left, right))),
        (Err(error), _) | (_, Err(error)) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}
