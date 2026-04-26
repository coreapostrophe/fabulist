pub mod compile;
pub mod error;
pub mod frontend;
pub mod ir;
mod link;
pub mod runtime;

#[cfg(feature = "llvm-backend")]
pub mod llvm;

pub use compile::{lower_entry, lower_source, CompiledLlvmArtifact, StoryCompiler};
pub use error::{Error, Result};

pub const LLVM_BACKEND_ENABLED: bool = cfg!(feature = "llvm-backend");
