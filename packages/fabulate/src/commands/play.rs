use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use fabc::{CompiledBundle, StoryEvent};

use crate::error::Result;

#[derive(clap::Args)]
pub struct Play {
    /// Path to a compiled bundle directory or the story.json manifest itself
    pub bundle: PathBuf,
}

impl Play {
    pub fn exec(&self) -> Result<()> {
        let bundle = CompiledBundle::load(&self.bundle)?;
        let mut machine = bundle.story_machine_with_native_fallback()?;
        let mut event = machine.start()?;

        loop {
            match event {
                StoryEvent::Narration(view) => {
                    println!("{}", view.text);
                    prompt_continue()?;
                    event = machine.advance()?;
                }
                StoryEvent::Dialogue(view) => {
                    println!("[{}] {}", view.speaker, view.text);
                    prompt_continue()?;
                    event = machine.advance()?;
                }
                StoryEvent::Selection(selection) => {
                    for (index, choice) in selection.choices.iter().enumerate() {
                        println!("{}. {}", index + 1, choice.text);
                    }

                    let choice = prompt_choice(selection.choices.len())?;
                    event = machine.choose(choice)?;
                }
                StoryEvent::Finished => {
                    println!("Story finished.");
                    return Ok(());
                }
            }
        }
    }
}

fn prompt_continue() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    if !stdin.is_terminal() || !stdout.is_terminal() {
        return Ok(());
    }

    let mut line = String::new();

    write!(stdout, "> ")?;
    stdout.flush()?;
    stdin.read_line(&mut line)?;

    Ok(())
}

fn prompt_choice(choice_count: usize) -> Result<usize> {
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
