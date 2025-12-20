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
