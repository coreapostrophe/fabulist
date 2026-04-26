use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use fabc_rt::StoryMachine;

use fabc_llvm::ir::{FunctionId, StoryProgram};

#[cfg(feature = "llvm-backend")]
use fabc_llvm::runtime::NativeClosureHost;

#[cfg(feature = "llvm-backend")]
use fabc_rt::Value;

#[cfg(not(feature = "llvm-backend"))]
use fabc_llvm::Error as LlvmError;

use crate::{error::Error, Result};

pub const COMPILED_BUNDLE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompiledBundleManifest {
    pub format_version: u32,
    pub module_name: String,
    pub program: StoryProgram,
    pub function_symbols: BTreeMap<FunctionId, String>,
}

#[derive(Debug, Clone)]
pub struct CompiledBundle {
    pub directory: PathBuf,
    pub manifest_path: PathBuf,
    pub llvm_ir_path: PathBuf,
    pub manifest: CompiledBundleManifest,
}

impl CompiledBundle {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let manifest_path = resolve_manifest_path(path.as_ref());
        let manifest_bytes = fs::read(&manifest_path).map_err(|source| Error::Io {
            path: manifest_path.clone(),
            source,
        })?;
        let manifest: CompiledBundleManifest =
            serde_json::from_slice(&manifest_bytes).map_err(|source| {
                Error::BundleManifestParse {
                    path: manifest_path.clone(),
                    source,
                }
            })?;

        if manifest.format_version != COMPILED_BUNDLE_FORMAT_VERSION {
            return Err(Error::UnsupportedBundleFormatVersion {
                path: manifest_path.clone(),
                found: manifest.format_version,
            });
        }

        for function in &manifest.program.functions {
            if !manifest.function_symbols.contains_key(&function.id) {
                return Err(Error::MissingBundleFunctionSymbol {
                    path: manifest_path.clone(),
                    function_id: function.id,
                });
            }
        }

        let directory = manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let llvm_ir_path = directory.join(format!("{}.ll", manifest.module_name));

        Ok(Self {
            directory,
            manifest_path,
            llvm_ir_path,
            manifest,
        })
    }

    pub fn story_machine(&self) -> Result<StoryMachine> {
        StoryMachine::new(self.manifest.program.clone()).map_err(Error::BundleRuntimeInitialization)
    }

    pub fn story_machine_with_native_fallback(&self) -> Result<StoryMachine> {
        match self.native_story_machine() {
            Ok(machine) => Ok(machine),
            Err(error)
                if matches!(
                    &error,
                    Error::Io { .. } | Error::Backend(_) | Error::BundleRuntimeInitialization(_)
                ) =>
            {
                self.story_machine()
            }
            Err(error) => Err(error),
        }
    }

    #[cfg(feature = "llvm-backend")]
    pub fn native_story_machine(&self) -> Result<StoryMachine> {
        let llvm_ir = fs::read_to_string(&self.llvm_ir_path).map_err(|source| Error::Io {
            path: self.llvm_ir_path.clone(),
            source,
        })?;
        let native_host = NativeClosureHost::from_llvm_ir(
            &llvm_ir,
            &self.manifest.program,
            &self.manifest.function_symbols,
            &self.manifest.module_name,
        )
        .map_err(Error::Backend)?;

        StoryMachine::with_native_executor(
            self.manifest.program.clone(),
            BTreeMap::<String, Value>::new(),
            native_host,
        )
        .map_err(Error::BundleRuntimeInitialization)
    }

    #[cfg(not(feature = "llvm-backend"))]
    pub fn native_story_machine(&self) -> Result<StoryMachine> {
        let _ = self;
        Err(Error::from(LlvmError::LlvmFeatureDisabled))
    }
}

fn resolve_manifest_path(path: &Path) -> PathBuf {
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        path.to_path_buf()
    } else {
        path.join("story.json")
    }
}
