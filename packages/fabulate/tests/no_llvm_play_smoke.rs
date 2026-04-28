use std::{env, fs, process::Command};

use fabc_reg_test::{temp_case_dir, workspace_root};

#[test]
fn no_llvm_cli_can_play_bundles_but_not_compile_them() {
    let root = temp_case_dir("fabulate_no_llvm_smoke");
    fs::create_dir_all(&root).expect("create temp dir");

    let entry = root.join("story.fab");
    let bundle_output = root.join("bundle");
    let target_dir = root.join("no-llvm-target");

    fs::write(
        &entry,
        r#"
        Story { start: "intro" }

        # intro
        * "No LLVM runtime needed"
        "#,
    )
    .expect("write entry story");

    let compile_output = Command::new(fabulate_bin())
        .arg("compile")
        .arg(&entry)
        .arg("--bundle")
        .arg(&bundle_output)
        .arg("--module-name")
        .arg("fabulate_no_llvm_smoke")
        .output()
        .expect("run fabulate compile for no-llvm play");

    assert!(
        compile_output.status.success(),
        "fabulate compile failed: stdout={} stderr={}",
        String::from_utf8_lossy(&compile_output.stdout),
        String::from_utf8_lossy(&compile_output.stderr)
    );

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let play_output = Command::new(&cargo)
        .current_dir(workspace_root())
        .env("CARGO_TARGET_DIR", &target_dir)
        .arg("run")
        .arg("--quiet")
        .arg("-p")
        .arg("fabulate")
        .arg("--no-default-features")
        .arg("--")
        .arg("play")
        .arg(&bundle_output)
        .output()
        .expect("run no-llvm fabulate play");

    assert!(
        play_output.status.success(),
        "no-llvm fabulate play failed: stdout={} stderr={}",
        String::from_utf8_lossy(&play_output.stdout),
        String::from_utf8_lossy(&play_output.stderr)
    );

    let stdout = String::from_utf8_lossy(&play_output.stdout);
    assert!(stdout.contains("No LLVM runtime needed"));
    assert!(stdout.contains("Story finished."));

    let no_llvm_compile_output = Command::new(cargo)
        .current_dir(workspace_root())
        .env("CARGO_TARGET_DIR", &target_dir)
        .arg("run")
        .arg("--quiet")
        .arg("-p")
        .arg("fabulate")
        .arg("--no-default-features")
        .arg("--")
        .arg("compile")
        .arg(&entry)
        .output()
        .expect("run no-llvm fabulate compile");

    assert!(
        !no_llvm_compile_output.status.success(),
        "expected no-llvm fabulate compile to fail: stdout={} stderr={}",
        String::from_utf8_lossy(&no_llvm_compile_output.stdout),
        String::from_utf8_lossy(&no_llvm_compile_output.stderr)
    );

    let stderr = String::from_utf8_lossy(&no_llvm_compile_output.stderr);
    assert!(stderr
        .contains("LLVM code generation requires enabling one of the `llvmXX-0` crate features"));
}

fn fabulate_bin() -> String {
    env!("CARGO_BIN_EXE_fabulate").to_owned()
}
