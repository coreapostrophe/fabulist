use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const SIMPLE_STORY: &str = r##"
Story { start:  "dialogue_1" }

# dialogue_1
[Jose]
> "What's up"
    - "The ceiling." {
        next: () => {
            goto dialogue_2;
        }
    }
    - "Nothing much." {
        next:  () => {
            goto dialogue_2;
        }
    }
"##;

pub const COMPLEX_STORY: &str = r##"
Story { start:  "part_1" }

# part_1
[Hero]
> "Hello there!" {
    mood: "happy",
    next: () => {
        let x = 10;
        let y = 20;
        context.total = x + y;
        goto part_2;
    }
}
    - "Hi!" {
        next: () => { goto part_2; }
    }
    - "Who are you?" {
        next: () => { goto part_3; }
    }

# part_2
[Villain]
> "I've been expecting you."

# part_3
[Hero]
> "I don't trust you."
"##;

pub fn temp_case_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("{name}-{nonce}"))
}

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root should exist")
        .to_path_buf()
}
