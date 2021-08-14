use crate::project::todo::Todo;
use crate::reader;
use std::io;
use walkdir::WalkDir;

/// Walk recursively through the provided path and find TODOs in each file.
pub fn walk(dir_name: &str) -> Result<(), io::Error> {
    let mut todos: Vec<Todo> = Vec::new();

    for entry in WalkDir::new(dir_name)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.path().is_file() {
            let file_todos = reader::match_against_file(entry.path().to_str().unwrap())?;
            todos.extend(file_todos);
        }
    }

    println!("Count: {}", todos.len());

    for todo in todos.into_iter() {
        println!("--------------------------");
        println!("{}", todo);
    }

    Ok(())
}
