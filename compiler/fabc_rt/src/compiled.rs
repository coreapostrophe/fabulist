use std::{
    cell::RefCell,
    collections::BTreeMap,
    ffi::c_void,
    fmt::{Debug, Formatter, Result as FmtResult},
    slice, str,
};

use fabc_ir::FunctionId;

use super::{
    ClosureValue, CompiledFunctionHost, CompiledInvocationResult, ObjectRef, Scope, Value,
};

type RawPtr = *mut c_void;

pub type CompiledClosureFn = unsafe extern "C" fn(RawPtr, RawPtr) -> RawPtr;
pub type RuntimeSymbol = (&'static str, usize);

type InvokeFunctionFn = unsafe fn(
    *const c_void,
    FunctionId,
    Scope,
    ObjectRef,
    Vec<Value>,
) -> Result<CompiledInvocationResult, String>;

#[derive(Debug, Clone, Copy)]
pub struct LinkedFunctionDescriptor {
    pub id: FunctionId,
    pub symbol: &'static str,
    pub params: &'static [&'static str],
    pub function: CompiledClosureFn,
}

#[derive(Clone)]
struct LinkedFunctionMetadata {
    params: Vec<String>,
    function: CompiledClosureFn,
}

pub struct LinkedCompiledFunctionHost {
    functions: BTreeMap<FunctionId, LinkedFunctionMetadata>,
    symbols: BTreeMap<String, FunctionId>,
}

#[derive(Clone, Copy)]
struct ActiveHostDispatch {
    host: *const c_void,
    invoke_function: InvokeFunctionFn,
    resolve_function_symbol: unsafe fn(*const c_void, &str) -> Result<FunctionId, String>,
}

impl ActiveHostDispatch {
    fn new<T: CompiledFunctionHost>(host: &T) -> Self {
        Self {
            host: host as *const T as *const c_void,
            invoke_function: invoke_function_impl::<T>,
            resolve_function_symbol: resolve_function_symbol_impl::<T>,
        }
    }
}

impl LinkedCompiledFunctionHost {
    pub fn new(descriptors: &[LinkedFunctionDescriptor]) -> Self {
        let mut functions = BTreeMap::new();
        let mut symbols = BTreeMap::new();

        for descriptor in descriptors {
            functions.insert(
                descriptor.id,
                LinkedFunctionMetadata {
                    params: descriptor
                        .params
                        .iter()
                        .map(|param| (*param).to_string())
                        .collect(),
                    function: descriptor.function,
                },
            );
            symbols.insert(descriptor.symbol.to_string(), descriptor.id);
        }

        Self { functions, symbols }
    }
}

impl Debug for LinkedCompiledFunctionHost {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter
            .debug_struct("LinkedCompiledFunctionHost")
            .field("functions", &self.functions.keys().collect::<Vec<_>>())
            .finish()
    }
}

