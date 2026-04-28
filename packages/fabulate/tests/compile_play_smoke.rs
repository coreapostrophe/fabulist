use std::{
    env, fs,
    io::Write,
    process::{Command, Stdio},
};

use fabc_reg_test::temp_case_dir;

#[test]
fn compile_command_emits_bundle_and_object_outputs() {
    let root = temp_case_dir("fabulate_compile_smoke");
    fs::create_dir_all(&root).expect("create temp dir");

    let entry = root.join("story.fab");
    let llvm_output = root.join("artifacts/story.ll");
    let object_output = root.join("artifacts/story.o");
    let bundle_output = root.join("bundle");
    let bundled_llvm_output = bundle_output.join("fabulate_compile_smoke.ll");
    let manifest_path = bundle_output.join("story.json");

    fs::write(
        &entry,
        r#"
        Story { start: "intro" }

        # intro
        [Guide]
        > "CLI compile works"
        - "Continue" {
            next: () => {
                context.total = 4 + 5;
                goto outro;
            }
        }

        # outro
        * "Done"
        "#,
    )
    .expect("write entry story");

    let cli_output = Command::new(fabulate_bin())
        .arg("compile")
        .arg(&entry)
        .arg("--output")
        .arg(&llvm_output)
        .arg("--object-output")
        .arg(&object_output)
        .arg("--bundle")
        .arg(&bundle_output)
        .arg("--module-name")
        .arg("fabulate_compile_smoke")
        .output()
        .expect("run fabulate compile");

    assert!(
        cli_output.status.success(),
        "fabulate compile failed: stdout={} stderr={}",
        String::from_utf8_lossy(&cli_output.stdout),
        String::from_utf8_lossy(&cli_output.stderr)
    );
    assert!(
        llvm_output.exists(),
        "expected llvm ir at {}",
        llvm_output.display()
    );
    assert!(
        object_output.exists(),
        "expected object file at {}",
        object_output.display()
    );
    assert!(
        bundled_llvm_output.exists(),
        "expected bundled llvm ir at {}",
        bundled_llvm_output.display()
    );
    assert!(
        manifest_path.exists(),
        "expected bundle manifest at {}",
        manifest_path.display()
    );
    assert!(
        fs::metadata(&object_output)
            .expect("object output metadata")
            .len()
            > 0,
        "expected object output to be non-empty"
    );

    let stdout = String::from_utf8_lossy(&cli_output.stdout);
    assert!(stdout.contains("Wrote LLVM IR"));
    assert!(stdout.contains("Wrote native object file"));
    assert!(stdout.contains("Wrote compiled bundle manifest"));

    let manifest = fs::read_to_string(&manifest_path).expect("read bundle manifest");
    assert!(manifest.contains("\"format_version\": 1"));
    assert!(manifest.contains("\"module_name\": \"fabulate_compile_smoke\""));
}

#[test]
fn play_command_runs_compiled_bundle() {
    let root = temp_case_dir("fabulate_play_smoke");
    fs::create_dir_all(&root).expect("create temp dir");

    let entry = root.join("story.fab");
    let bundle_output = root.join("bundle");

    fs::write(
        &entry,
        r#"
        Story { start: "intro" }

        # intro
        [Guide]
        > "CLI play works"
        - "Continue" {
            next: () => {
                goto outro;
            }
        }

        # outro
        * "Done"
        "#,
    )
    .expect("write entry story");

    let compile_output = Command::new(fabulate_bin())
        .arg("compile")
        .arg(&entry)
        .arg("--bundle")
        .arg(&bundle_output)
        .arg("--module-name")
        .arg("fabulate_play_smoke")
        .output()
        .expect("run fabulate compile for play");

    assert!(
        compile_output.status.success(),
        "fabulate compile failed: stdout={} stderr={}",
        String::from_utf8_lossy(&compile_output.stdout),
        String::from_utf8_lossy(&compile_output.stderr)
    );

    let mut child = Command::new(fabulate_bin())
        .arg("play")
        .arg(&bundle_output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn fabulate play");
    child
        .stdin
        .as_mut()
        .expect("play stdin")
        .write_all(b"1\n")
        .expect("write play choice");

    let output = child.wait_with_output().expect("wait for fabulate play");
    assert!(
        output.status.success(),
        "fabulate play failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[Guide] CLI play works"));
    assert!(stdout.contains("1. Continue"));
    assert!(stdout.contains("Done"));
    assert!(stdout.contains("Story finished."));
}

fn fabulate_bin() -> String {
    env!("CARGO_BIN_EXE_fabulate").to_owned()
}
