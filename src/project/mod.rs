use anyhow::Result;
use clap::arg_enum;
use std::io;
use std::io::Write;
use todo::Todo;
use walkdir::WalkDir;

use crate::git::GitPlatform;

use line_iterator::LineIterator;
use std::fs::{rename, File};
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
    entrypoint: &'a str,
    git_platform: Box<dyn GitPlatform<'a> + 'a>,
}

impl<'a> Project<'a> {
    pub fn from<T: AsRef<str>>(
        mode: Mode,
        entrypoint: &'a T,
        git_platform: Box<dyn GitPlatform<'a> + 'a>,
    ) -> Self {
        Project {
            mode,
            entrypoint: entrypoint.as_ref(),
            git_platform,
        }
    }

    /// Walk recursively through the provided path and treat TODOs in each file
    /// depending on the mode.
    pub async fn walk(&self) -> Result<()> {
        for entry in WalkDir::new(self.entrypoint)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.path().is_file() {
                continue;
            }

            let entrypoint = entry.path().to_str().unwrap();

            match &self.mode {
                Mode::Match => self.match_file(entrypoint)?,
                Mode::Patch => self.patch_file(entrypoint).await?,
            }
        }

        Ok(())
    }

    pub fn match_file(&self, file_path: &str) -> Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            if let Some(content) = regex::extract_untracked_todo_content(&line.unwrap()) {
                let todo: Todo =
                    Todo::from(None, file_path.to_string(), index + 1, content.to_string());
                println!("{}\n", todo);
            }
        }

        Ok(())
    }

    pub async fn patch_file(&self, file_path: &str) -> Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let tmp_file_name = format!("{}.tmp", file_path);
        let tmp_file = File::create(&tmp_file_name)?;
        let mut writer = BufWriter::new(tmp_file);

        for (index, chunk) in LineIterator::new(reader).enumerate() {
            let chunk = chunk?;
            let line = std::str::from_utf8(&chunk)?;

            if let Some(content) = regex::extract_untracked_todo_content(&line) {
                let todo: Todo =
                    Todo::from(None, file_path.to_string(), index + 1, content.to_string());

                println!();

                match self.prompt(&todo)? {
                    Answer::Yes => {
                        // TODO(#2) use custom values for owner/repo
                        let issue_id = self
                            .git_platform
                            .open_issue("znuznu", "nunki", todo)
                            .await?;
                        let patched_line = regex::replace_untracked_todo(&line, issue_id);
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
        rename(tmp_file_name, file_path)?;

        Ok(())
    }

    fn prompt(&self, todo: &Todo) -> Result<Answer> {
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
