use std::{
    fs,
    path::{Path, PathBuf},
};

use fabc_llvm::{
    compile::{CompiledLlvmArtifact, StoryCompiler},
    ir::StoryProgram,
};

use crate::{
    bundle::{CompiledBundleManifest, COMPILED_BUNDLE_FORMAT_VERSION},
    error::{Error, Result},
};

#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub entry: PathBuf,
    pub output: Option<PathBuf>,
    pub module_name: Option<String>,
    pub bundle_output: Option<PathBuf>,
}

impl CompileOptions {
    pub fn new(entry: impl Into<PathBuf>) -> Self {
        Self {
            entry: entry.into(),
            output: None,
            module_name: None,
            bundle_output: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompileBundleArtifact {
    pub directory: PathBuf,
    pub llvm_ir_path: PathBuf,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CompileArtifact {
    pub entry: PathBuf,
    pub output_path: PathBuf,
    pub module_name: String,
    pub llvm_ir: String,
    pub bundle: Option<CompileBundleArtifact>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Compiler;

impl Compiler {
    pub fn compile(entry: impl Into<PathBuf>) -> Result<CompileArtifact> {
        Self::compile_with_options(CompileOptions::new(entry))
    }

    pub fn compile_with_options(options: CompileOptions) -> Result<CompileArtifact> {
        Compiler.run(options)
    }

    pub fn build_program(&self, entry: impl AsRef<Path>) -> Result<StoryProgram> {
        StoryCompiler
            .lower_entry(entry.as_ref())
            .map_err(Error::from)
    }

    pub fn emit_llvm_ir(
        &self,
        entry: impl AsRef<Path>,
        module_name: Option<&str>,
    ) -> Result<String> {
        let module_name = module_name
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| default_module_name(entry.as_ref()));

        StoryCompiler
            .emit_entry_llvm_ir(entry.as_ref(), &module_name)
            .map_err(Error::from)
    }

    pub fn run(&self, options: CompileOptions) -> Result<CompileArtifact> {
        let bundle_output = options.bundle_output.clone();
        let module_name = options
            .module_name
            .unwrap_or_else(|| default_module_name(&options.entry));
        let program = self.build_program(&options.entry)?;
        let compiled = StoryCompiler.emit_program_llvm_artifact(&program, &module_name)?;
        let default_bundle_llvm_path = bundle_output
            .as_ref()
            .map(|directory| directory.join(format!("{module_name}.ll")));
        let output_path = options.output.unwrap_or_else(|| {
            default_bundle_llvm_path
                .clone()
                .unwrap_or_else(|| default_output_path(&options.entry))
        });

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|source| Error::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        fs::write(&output_path, &compiled.llvm_ir).map_err(|source| Error::Io {
            path: output_path.clone(),
            source,
        })?;

        let bundle = if let Some(directory) = bundle_output {
            Some(self.write_bundle(&directory, &module_name, &program, &compiled, &output_path)?)
        } else {
            None
        };

        Ok(CompileArtifact {
            entry: options.entry,
            output_path,
            module_name,
            llvm_ir: compiled.llvm_ir,
            bundle,
        })
    }
    fn write_bundle(
        &self,
        directory: &Path,
        module_name: &str,
        program: &StoryProgram,
        compiled: &CompiledLlvmArtifact,
        output_path: &Path,
    ) -> Result<CompileBundleArtifact> {
        fs::create_dir_all(directory).map_err(|source| Error::Io {
            path: directory.to_path_buf(),
            source,
        })?;

        let llvm_ir_path = directory.join(format!("{module_name}.ll"));
        if llvm_ir_path != output_path {
            fs::write(&llvm_ir_path, &compiled.llvm_ir).map_err(|source| Error::Io {
                path: llvm_ir_path.clone(),
                source,
            })?;
        }

        let manifest_path = directory.join("story.json");
        let manifest = CompiledBundleManifest {
            format_version: COMPILED_BUNDLE_FORMAT_VERSION,
            module_name: module_name.to_string(),
            program: program.clone(),
            function_symbols: compiled.function_symbols.clone(),
        };
        let manifest_bytes =
            serde_json::to_vec_pretty(&manifest).map_err(Error::BundleManifestSerialize)?;
        fs::write(&manifest_path, manifest_bytes).map_err(|source| Error::Io {
            path: manifest_path.clone(),
            source,
        })?;

        Ok(CompileBundleArtifact {
            directory: directory.to_path_buf(),
            llvm_ir_path,
            manifest_path,
        })
    }
}

fn default_output_path(entry: &Path) -> PathBuf {
    entry.with_extension("ll")
}

fn default_module_name(entry: &Path) -> String {
    let stem = entry
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or("story");

    stem.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use fabc_error::kind::{CompileErrorKind, ErrorKind};
    use fabc_llvm::{
        ir::{Expr, Literal, StepSpec, Stmt},
        runtime::StoryEvent,
    };
    use serde_json::Value;

    use super::{CompileOptions, Compiler};
    use crate::{CompiledBundle, Error};

    #[test]
    fn compiles_entry_with_static_imports() {
        let root = temp_case_dir("static_imports");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let imported = root.join("branch.fab");

        fs::write(
            &entry,
            r#"
            module "./branch.fab" as branch;

            Story { start: "intro" }

            # intro
            [guide]
            > "Choose." 
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

        let llvm_ir = Compiler
            .emit_llvm_ir(&entry, Some("entry_story"))
            .expect("emit llvm ir");

        assert!(llvm_ir.contains("fabc_story_start"));
        assert!(llvm_ir.contains("branch.end"));
    }

    #[test]
    fn resolves_module_exports_in_general_expressions() {
        let root = temp_case_dir("module_exports");
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
                label: branch.title,
                next: () => {
                    let target = branch.ending;
                    goto target;
                }
            }
            "#,
        )
        .expect("write entry");

        fs::write(
            &imported,
            r#"
            Story { exports: { ending: outro, title: "Imported Title" } }

            # outro
            * "Imported ending"
            "#,
        )
        .expect("write import");

        let program = Compiler.build_program(&entry).expect("build program");
        let intro_index = program
            .find_part_index("intro")
            .expect("intro part should exist");

        let StepSpec::Selection(selection) = &program.parts[intro_index].steps[0] else {
            panic!("expected selection step");
        };

        assert_eq!(
            selection.choices[0].properties.get("label"),
            Some(&Expr::Literal(Literal::String(
                "Imported Title".to_string()
            )))
        );

        let function = program.function(0).expect("closure should lower");
        assert_eq!(
            function.body.statements[0],
            Stmt::Let {
                name: "target".to_string(),
                initializer: Expr::StoryReference("branch.outro".to_string()),
            }
        );
    }

    #[test]
    fn rejects_non_object_exports_metadata() {
        let root = temp_case_dir("invalid_exports");
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
            Story { exports: 42 }

            # end
            * "Imported ending"
            "#,
        )
        .expect("write import");

        let error = Compiler
            .build_program(&entry)
            .expect_err("non-object exports should fail");

        assert!(matches!(error, Error::InvalidExportsObject { .. }));
    }

    #[test]
    fn rejects_semantic_errors_before_lowering() {
        let root = temp_case_dir("semantic_errors");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");

        fs::write(
            &entry,
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
        .expect("write entry");

        let error = Compiler
            .build_program(&entry)
            .expect_err("semantic error should fail before lowering");

        let Error::SemanticDiagnostics { diagnostics } = error else {
            panic!("expected semantic diagnostics error");
        };

        assert!(diagnostics.iter().any(|diagnostic| matches!(
            diagnostic.kind,
            ErrorKind::Compile(CompileErrorKind::UninitializedVariable)
        )));
    }

    #[test]
    fn writes_compiled_bundle_manifest() {
        let root = temp_case_dir("bundle_manifest");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let bundle_dir = root.join("bundle");

        fs::write(
            &entry,
            r#"
            Story { start: "intro" }

            # intro
            - "Continue" {
                next: () => {
                    goto outro;
                }
            }

            # outro
            * "Done"
            "#,
        )
        .expect("write entry");

        let artifact = Compiler::compile_with_options(CompileOptions {
            entry: entry.clone(),
            output: None,
            module_name: Some("bundle_story".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = artifact.bundle.expect("bundle should be emitted");
        assert_eq!(artifact.output_path, bundle.llvm_ir_path);
        assert!(bundle.manifest_path.exists());

        let manifest = fs::read_to_string(&bundle.manifest_path).expect("read manifest");
        let manifest: Value = serde_json::from_str(&manifest).expect("parse manifest");

        assert_eq!(manifest["format_version"], 1);
        assert_eq!(manifest["module_name"], "bundle_story");
        assert_eq!(manifest["program"]["start_part"], "intro");
        assert_eq!(manifest["function_symbols"]["0"], "fabc_fn_0");
    }

    #[test]
    fn loads_compiled_bundle_and_runs_story_machine() {
        let root = temp_case_dir("bundle_playback");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let bundle_dir = root.join("bundle");

        fs::write(
            &entry,
            r#"
            Story { start: "intro" }

            # intro
            [Guide]
            > "Welcome"
            - "Continue" {
                next: () => {
                    goto outro;
                }
            }

            # outro
            * "Done"
            "#,
        )
        .expect("write entry");

        Compiler::compile_with_options(CompileOptions {
            entry,
            output: None,
            module_name: Some("bundle_playback".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = CompiledBundle::load(&bundle_dir).expect("load bundle");
        let mut machine = bundle.story_machine().expect("build story machine");

        let event = machine.start().expect("start bundle story");
        assert_eq!(
            event,
            StoryEvent::Dialogue(fabc_llvm::runtime::DialogueView {
                speaker: "Guide".to_string(),
                text: "Welcome".to_string(),
                properties: Default::default(),
            })
        );

        let event = machine.advance().expect("reach selection");
        let StoryEvent::Selection(selection) = event else {
            panic!("expected selection event");
        };
        assert_eq!(selection.choices[0].text, "Continue");

        let event = machine.choose(0).expect("resolve choice");
        assert_eq!(
            event,
            StoryEvent::Narration(fabc_llvm::runtime::NarrationView {
                text: "Done".to_string(),
                properties: Default::default(),
            })
        );
    }

    #[test]
    fn falls_back_to_interpreted_story_machine_when_bundle_llvm_is_missing() {
        let root = temp_case_dir("bundle_playback_fallback");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let bundle_dir = root.join("bundle");

        fs::write(
            &entry,
            r#"
            Story { start: "intro" }

            # intro
            * "Fallback works"
            "#,
        )
        .expect("write entry");

        Compiler::compile_with_options(CompileOptions {
            entry,
            output: None,
            module_name: Some("bundle_fallback".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = CompiledBundle::load(&bundle_dir).expect("load bundle");
        fs::remove_file(&bundle.llvm_ir_path).expect("remove llvm ir");

        let mut machine = bundle
            .story_machine_with_native_fallback()
            .expect("build fallback story machine");

        let event = machine.start().expect("start fallback story");
        assert_eq!(
            event,
            StoryEvent::Narration(fabc_llvm::runtime::NarrationView {
                text: "Fallback works".to_string(),
                properties: Default::default(),
            })
        );
    }

    #[test]
    fn loads_compiled_bundle_and_runs_native_story_machine() {
        let root = temp_case_dir("native_bundle_playback");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let bundle_dir = root.join("bundle");

        fs::write(
            &entry,
            r#"
            Story { start: "part_1" }

            # part_1
            [Hero]
            > "Hello there!"
            - "Hi!" {
                next: () => {
                    let x = 10;
                    let y = 20;
                    context.total = x + y;
                    goto part_2;
                }
            }

            # part_2
            [Villain]
            > "I've been expecting you."
            "#,
        )
        .expect("write entry");

        Compiler::compile_with_options(CompileOptions {
            entry,
            output: None,
            module_name: Some("native_bundle_story".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = CompiledBundle::load(&bundle_dir).expect("load bundle");
        let mut machine = bundle
            .native_story_machine()
            .expect("build native story machine");

        let event = machine.start().expect("start native bundle story");
        assert_eq!(
            event,
            StoryEvent::Dialogue(fabc_llvm::runtime::DialogueView {
                speaker: "Hero".to_string(),
                text: "Hello there!".to_string(),
                properties: Default::default(),
            })
        );

        let event = machine.advance().expect("reach selection");
        let StoryEvent::Selection(selection) = event else {
            panic!("expected selection event");
        };
        assert_eq!(selection.choices[0].text, "Hi!");

        let event = machine.choose(0).expect("resolve native choice");
        assert_eq!(
            machine.context_value("total"),
            Some(fabc_llvm::runtime::Value::Number(30.0))
        );
        assert_eq!(
            event,
            StoryEvent::Dialogue(fabc_llvm::runtime::DialogueView {
                speaker: "Villain".to_string(),
                text: "I've been expecting you.".to_string(),
                properties: Default::default(),
            })
        );
    }

    #[test]
    fn native_bundle_playback_propagates_nested_closure_goto() {
        let root = temp_case_dir("native_nested_goto_bundle_playback");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let bundle_dir = root.join("bundle");

        fs::write(
            &entry,
            r#"
            Story { start: "part_1" }

            # part_1
            [Guide]
            > "Choose carefully."
            - "Jump" {
                next: () => {
                    let jump = () => {
                        goto part_2;
                    };

                    jump();
                    context.after_jump = true;
                }
            }

            # part_2
            [Guide]
            > "Nested goto worked."
            "#,
        )
        .expect("write entry");

        Compiler::compile_with_options(CompileOptions {
            entry,
            output: None,
            module_name: Some("native_nested_bundle_story".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = CompiledBundle::load(&bundle_dir).expect("load bundle");
        let mut machine = bundle
            .native_story_machine()
            .expect("build native story machine");

        let event = machine.start().expect("start native bundle story");
        assert_eq!(
            event,
            StoryEvent::Dialogue(fabc_llvm::runtime::DialogueView {
                speaker: "Guide".to_string(),
                text: "Choose carefully.".to_string(),
                properties: Default::default(),
            })
        );

        let event = machine.advance().expect("reach selection");
        let StoryEvent::Selection(selection) = event else {
            panic!("expected selection event");
        };
        assert_eq!(selection.choices[0].text, "Jump");

        let event = machine.choose(0).expect("resolve nested native choice");
        assert_eq!(machine.context_value("after_jump"), None);
        assert_eq!(
            event,
            StoryEvent::Dialogue(fabc_llvm::runtime::DialogueView {
                speaker: "Guide".to_string(),
                text: "Nested goto worked.".to_string(),
                properties: Default::default(),
            })
        );
    }

    fn temp_case_dir(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        env::temp_dir().join(format!("fabc-{name}-{nonce}"))
    }
}
