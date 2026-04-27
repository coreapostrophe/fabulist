use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter, Result as FmtResult},
    rc::Rc,
    result::Result as StdResult,
};

use inkwell::{
    context::Context,
    execution_engine::ExecutionEngine,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{InitializationConfig, Target},
    OptimizationLevel,
};

use fabc_rt::{invoke_compiled_with_active_host, runtime_symbols, CompiledClosureFn};

use crate::{
    error::{Error, Result},
    ir::{FunctionId, StoryProgram},
};

use super::{CompiledFunctionHost, CompiledInvocationResult, ObjectRef, Scope, Value};

#[derive(Clone)]
struct NativeFunctionMetadata {
    params: Vec<String>,
    function: CompiledClosureFn,
}

pub struct NativeClosureHost {
    _context: &'static Context,
    _engine: ExecutionEngine<'static>,
    functions: BTreeMap<FunctionId, NativeFunctionMetadata>,
    symbols: BTreeMap<String, FunctionId>,
}

impl Debug for NativeClosureHost {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter
            .debug_struct("NativeClosureHost")
            .field("functions", &self.functions.keys().collect::<Vec<_>>())
            .finish()
    }
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
            // SAFETY: the symbol was emitted by our LLVM backend, which gives every compiled
            // closure the `CompiledClosureFn` ABI expected by the runtime bridge.
            let function_ptr = unsafe { std::mem::transmute::<usize, CompiledClosureFn>(address) };

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

    fn invoke_function_inner(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> StdResult<CompiledInvocationResult, String> {
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

        invoke_compiled_with_active_host(self, metadata.function, frame, context)
    }
}

impl CompiledFunctionHost for NativeClosureHost {
    fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> StdResult<CompiledInvocationResult, String> {
        self.invoke_function_inner(function_id, captured, context, args)
    }

    fn resolve_function_symbol(&self, symbol: &str) -> StdResult<FunctionId, String> {
        self.symbols
            .get(symbol)
            .copied()
            .ok_or_else(|| format!("unknown native closure symbol `{symbol}`"))
    }
}

fn runtime_declarations(
    module: &Module<'static>,
) -> Vec<(inkwell::values::FunctionValue<'static>, usize)> {
    let mut declarations = Vec::new();

    for (name, address) in runtime_symbols() {
        map_runtime_declaration(module, &mut declarations, name, address);
    }

    declarations
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