thread_local! {
    static ACTIVE_HOST: RefCell<Option<ActiveHostDispatch>> = const { RefCell::new(None) };
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

enum NativeOutcome {
    Continue,
    Return(Value),
    Goto(String),
}

impl CompiledFunctionHost for LinkedCompiledFunctionHost {
    fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> Result<CompiledInvocationResult, String> {
        let Some(metadata) = self.functions.get(&function_id) else {
            return Err(format!("unknown linked closure {function_id}"));
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

        invoke_compiled_with_active_host(self, metadata.function, frame, context)
    }

    fn resolve_function_symbol(&self, symbol: &str) -> Result<FunctionId, String> {
        self.symbols
            .get(symbol)
            .copied()
            .ok_or_else(|| format!("unknown linked closure symbol `{symbol}`"))
    }
}

pub fn invoke_compiled_with_active_host<T: CompiledFunctionHost>(
    host: &T,
    function: CompiledClosureFn,
    frame: Scope,
    context: ObjectRef,
) -> Result<CompiledInvocationResult, String> {
    let frame_ptr = Box::into_raw(Box::new(frame)) as RawPtr;
    let context_ptr = Box::into_raw(Box::new(context)) as RawPtr;

    let previous_host = ACTIVE_HOST.with(|slot| slot.replace(Some(ActiveHostDispatch::new(host))));
    LAST_ERROR.with(|slot| {
        slot.borrow_mut().take();
    });

    let outcome_ptr = unsafe { function(frame_ptr, context_ptr) };

    ACTIVE_HOST.with(|slot| {
        slot.replace(previous_host);
    });
    unsafe {
        drop(Box::from_raw(frame_ptr as *mut Scope));
        drop(Box::from_raw(context_ptr as *mut ObjectRef));
    }

    if let Some(error) = LAST_ERROR.with(|slot| slot.borrow_mut().take()) {
        return Err(error);
    }

    let outcome = unsafe { take_outcome(outcome_ptr) };
    Ok(match outcome {
        NativeOutcome::Continue => CompiledInvocationResult {
            value: Value::None,
            goto: None,
        },
        NativeOutcome::Return(value) => CompiledInvocationResult { value, goto: None },
        NativeOutcome::Goto(target) => CompiledInvocationResult {
            value: Value::None,
            goto: Some(target),
        },
    })
}

pub fn runtime_symbols() -> [RuntimeSymbol; 36] {
    [
        (
            "fabc_rt_value_none",
            runtime_addr(fabc_rt_value_none as *const ()),
        ),
        (
            "fabc_rt_value_number",
            runtime_addr(fabc_rt_value_number as *const ()),
        ),
        (
            "fabc_rt_value_boolean",
            runtime_addr(fabc_rt_value_boolean as *const ()),
        ),
        (
            "fabc_rt_value_string",
            runtime_addr(fabc_rt_value_string as *const ()),
        ),
        (
            "fabc_rt_value_story_ref",
            runtime_addr(fabc_rt_value_story_ref as *const ()),
        ),
        (
            "fabc_rt_context_value",
            runtime_addr(fabc_rt_context_value as *const ()),
        ),
        (
            "fabc_rt_object_new",
            runtime_addr(fabc_rt_object_new as *const ()),
        ),
        (
            "fabc_rt_object_insert",
            runtime_addr(fabc_rt_object_insert as *const ()),
        ),
        (
            "fabc_rt_env_child",
            runtime_addr(fabc_rt_env_child as *const ()),
        ),
        (
            "fabc_rt_env_load",
            runtime_addr(fabc_rt_env_load as *const ()),
        ),
        (
            "fabc_rt_env_define",
            runtime_addr(fabc_rt_env_define as *const ()),
        ),
        (
            "fabc_rt_env_assign",
            runtime_addr(fabc_rt_env_assign as *const ()),
        ),
        (
            "fabc_rt_member_get",
            runtime_addr(fabc_rt_member_get as *const ()),
        ),
        (
            "fabc_rt_member_assign",
            runtime_addr(fabc_rt_member_assign as *const ()),
        ),
        ("fabc_rt_call", runtime_addr(fabc_rt_call as *const ())),
        (
            "fabc_rt_closure_new",
            runtime_addr(fabc_rt_closure_new as *const ()),
        ),
        (
            "fabc_rt_unary_not",
            runtime_addr(fabc_rt_unary_not as *const ()),
        ),
        (
            "fabc_rt_unary_negate",
            runtime_addr(fabc_rt_unary_negate as *const ()),
        ),
        (
            "fabc_rt_binary_add",
            runtime_addr(fabc_rt_binary_add as *const ()),
        ),
        (
            "fabc_rt_binary_subtract",
            runtime_addr(fabc_rt_binary_subtract as *const ()),
        ),
        (
            "fabc_rt_binary_multiply",
            runtime_addr(fabc_rt_binary_multiply as *const ()),
        ),
        (
            "fabc_rt_binary_divide",
            runtime_addr(fabc_rt_binary_divide as *const ()),
        ),
        (
            "fabc_rt_binary_equal",
            runtime_addr(fabc_rt_binary_equal as *const ()),
        ),
        (
            "fabc_rt_binary_not_equal",
            runtime_addr(fabc_rt_binary_not_equal as *const ()),
        ),
        (
            "fabc_rt_binary_greater",
            runtime_addr(fabc_rt_binary_greater as *const ()),
        ),
        (
            "fabc_rt_binary_greater_equal",
            runtime_addr(fabc_rt_binary_greater_equal as *const ()),
        ),
        (
            "fabc_rt_binary_less",
            runtime_addr(fabc_rt_binary_less as *const ()),
        ),
        (
            "fabc_rt_binary_less_equal",
            runtime_addr(fabc_rt_binary_less_equal as *const ()),
        ),
        (
            "fabc_rt_binary_and",
            runtime_addr(fabc_rt_binary_and as *const ()),
        ),
        (
            "fabc_rt_binary_or",
            runtime_addr(fabc_rt_binary_or as *const ()),
        ),
        (
            "fabc_rt_is_truthy",
            runtime_addr(fabc_rt_is_truthy as *const ()),
        ),
        (
            "fabc_rt_outcome_continue",
            runtime_addr(fabc_rt_outcome_continue as *const ()),
        ),
        (
            "fabc_rt_outcome_return",
            runtime_addr(fabc_rt_outcome_return as *const ()),
        ),
        (
            "fabc_rt_outcome_goto",
            runtime_addr(fabc_rt_outcome_goto as *const ()),
        ),
        (
            "fabc_rt_outcome_kind",
            runtime_addr(fabc_rt_outcome_kind as *const ()),
        ),
        (
            "fabc_rt_outcome_into_value",
            runtime_addr(fabc_rt_outcome_into_value as *const ()),
        ),
    ]
}

fn runtime_addr(function: *const ()) -> usize {
    function as usize
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

fn read_string(bytes: *const i8, len: u64) -> Result<String, String> {
    let bytes = unsafe { slice::from_raw_parts(bytes as *const u8, len as usize) };
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|error| error.to_string())
}

fn set_last_error(error: String) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(error);
    });
}

