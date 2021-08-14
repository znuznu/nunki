use clap::{App, Arg};
use std::path::Path;

mod project;
mod reader;
mod walker;

fn main() {
    let matches = App::new("Nunki CLI")
        .version("0.1.0")
        .author("Arthur 'znu' F.")
        .about("Collects TODOs in source code and reports them as issues.")
        .arg(Arg::with_name("path").short("p").takes_value(true))
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        if !Path::exists(Path::new(path)) {
            eprintln!("ERROR: Path {} doesn't exist.", path);
        }

        if let Err(e) = walker::walk(path) {
            eprintln!("{}", e);
        }
    }
}
