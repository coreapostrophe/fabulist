use std::{
    collections::BTreeMap,
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
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
    pub object_output: Option<PathBuf>,
    pub module_name: Option<String>,
    pub bundle_output: Option<PathBuf>,
}

impl CompileOptions {
    pub fn new(entry: impl Into<PathBuf>) -> Self {
        Self {
            entry: entry.into(),
            output: None,
            object_output: None,
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
    pub object_path: Option<PathBuf>,
    pub module_name: String,
    pub llvm_ir: String,
    pub bundle: Option<CompileBundleArtifact>,
}

#[derive(Debug, Clone)]
pub struct ExecutableOptions {
    pub entry: PathBuf,
    pub output: Option<PathBuf>,
    pub module_name: Option<String>,
    pub release: bool,
}

impl ExecutableOptions {
    pub fn new(entry: impl Into<PathBuf>) -> Self {
        Self {
            entry: entry.into(),
            output: None,
            module_name: None,
            release: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutableArtifact {
    pub entry: PathBuf,
    pub output_path: PathBuf,
    pub module_name: String,
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

    pub fn build_executable(entry: impl Into<PathBuf>) -> Result<ExecutableArtifact> {
        Self::build_executable_with_options(ExecutableOptions::new(entry))
    }

    pub fn build_executable_with_options(options: ExecutableOptions) -> Result<ExecutableArtifact> {
        Compiler.run_executable(options)
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
        let object_output = options.object_output.clone();
        let compiled = if let Some(object_output) = object_output.as_ref() {
            if let Some(parent) = object_output.parent() {
                fs::create_dir_all(parent).map_err(|source| Error::Io {
                    path: parent.to_path_buf(),
                    source,
                })?;
            }

            StoryCompiler.emit_program_object_file(&program, &module_name, object_output)?
        } else {
            StoryCompiler.emit_program_llvm_artifact(&program, &module_name)?
        };
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
            object_path: object_output,
            module_name,
            llvm_ir: compiled.llvm_ir,
            bundle,
        })
    }

    pub fn run_executable(&self, options: ExecutableOptions) -> Result<ExecutableArtifact> {
        let module_name = options
            .module_name
            .unwrap_or_else(|| default_module_name(&options.entry));
        let program = self.build_program(&options.entry)?;
        let story_json =
            serde_json::to_string_pretty(&program).map_err(Error::StandaloneStorySerialize)?;
        let output_path = options
            .output
            .unwrap_or_else(|| default_executable_output_path(&options.entry));

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|source| Error::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let launcher_dir = standalone_launcher_dir(&module_name);
        fs::create_dir_all(launcher_dir.join("src")).map_err(|source| Error::Io {
            path: launcher_dir.join("src"),
            source,
        })?;

        let launcher_manifest_path = launcher_dir.join("Cargo.toml");
        let launcher_build_path = launcher_dir.join("build.rs");
        let launcher_story_path = launcher_dir.join("story.json");
        let launcher_object_path = launcher_dir.join("story.o");
        let launcher_main_path = launcher_dir.join("src/main.rs");
        let launcher_functions_path = launcher_dir.join("src/linked_functions.rs");

        let compiled = StoryCompiler.emit_program_object_file(
            &program,
            &module_name,
            &launcher_object_path,
        )?;

        fs::write(
            &launcher_manifest_path,
            standalone_launcher_manifest(&module_name),
        )
        .map_err(|source| Error::Io {
            path: launcher_manifest_path.clone(),
            source,
        })?;
        fs::write(&launcher_build_path, standalone_launcher_build()).map_err(|source| {
            Error::Io {
                path: launcher_build_path.clone(),
                source,
            }
        })?;
        fs::write(&launcher_story_path, story_json).map_err(|source| Error::Io {
            path: launcher_story_path.clone(),
            source,
        })?;
        fs::write(&launcher_main_path, standalone_launcher_main()).map_err(|source| Error::Io {
            path: launcher_main_path.clone(),
            source,
        })?;
        fs::write(
            &launcher_functions_path,
            standalone_launcher_linked_functions(&program, &compiled.function_symbols)?,
        )
        .map_err(|source| Error::Io {
            path: launcher_functions_path.clone(),
            source,
        })?;

        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let launcher_target_dir = launcher_dir.join("target");
        let mut command = Command::new(&cargo);
        command.arg("build");
        if options.release {
            command.arg("--release");
        }
        command.current_dir(&launcher_dir);
        command.env("CARGO_TARGET_DIR", &launcher_target_dir);

        let build_output = command.output().map_err(|source| Error::Io {
            path: launcher_dir.clone(),
            source,
        })?;
        if !build_output.status.success() {
            return Err(Error::StandaloneBuildCommand {
                command: format!(
                    "{cargo} build{}",
                    if options.release { " --release" } else { "" }
                ),
                status: build_output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&build_output.stderr)
                    .trim()
                    .to_string(),
            });
        }

        let binary_name = format!("fabc-standalone-{module_name}{}", env::consts::EXE_SUFFIX);
        let built_binary_path = launcher_target_dir
            .join(if options.release { "release" } else { "debug" })
            .join(&binary_name);
        fs::copy(&built_binary_path, &output_path).map_err(|source| Error::Io {
            path: built_binary_path.clone(),
            source,
        })?;

        Ok(ExecutableArtifact {
            entry: options.entry,
            output_path,
            module_name,
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

fn default_executable_output_path(entry: &Path) -> PathBuf {
    let name = format!("{}{}", default_module_name(entry), env::consts::EXE_SUFFIX);
    entry.with_file_name(name)
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

fn standalone_launcher_dir(module_name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!("fabc-standalone-{module_name}-{nonce}"))
}

fn standalone_launcher_manifest(module_name: &str) -> String {
    let compiler_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("compiler directory should exist");
    let fabc_ir_path = compiler_dir.join("fabc_ir");
    let fabc_rt_path = compiler_dir.join("fabc_rt");

    format!(
        r#"[package]
name = "fabc-standalone-{module_name}"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]
fabc_ir = {{ path = "{}" }}
fabc_rt = {{ path = "{}" }}
serde_json = "1.0.145"
"#,
        fabc_ir_path.display(),
        fabc_rt_path.display(),
    )
}

fn standalone_launcher_build() -> &'static str {
    r#"use std::path::Path;

fn main() {
    let object_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("story.o");
    println!("cargo:rerun-if-changed={}", object_path.display());
    println!("cargo:rustc-link-arg={}", object_path.display());
}
"#
}

fn standalone_launcher_main() -> &'static str {
    r#"use std::{
    error::Error as StdError,
    io::{self, Write},
    process::ExitCode,
    rc::Rc,
};

use fabc_ir::StoryProgram;
use fabc_rt::{LinkedCompiledFunctionHost, StoryEvent, StoryMachine};

mod linked_functions;

use linked_functions::FUNCTIONS;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn StdError>> {
    let program: StoryProgram = serde_json::from_str(include_str!("../story.json"))?;
    let compiled_host = Rc::new(LinkedCompiledFunctionHost::new(FUNCTIONS));
    let mut machine = StoryMachine::with_compiled_executor(program, Default::default(), compiled_host)?;
    let mut event = machine.start()?;

    loop {
        match event {
            StoryEvent::Narration(view) => {
                println!("{}", view.text);
                event = machine.advance()?;
            }
            StoryEvent::Dialogue(view) => {
                println!("[{}] {}", view.speaker, view.text);
                event = machine.advance()?;
            }
            StoryEvent::Selection(selection) => {
                for (index, choice) in selection.choices.iter().enumerate() {
                    println!("{}. {}", index + 1, choice.text);
                }

                let choice = prompt_choice(selection.choices.len())?;
                event = machine.choose(choice)?;
            }
            StoryEvent::Finished => return Ok(()),
        }
    }
}

fn prompt_choice(choice_count: usize) -> io::Result<usize> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        line.clear();
        stdin.read_line(&mut line)?;

        let trimmed = line.trim();
        let Ok(index) = trimmed.parse::<usize>() else {
            eprintln!("Enter a number between 1 and {choice_count}.");
            continue;
        };

        if (1..=choice_count).contains(&index) {
            return Ok(index - 1);
        }

        eprintln!("Enter a number between 1 and {choice_count}.");
    }
}
"#
}

