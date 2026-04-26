use std::{collections::BTreeMap, path::Path};

use fabc_analyzer::Analyzer;
use fabc_parser::{ast::init::Init, Parser};

use crate::{
    error::{Error, Result},
    frontend::lower::Lowerer,
    ir::{FunctionId, StoryProgram},
    link::ModuleLinker,
    runtime::StoryMachine,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct StoryCompiler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledLlvmArtifact {
    pub llvm_ir: String,
    pub function_symbols: BTreeMap<FunctionId, String>,
}

pub fn lower_source(source: &str) -> Result<StoryProgram> {
    StoryCompiler.lower_source(source)
}

pub fn lower_entry(entry: impl AsRef<Path>) -> Result<StoryProgram> {
    StoryCompiler.lower_entry(entry)
}

impl StoryCompiler {
    pub fn lower_inits(&self, inits: Vec<Init>) -> Result<StoryProgram> {
        Lowerer::default().lower_inits(inits)
    }

    pub fn lower_entry(&self, entry: impl AsRef<Path>) -> Result<StoryProgram> {
        let linked_inits = ModuleLinker::default().link_inits(entry.as_ref())?;
        let diagnostics = Analyzer::analyze(&linked_inits).errors;
        if !diagnostics.is_empty() {
            return Err(Error::SemanticDiagnostics { diagnostics });
        }

        self.lower_inits(linked_inits)
    }

    pub fn lower_source(&self, source: &str) -> Result<StoryProgram> {
        let parsed = Parser::parse_str(source);
        if !parsed.errors.is_empty() {
            return Err(Error::Diagnostics(parsed.errors));
        }

        let diagnostics = Analyzer::analyze(&parsed.result).errors;
        if !diagnostics.is_empty() {
            return Err(Error::Diagnostics(diagnostics));
        }

        self.lower_inits(parsed.result)
    }

    pub fn machine_from_entry(&self, entry: impl AsRef<Path>) -> Result<StoryMachine> {
        let program = self.lower_entry(entry)?;
        StoryMachine::new(program)
            .map_err(|runtime_error| Error::RuntimeInitialization(runtime_error.to_string()))
    }

    pub fn machine_from_source(&self, source: &str) -> Result<StoryMachine> {
        let program = self.lower_source(source)?;
        StoryMachine::new(program)
            .map_err(|runtime_error| Error::RuntimeInitialization(runtime_error.to_string()))
    }

    #[allow(unused_variables)]
    pub fn emit_llvm_ir(&self, source: &str, module_name: &str) -> Result<String> {
        let program = self.lower_source(source)?;
        self.emit_program_llvm_ir(&program, module_name)
    }

    pub fn emit_entry_llvm_ir(&self, entry: impl AsRef<Path>, module_name: &str) -> Result<String> {
        let program = self.lower_entry(entry)?;
        self.emit_program_llvm_ir(&program, module_name)
    }

    pub fn emit_entry_object_file(
        &self,
        entry: impl AsRef<Path>,
        module_name: &str,
        output: impl AsRef<Path>,
    ) -> Result<CompiledLlvmArtifact> {
        let program = self.lower_entry(entry)?;
        self.emit_program_object_file(&program, module_name, output)
    }

    #[allow(unused_variables)]
    pub fn emit_program_llvm_ir(
        &self,
        program: &StoryProgram,
        module_name: &str,
    ) -> Result<String> {
        Ok(self
            .emit_program_llvm_artifact(program, module_name)?
            .llvm_ir)
    }

    #[allow(unused_variables)]
    pub fn emit_program_llvm_artifact(
        &self,
        program: &StoryProgram,
        module_name: &str,
    ) -> Result<CompiledLlvmArtifact> {
        self.emit_program_llvm_artifact_with_options(program, module_name, None::<&Path>)
    }

    #[allow(unused_variables)]
    pub fn emit_program_object_file(
        &self,
        program: &StoryProgram,
        module_name: &str,
        output: impl AsRef<Path>,
    ) -> Result<CompiledLlvmArtifact> {
        self.emit_program_llvm_artifact_with_options(program, module_name, Some(output.as_ref()))
    }

    #[allow(unused_variables)]
    fn emit_program_llvm_artifact_with_options(
        &self,
        program: &StoryProgram,
        module_name: &str,
        object_output: Option<&Path>,
    ) -> Result<CompiledLlvmArtifact> {
        let program = program.clone();

        #[cfg(feature = "llvm-backend")]
        {
            use inkwell::context::Context;
            use inkwell::targets::{
                CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
            };
            use inkwell::OptimizationLevel;

            let context = Context::create();
            let artifact = crate::llvm::LlvmEmitter::new(&context, module_name)?.emit(&program)?;

            if let Some(output_path) = object_output {
                let output_path = output_path.to_path_buf();
                Target::initialize_native(&InitializationConfig::default())
                    .map_err(|error| Error::TargetInitialization(error.to_string()))?;

                let triple = TargetMachine::get_default_triple();
                let target = Target::from_triple(&triple)
                    .map_err(|error| Error::TargetMachine(error.to_string()))?;
                let cpu = TargetMachine::get_host_cpu_name();
                let features = TargetMachine::get_host_cpu_features();
                let target_machine = target
                    .create_target_machine(
                        &triple,
                        cpu.to_str().map_err(|error| Error::TargetMachine(error.to_string()))?,
                        features
                            .to_str()
                            .map_err(|error| Error::TargetMachine(error.to_string()))?,
                        OptimizationLevel::Default,
                        RelocMode::PIC,
                        CodeModel::Default,
                    )
                    .ok_or_else(|| {
                        Error::TargetMachine(
                            "LLVM could not create a native target machine".to_string(),
                        )
                    })?;

                artifact.module.set_triple(&triple);
                let data_layout = target_machine.get_target_data().get_data_layout();
                artifact.module.set_data_layout(&data_layout);
                target_machine
                    .write_to_file(&artifact.module, FileType::Object, &output_path)
                    .map_err(|error| Error::ObjectWrite {
                        path: output_path,
                        message: error.to_string(),
                    })?;
            }

            Ok(CompiledLlvmArtifact {
                llvm_ir: artifact.module.print_to_string().to_string(),
                function_symbols: artifact.function_symbols,
            })
        }

        #[cfg(not(feature = "llvm-backend"))]
        {
            let _ = (program, module_name, object_output);
            Err(Error::LlvmFeatureDisabled)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use fabc_error::kind::{CompileErrorKind, ErrorKind};

    use super::{Error, StoryCompiler};

    #[test]
    fn lower_entry_resolves_static_imports() {
        let root = temp_case_dir("llvm_static_imports");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let imported = root.join("branch.fab");

        fs::write(
            &entry,
            r#"
            module "./branch.fab" as branch;

            Story { start: "intro" }

            # intro
            - "Continue" {
                next: () => {
                    goto branch.end;
                }
            }
            "#,
        )
        .expect("write entry");

        fs::write(
            &imported,
            r#"
            Story {}

            # end
            * "Imported ending"
            "#,
        )
        .expect("write import");

        let program = StoryCompiler
            .lower_entry(&entry)
            .expect("entry lowering should resolve imports");

        assert_eq!(program.start_part, "intro");
        assert!(program.find_part_index("branch.end").is_some());
    }

    #[test]
    fn lower_source_rejects_semantic_errors() {
        let error = StoryCompiler
            .lower_source(
                r#"
                Story { start: "intro" }

                # intro
                - "Continue" {
                    next: () => {
                        let target = missing;
                        goto target;
                    }
                }
                "#,
            )
            .expect_err("semantic errors should fail before lowering");

        let Error::Diagnostics(diagnostics) = error else {
            panic!("expected diagnostics error");
        };

        assert!(diagnostics.iter().any(|diagnostic| matches!(
            diagnostic.kind,
            ErrorKind::Compile(CompileErrorKind::UninitializedVariable)
        )));
    }

    fn temp_case_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        env::temp_dir().join(format!("fabc-llvm-{name}-{nonce}"))
    }
}
