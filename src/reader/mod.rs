use crate::project::todo::Todo;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

mod regex;

pub fn match_against_file(file_path: &str) -> Result<Vec<Todo>, io::Error> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file);

    let mut todos: Vec<Todo> = Vec::new();
    let mut line_count = 0;

    for line in file.lines() {
        line_count += 1;

        if let Some(content) = regex::extract_untracked_todo_content(&line.unwrap()) {
            let todo: Todo =
                Todo::from(None, file_path.to_string(), line_count, content.to_string());
            todos.push(todo);
        }
    }

    Ok(todos)
}