fn standalone_launcher_linked_functions(
    program: &StoryProgram,
    function_symbols: &BTreeMap<usize, String>,
) -> Result<String> {
    let mut code = String::from(
        "use std::ffi::c_void;\n\nuse fabc_rt::{CompiledClosureFn, LinkedFunctionDescriptor};\n\n",
    );

    for function in &program.functions {
        let symbol = function_symbols.get(&function.id).ok_or_else(|| {
            Error::from(fabc_llvm::Error::Codegen(format!(
                "missing compiled symbol for closure {}",
                function.id
            )))
        })?;

        code.push_str("unsafe extern \"C\" {\n");
        let _ = writeln!(
            code,
            "    #[link_name = {symbol:?}]\n    fn linked_fabc_fn_{id}(frame: *mut c_void, context: *mut c_void) -> *mut c_void;",
            symbol = symbol,
            id = function.id,
        );
        code.push_str("}\n\n");

        let params = function
            .params
            .iter()
            .map(|param| format!("{param:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            code,
            "static PARAMS_{id}: &[&str] = &[{params}];",
            id = function.id,
            params = params,
        );
    }

    code.push_str("\npub static FUNCTIONS: &[LinkedFunctionDescriptor] = &[\n");
    for function in &program.functions {
        let symbol = function_symbols.get(&function.id).ok_or_else(|| {
            Error::from(fabc_llvm::Error::Codegen(format!(
                "missing compiled symbol for closure {}",
                function.id
            )))
        })?;
        let _ = writeln!(
            code,
            "    LinkedFunctionDescriptor {{ id: {id}, symbol: {symbol:?}, params: PARAMS_{id}, function: linked_fabc_fn_{id} as CompiledClosureFn }},",
            id = function.id,
            symbol = symbol,
        );
    }
    code.push_str("];\n");

    Ok(code)
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::PathBuf,
        process::{Command, Stdio},
        time::{SystemTime, UNIX_EPOCH},
    };

    use fabc_error::kind::{CompileErrorKind, ErrorKind};
    use fabc_llvm::{
        ir::{Expr, Literal, StepSpec, Stmt},
        runtime::{DialogueView, NarrationView, StoryEvent},
    };
    use serde_json::Value;

    use super::{CompileOptions, Compiler, ExecutableOptions};
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
    fn emits_native_object_file_when_requested() {
        let root = temp_case_dir("object_output");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let llvm_ir_path = root.join("story.ll");
        let object_path = root.join("story.o");

        fs::write(
            &entry,
            r#"
            Story { start: "intro" }

            # intro
            * "Standalone next step"
            "#,
        )
        .expect("write entry");

        let artifact = Compiler::compile_with_options(CompileOptions {
            entry,
            output: Some(llvm_ir_path.clone()),
            object_output: Some(object_path.clone()),
            module_name: Some("object_story".to_string()),
            bundle_output: None,
        })
        .expect("emit object file");

        assert_eq!(artifact.output_path, llvm_ir_path);
        assert_eq!(artifact.object_path, Some(object_path.clone()));
        assert!(artifact.output_path.exists());
        assert!(object_path.exists());

        let metadata = fs::metadata(&object_path).expect("object file metadata");
        assert!(metadata.len() > 0);
    }

    #[test]
    fn builds_standalone_executable_and_runs_it() {
        let root = temp_case_dir("standalone_build");
        fs::create_dir_all(&root).expect("create temp dir");

        let entry = root.join("entry.fab");
        let executable_path = root.join(format!("story{}", env::consts::EXE_SUFFIX));

        fs::write(
            &entry,
            r#"
            Story { start: "intro" }

            # intro
            [Guide]
            > "Linked launcher works"
            - "Continue" {
                next: () => {
                    context.total = 7 + 8;
                    goto outro;
                }
            }

            # outro
            * "Done"
            "#,
        )
        .expect("write entry");

        let artifact = Compiler::build_executable_with_options(ExecutableOptions {
            entry,
            output: Some(executable_path.clone()),
            module_name: Some("standalone_story".to_string()),
            release: false,
        })
        .expect("build standalone executable");

        assert_eq!(artifact.output_path, executable_path);
        assert!(artifact.output_path.exists());

        let mut child = Command::new(&artifact.output_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn standalone executable");
        use std::io::Write as _;
        child
            .stdin
            .as_mut()
            .expect("standalone stdin")
            .write_all(b"1\n")
            .expect("write choice");
        let output = child.wait_with_output().expect("run standalone executable");
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("[Guide] Linked launcher works"));
        assert!(stdout.contains("1. Continue"));
        assert!(stdout.contains("Done"));
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
            object_output: None,
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
            object_output: None,
            module_name: Some("bundle_playback".to_string()),
            bundle_output: Some(bundle_dir.clone()),
        })
        .expect("compile bundle");

        let bundle = CompiledBundle::load(&bundle_dir).expect("load bundle");
        let mut machine = bundle.story_machine().expect("build story machine");

        let event = machine.start().expect("start bundle story");
        assert_eq!(
            event,
            StoryEvent::Dialogue(DialogueView {
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
            StoryEvent::Narration(NarrationView {
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
            object_output: None,
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
            StoryEvent::Narration(NarrationView {
                text: "Fallback works".to_string(),
                properties: Default::default(),
            })
        );
    }

    fn temp_case_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        env::temp_dir().join(format!("fabc-{name}-{nonce}"))
    }
}
