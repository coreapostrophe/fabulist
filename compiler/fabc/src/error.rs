use std::{io, path::PathBuf};

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
    BundleManifestSerialize(#[source] serde_json::Error),
    #[error("failed to parse compiled story bundle manifest `{path}`: {source}")]
    BundleManifestParse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("unsupported bundle format version {found} in `{path}`")]
    UnsupportedBundleFormatVersion { path: PathBuf, found: u32 },
    #[error("compiled bundle `{path}` is missing a function symbol for closure {function_id}")]
    MissingBundleFunctionSymbol { path: PathBuf, function_id: usize },
    #[error("failed to initialize story machine from compiled bundle: {0}")]
    BundleRuntimeInitialization(#[source] fabc_llvm::runtime::RuntimeError),
    #[error(transparent)]
    Backend(fabc_llvm::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<fabc_llvm::Error> for Error {
    fn from(error: fabc_llvm::Error) -> Self {
        match error {
            fabc_llvm::Error::Io { path, source } => Error::Io { path, source },
            fabc_llvm::Error::ParseDiagnostics { path, diagnostics } => {
                Error::ParseDiagnostics { path, diagnostics }
            }
            fabc_llvm::Error::SemanticDiagnostics { diagnostics } => {
                Error::SemanticDiagnostics { diagnostics }
            }
            fabc_llvm::Error::MissingStory { path } => Error::MissingStory { path },
            fabc_llvm::Error::MultipleStories { path, count } => {
                Error::MultipleStories { path, count }
            }
            fabc_llvm::Error::CircularImport { chain } => Error::CircularImport { chain },
            fabc_llvm::Error::DuplicatePart {
                part,
                first,
                second,
            } => Error::DuplicatePart {
                part,
                first,
                second,
            },
            fabc_llvm::Error::InvalidImportPath { from, import } => {
                Error::InvalidImportPath { from, import }
            }
            fabc_llvm::Error::InvalidExportsObject { path } => Error::InvalidExportsObject { path },
            fabc_llvm::Error::InvalidExportValue { path, export } => {
                Error::InvalidExportValue { path, export }
            }
            other => Error::Backend(other),
        }
    }
}
