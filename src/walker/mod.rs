use crate::reader;
use std::io;
use walkdir::WalkDir;

pub fn walk(dir_name: &str) -> io::Result<()> {
    for entry in WalkDir::new(dir_name)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.path().is_file() {
            reader::read_file(entry.path().to_str().unwrap())?;
        }
    }

    Ok(())
}
