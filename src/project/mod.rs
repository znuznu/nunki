use crate::git::data::GitData;
use crate::git::Git;

use anyhow::Result;
use clap::arg_enum;
use prompt::{prompt, Answer};
use std::collections::HashSet;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::FromIterator;

use todo::Todo;
use walkdir::WalkDir;

use line_iterator::LineIterator;

pub mod line_iterator;
pub mod prompt;
pub mod regex;
pub mod todo;

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
    git_platform: Box<dyn Git<'a> + 'a>,
    git_data: GitData<'a>,
    ignored_paths: HashSet<String>,
}

impl<'a> Project<'a> {
    pub fn from<T: AsRef<str>>(
        mode: Mode,
        entrypoint: &'a T,
        git_platform: Box<dyn Git<'a> + 'a>,
        git_data: GitData<'a>,
        ignored_paths: Vec<String>,
    ) -> Self {
        Project {
            mode,
            entrypoint: entrypoint.as_ref(),
            git_platform,
            git_data,
            ignored_paths: HashSet::from_iter(ignored_paths),
        }
    }

    /// Walk recursively through the provided path and process TODOs in each file
    /// depending on the mode.
    pub async fn walk(&self) -> Result<()> {
        for entry in WalkDir::new(self.entrypoint)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            match entry.path().to_str() {
                Some(p) => {
                    if self.ignored_paths.contains(p) {
                        continue;
                    }
                }
                None => continue,
            }

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

    fn match_file(&self, file_path: &str) -> Result<()> {
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

    async fn patch_file(&self, file_path: &str) -> Result<()> {
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

                match prompt(&todo)? {
                    Answer::Yes => {
                        let issue_id = self
                            .git_platform
                            .open_issue(self.git_data.owner, self.git_data.repo, todo)
                            .await?;
                        let patched_line = regex::replace_untracked_todo(&line, issue_id);
                        writer.write_all(&patched_line.as_bytes())?;
                    }
                    Answer::No => {
                        writer.write_all(&line.as_bytes())?;
                    }
                }
            } else {
                writer.write_all(&line.as_bytes())?;
            }
        }

        writer.flush()?;
        rename(tmp_file_name, file_path)?;

        Ok(())
    }
}
