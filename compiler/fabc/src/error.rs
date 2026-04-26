use std::{io, path::PathBuf, result::Result as StdResult};

use fabc_llvm::Error as LlvmError;
use fabc_rt::RuntimeError as StoryRuntimeError;
use serde_json::Error as JsonError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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
    #[error("failed to encode compiled story bundle manifest: {0}")]
    BundleManifestSerialize(#[source] JsonError),
    #[error("failed to parse compiled story bundle manifest `{path}`: {source}")]
    BundleManifestParse {
        path: PathBuf,
        #[source]
        source: JsonError,
    },
    #[error("unsupported bundle format version {found} in `{path}`")]
    UnsupportedBundleFormatVersion { path: PathBuf, found: u32 },
    #[error("compiled bundle `{path}` is missing a function symbol for closure {function_id}")]
    MissingBundleFunctionSymbol { path: PathBuf, function_id: usize },
    #[error("failed to initialize story machine from compiled bundle: {0}")]
    BundleRuntimeInitialization(#[source] StoryRuntimeError),
    #[error("failed to encode embedded standalone story: {0}")]
    StandaloneStorySerialize(#[source] JsonError),
    #[error("standalone launcher command `{command}` failed with status {status}: {stderr}")]
    StandaloneBuildCommand {
        command: String,
        status: i32,
        stderr: String,
    },
    #[error(transparent)]
    Backend(LlvmError),
}

pub type Result<T> = StdResult<T, Error>;

impl From<LlvmError> for Error {
    fn from(error: LlvmError) -> Self {
        match error {
            LlvmError::Io { path, source } => Error::Io { path, source },
            LlvmError::ParseDiagnostics { path, diagnostics } => {
                Error::ParseDiagnostics { path, diagnostics }
            }
            LlvmError::SemanticDiagnostics { diagnostics } => {
                Error::SemanticDiagnostics { diagnostics }
            }
            LlvmError::MissingStory { path } => Error::MissingStory { path },
            LlvmError::MultipleStories { path, count } => Error::MultipleStories { path, count },
            LlvmError::CircularImport { chain } => Error::CircularImport { chain },
            LlvmError::DuplicatePart {
                part,
                first,
                second,
            } => Error::DuplicatePart {
                part,
                first,
                second,
            },
            LlvmError::InvalidImportPath { from, import } => {
                Error::InvalidImportPath { from, import }
            }
            LlvmError::InvalidExportsObject { path } => Error::InvalidExportsObject { path },
            LlvmError::InvalidExportValue { path, export } => {
                Error::InvalidExportValue { path, export }
            }
            other => Error::Backend(other),
        }
    }
}
