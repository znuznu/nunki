use crate::project::Todo;
use anyhow::Result;
use std::io;
use std::io::Write;

pub enum Answer {
    Yes,
    No,
}

pub fn prompt(todo: &Todo) -> Result<Answer> {
    print!(
            "Untracked todo found L{} in {} \n{}\nWould you like to open an issue for it on Github ? [y/N] ",
            todo.line, todo.file_path, todo.content,
        );
    io::stdout().flush().unwrap();

    loop {
        let mut answer = String::new();

        io::stdin().read_line(&mut answer)?;

        return match answer.to_lowercase().trim() {
            "y" => Ok(Answer::Yes),
            "n" | "" => Ok(Answer::No),
            _ => {
                print!("Invalid input. [y/N] ");
                io::stdout().flush().unwrap();

                continue;
            }
        };
    }
}
