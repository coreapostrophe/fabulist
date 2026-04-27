use std::{
    env, fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn build_command_emits_runnable_standalone_executable() {
    let root = temp_case_dir("fabulate_build_smoke");
    fs::create_dir_all(&root).expect("create temp dir");

    let entry = root.join("story.fab");
    let output = root.join("story-app");

    fs::write(
        &entry,
        r#"
        Story { start: "intro" }

        # intro
        [Guide]
        > "CLI standalone works"
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
        .arg("build")
        .arg(&entry)
        .arg("--module-name")
        .arg("fabulate_build_smoke")
        .arg("--output")
        .arg(&output)
        .output()
        .expect("run fabulate build");

    assert!(
        cli_output.status.success(),
        "fabulate build failed: stdout={} stderr={}",
        String::from_utf8_lossy(&cli_output.stdout),
        String::from_utf8_lossy(&cli_output.stderr)
    );
    assert!(
        output.exists(),
        "expected executable at {}",
        output.display()
    );

    let stdout = String::from_utf8_lossy(&cli_output.stdout);
    assert!(stdout.contains("Wrote standalone executable"));

    let mut child = Command::new(&output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn built standalone executable");
    child
        .stdin
        .as_mut()
        .expect("standalone stdin")
        .write_all(b"1\n")
        .expect("write standalone choice");

    let output = child
        .wait_with_output()
        .expect("wait for standalone executable");
    assert!(
        output.status.success(),
        "standalone executable failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[Guide] CLI standalone works"));
    assert!(stdout.contains("1. Continue"));
    assert!(stdout.contains("Done"));
}

fn fabulate_bin() -> String {
    env!("CARGO_BIN_EXE_fabulate").to_owned()
}

fn temp_case_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!("{name}-{nonce}"))
}
