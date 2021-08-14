use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

mod regex;

pub fn read_file(file_name: &str) -> io::Result<()> {
    println!("Filename: {}", file_name);

    let file = File::open(file_name)?;
    let file = BufReader::new(file);

    for line in file.lines() {
        if let Some(todo) = regex::match_todo(&line.unwrap()) {
            println!("{:?}", todo.to_string());
        }
    }

    Ok(())
}
