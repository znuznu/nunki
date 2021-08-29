use crate::git::GitPlatform;
use crate::project::{Mode, Project};
use clap::{value_t, App, Arg};
use std::path::Path;

use token::{get_token, platform::Platform};

mod git;
mod project;
mod token;

fn main() {
    let token = get_token(Platform::Github);

    // let git_platform = GitPlatform::new(&token);

    let matches = App::new("Nunki CLI")
        .version("0.1.0")
        .author("Arthur 'znu' F.")
        .about("Collects TODOs in source code and reports them as issues.")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .help(
                    "The execution mode.\n\nSet to `match`: extract all untracked \
                    todos of the project and print them, without affecting anything.\n\nSet \
                    to `patch`: extract all untracked todos and create an issue \
                    related to these todos on the remote Github repository. The issue # \
                    is then assigned directly in the source code and committed, ready to be \
                    pushed.",
                )
                .next_line_help(true)
                .possible_values(&Mode::variants())
                .default_value("match")
                .required(true)
                .case_insensitive(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .help("The source code entrypoint (directory or file).")
                .takes_value(true),
        )
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        if !Path::exists(Path::new(path)) {
            eprintln!("ERROR: Path {} doesn't exist.", path);
        }

        let mode = value_t!(matches, "mode", Mode).unwrap();

        let project: Project = Project::from(mode, &path, None);

        if let Err(e) = project.walk() {
            eprintln!("{}", e);
        }
    }
}
