#[cfg(feature = "llvm-backend")]
mod native;

pub use fabc_rt::{
    ChoiceView, ClosureValue, CompiledFunctionHost, CompiledInvocationResult, DialogueView,
    NarrationView, ObjectRef, Result, RuntimeError, Scope, SelectionView, StoryEvent, StoryMachine,
    Value,
};
#[cfg(feature = "llvm-backend")]
pub use native::NativeClosureHost;
