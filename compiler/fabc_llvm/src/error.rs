use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("compiler emitted diagnostics: {0:?}")]
    Diagnostics(Vec<fabc_error::Error>),
    #[error("failed to read `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("parser reported diagnostics for `{path}`: {diagnostics:?}")]
    ParseDiagnostics {
        path: PathBuf,
        diagnostics: Vec<fabc_error::Error>,
    },
    #[error("semantic analysis reported diagnostics: {diagnostics:?}")]
    SemanticDiagnostics { diagnostics: Vec<fabc_error::Error> },
    #[error("`{path}` does not contain a story")]
    MissingStory { path: PathBuf },
    #[error("`{path}` contains multiple story blocks: {count}")]
    MultipleStories { path: PathBuf, count: usize },
    #[error("circular module import detected: {chain:?}")]
    CircularImport { chain: Vec<PathBuf> },
    #[error("duplicate part `{part}` from `{first}` and `{second}`")]
    DuplicatePart {
        part: String,
        first: PathBuf,
        second: PathBuf,
    },
    #[error("import path `{import}` from `{from}` is invalid")]
    InvalidImportPath { from: PathBuf, import: String },
    #[error("`exports` in `{path}` must be an object literal")]
    InvalidExportsObject { path: PathBuf },
    #[error("export `{export}` in `{path}` must be a literal, story reference, or imported module member")]
    InvalidExportValue { path: PathBuf, export: String },
    #[error("expected exactly one story init, found none")]
    MissingStoryInit,
    #[error("expected exactly one story init, found {0}")]
    MultipleStoryInits(usize),
    #[error("story metadata is missing a `start` entry")]
    MissingStartPart,
    #[error("story `start` metadata must be a string literal or a part identifier")]
    InvalidStartMetadata,
    #[error("story part `{0}` does not exist")]
    UnknownPart(String),
    #[error("module imports must be linked before lowering; use the entry-path APIs for `{0}`")]
    UnsupportedModuleImport(String),
    #[error("`next` properties must be closures")]
    InvalidNextHandler,
    #[error("closure parameters must be identifiers")]
    InvalidClosureParameter,
    #[error("runtime initialization failed: {0}")]
    RuntimeInitialization(String),
    #[error("LLVM native JIT failed: {0}")]
    NativeJit(String),
    #[error("LLVM code generation failed: {0}")]
    Codegen(String),
    #[error("LLVM code generation requires enabling one of the `llvmXX-0` crate features")]
    LlvmFeatureDisabled,
}

pub type Result<T> = std::result::Result<T, Error>;