fn with_active_host<T>(
    f: impl FnOnce(&dyn CompiledFunctionHost) -> Result<T, String>,
) -> Result<T, String> {
    ACTIVE_HOST.with(|slot| match *slot.borrow() {
        Some(dispatch) => f(&ActiveCompiledFunctionHostRef { dispatch }),
        None => Err("compiled function host is not active".to_string()),
    })
}

struct ActiveCompiledFunctionHostRef {
    dispatch: ActiveHostDispatch,
}

impl Debug for ActiveCompiledFunctionHostRef {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("ActiveCompiledFunctionHostRef")
    }
}

impl CompiledFunctionHost for ActiveCompiledFunctionHostRef {
    fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> Result<CompiledInvocationResult, String> {
        unsafe {
            (self.dispatch.invoke_function)(
                self.dispatch.host,
                function_id,
                captured,
                context,
                args,
            )
        }
    }

    fn resolve_function_symbol(&self, symbol: &str) -> Result<FunctionId, String> {
        unsafe { (self.dispatch.resolve_function_symbol)(self.dispatch.host, symbol) }
    }
}

unsafe fn invoke_function_impl<T: CompiledFunctionHost>(
    host: *const c_void,
    function_id: FunctionId,
    captured: Scope,
    context: ObjectRef,
    args: Vec<Value>,
) -> Result<CompiledInvocationResult, String> {
    (&*(host as *const T)).invoke_function(function_id, captured, context, args)
}

