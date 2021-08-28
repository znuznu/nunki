use anyhow::Result;
use clap::arg_enum;
use std::io;
use std::io::Write;
use todo::Todo;
use walkdir::WalkDir;

use line_iterator::LineIterator;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub mod line_iterator;
pub mod regex;
pub mod todo;

enum Answer {
    Yes,
    No,
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Mode {
        Match,
        Patch,
    }
}

pub struct Project<'a> {
    mode: Mode,
    entrypoint: &'a str, // keywords: &'a [&'a str],
}

impl<'a> Project<'a> {
    pub fn from<T: AsRef<str>>(
        mode: Mode,
        entrypoint: &'a T, /* keywords: &'a [&str] */
    ) -> Self {
        Project {
            mode,
            entrypoint: entrypoint.as_ref(),
            // keywords,
        }
    }

    pub fn exec(&self) -> Result<()> {
        self.walk()
    }

    /// Walk recursively through the provided path and find TODOs in each file.
    pub fn walk(&self) -> Result<()> {
        let mut todos: Vec<Todo> = Vec::new();

        for entry in WalkDir::new(self.entrypoint)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.path().is_file() {
                continue;
            }

            match &self.mode {
                Mode::Match => {
                    let matches = self.match_file(entry.path().to_str().unwrap())?;
                    todos.extend(matches);
                }
                Mode::Patch => self.patch_file(entry.path().to_str().unwrap())?,
            }
        }

        println!("Total {}", todos.len());

        for todo in todos.into_iter() {
            println!("\n{}", todo);
        }

        Ok(())
    }

    pub fn match_file(&self, file_path: &str) -> Result<Vec<Todo>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut todos: Vec<Todo> = Vec::new();
        let mut line_count = 0;

        for line in reader.lines() {
            line_count += 1;

            if let Some(content) = regex::extract_untracked_todo_content(&line.unwrap()) {
                let todo: Todo =
                    Todo::from(None, file_path.to_string(), line_count, content.to_string());
                todos.push(todo);
            }
        }

        Ok(todos)
    }

    pub fn patch_file(&self, file_path: &str) -> Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let tmp_file = File::create(format!("{}.tmp", file_path))?;
        let mut writer = BufWriter::new(tmp_file);

        for (index, chunk) in LineIterator::new(reader).enumerate() {
            let chunk = chunk?;
            let line = std::str::from_utf8(&chunk)?;

            if let Some(content) = regex::extract_untracked_todo_content(&line) {
                let todo: Todo =
                    Todo::from(None, file_path.to_string(), index + 1, content.to_string());

                // Maybe put it below with the answer
                println!();

                match self.prompt(todo)? {
                    Answer::Yes => {
                        let patched_line = regex::replace_untracked_todo(&line, 100);
                        writer.write(&patched_line.as_bytes())?;
                    }
                    Answer::No => {
                        writer.write(&line.as_bytes())?;
                    }
                }
            } else {
                writer.write(&line.as_bytes())?;
            }
        }

        writer.flush()?;

        Ok(())
    }

    // FIXME probably gonna messed up when the multi-threading is implemented later on
    fn prompt(&self, todo: Todo) -> Result<Answer> {
        print!(
            "Untracked todo found L{} in {} \nWould you like to open an issue for it on Github ? [y/N] ",
            todo.line, todo.file_path,
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
                    continue;
                }
            };
        }
    }
}