unsafe fn resolve_function_symbol_impl<T: CompiledFunctionHost>(
    host: *const c_void,
    symbol: &str,
) -> Result<FunctionId, String> {
    (&*(host as *const T)).resolve_function_symbol(symbol)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_value_none() -> RawPtr {
    box_value(Value::None)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_value_number(value: f64) -> RawPtr {
    box_value(Value::Number(value))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_value_boolean(value: bool) -> RawPtr {
    box_value(Value::Boolean(value))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_value_string(bytes: *const i8, len: u64) -> RawPtr {
    match read_string(bytes, len) {
        Ok(value) => box_value(Value::String(value)),
        Err(error) => {
            set_last_error(error);
            box_value(Value::None)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_value_story_ref(bytes: *const i8, len: u64) -> RawPtr {
    match read_string(bytes, len) {
        Ok(value) => box_value(Value::StoryRef(value)),
        Err(error) => {
            set_last_error(error);
            box_value(Value::None)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_context_value(context: RawPtr) -> RawPtr {
    box_value(Value::Object(clone_context(context)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_object_new() -> RawPtr {
    box_value(Value::object(BTreeMap::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_object_insert(
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_env_child(parent: RawPtr) -> RawPtr {
    Box::into_raw(Box::new(clone_scope(parent).child())) as RawPtr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_env_load(frame: RawPtr, name: *const i8, len: u64) -> RawPtr {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_env_define(
    frame: RawPtr,
    name: *const i8,
    len: u64,
    value: RawPtr,
) {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_env_assign(
    frame: RawPtr,
    name: *const i8,
    len: u64,
    value: RawPtr,
) {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_member_get(base: RawPtr, key: RawPtr) -> RawPtr {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_member_assign(container: RawPtr, key: RawPtr, value: RawPtr) {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_call(
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_closure_new(symbol: *const i8, len: u64, frame: RawPtr) -> RawPtr {
    let symbol = match read_string(symbol, len) {
        Ok(symbol) => symbol,
        Err(error) => {
            set_last_error(error);
            return box_value(Value::None);
        }
    };

    match with_active_host(|host| host.resolve_function_symbol(&symbol)) {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_unary_not(value: RawPtr) -> RawPtr {
    match take_value(value).to_bool() {
        Ok(value) => box_value(Value::Boolean(!value)),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_unary_negate(value: RawPtr) -> RawPtr {
    match take_value(value).to_number() {
        Ok(value) => box_value(Value::Number(-value)),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

macro_rules! binary_value_fn {
    ($name:ident, $handler:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(left: RawPtr, right: RawPtr) -> RawPtr {
            binary_value(left, right, $handler)
        }
    };
}

binary_value_fn!(fabc_rt_binary_add, |left: Value, right: Value| left
    .add(&right));
binary_value_fn!(fabc_rt_binary_subtract, |left: Value, right: Value| left
    .subtract(&right));
binary_value_fn!(fabc_rt_binary_multiply, |left: Value, right: Value| left
    .multiply(&right));
binary_value_fn!(fabc_rt_binary_divide, |left: Value, right: Value| left
    .divide(&right));
binary_value_fn!(fabc_rt_binary_equal, |left: Value, right: Value| Ok(
    Value::Boolean(left == right)
));
binary_value_fn!(fabc_rt_binary_not_equal, |left: Value, right: Value| Ok(
    Value::Boolean(left != right)
));
binary_value_fn!(fabc_rt_binary_greater, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_number()? > right.to_number()?))
});
binary_value_fn!(fabc_rt_binary_greater_equal, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_number()? >= right.to_number()?))
});
binary_value_fn!(fabc_rt_binary_less, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_number()? < right.to_number()?))
});
binary_value_fn!(fabc_rt_binary_less_equal, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_number()? <= right.to_number()?))
});
binary_value_fn!(fabc_rt_binary_and, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_bool()? && right.to_bool()?))
});
binary_value_fn!(fabc_rt_binary_or, |left: Value, right: Value| {
    Ok(Value::Boolean(left.to_bool()? || right.to_bool()?))
});

fn binary_value(
    left: RawPtr,
    right: RawPtr,
    handler: impl FnOnce(Value, Value) -> Result<Value, super::RuntimeError>,
) -> RawPtr {
    match handler(unsafe { take_value(left) }, unsafe { take_value(right) }) {
        Ok(value) => box_value(value),
        Err(error) => {
            set_last_error(error.to_string());
            box_value(Value::None)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_is_truthy(value: RawPtr) -> bool {
    match take_value(value).to_bool() {
        Ok(value) => value,
        Err(error) => {
            set_last_error(error.to_string());
            false
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_outcome_continue() -> RawPtr {
    box_outcome(NativeOutcome::Continue)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_outcome_return(value: RawPtr) -> RawPtr {
    box_outcome(NativeOutcome::Return(take_value(value)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_outcome_goto(target: RawPtr) -> RawPtr {
    match take_value(target).to_story_target() {
        Ok(target) => box_outcome(NativeOutcome::Goto(target)),
        Err(error) => {
            set_last_error(error.to_string());
            box_outcome(NativeOutcome::Return(Value::None))
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_outcome_kind(outcome: RawPtr) -> u64 {
    match &*(outcome as *mut NativeOutcome) {
        NativeOutcome::Continue => 0,
        NativeOutcome::Return(_) => 1,
        NativeOutcome::Goto(_) => 2,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fabc_rt_outcome_into_value(outcome: RawPtr) -> RawPtr {
    match take_outcome(outcome) {
        NativeOutcome::Continue => box_value(Value::None),
        NativeOutcome::Return(value) => box_value(value),
        NativeOutcome::Goto(target) => box_value(Value::StoryRef(target)),
    }
}
