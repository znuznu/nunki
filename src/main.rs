use crate::git::Git;
use crate::git::{data::GitData, get_remote_url, github::Github, provider::Provider};
use crate::project::{Mode, Project};
use clap::{value_t, App, Arg};
use config::Config;
use std::path::Path;

mod config;
mod git;
mod project;

#[tokio::main]
async fn main() {
    // Extract config from `nunki.toml`
    let config = Config::new();

    // Extract git data from .git/config
    let remote_url = get_remote_url(&config.remote.name);
    let git_data = GitData::from_url(&remote_url);

    // Find the token related to the provider
    let token = Provider::get_token(Provider::Github);

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
                    to `patch`: extract all untracked todos and ask to create an issue related \
                    to these todos on the remote (Github) repository. The issue # is then \
                    assigned directly in the source code, ready to be committed.",
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
        let platform = Github::new(&token);

        let project: Project = Project::from(mode, &path, Box::new(platform), git_data);

        if let Err(e) = project.walk().await {
            eprintln!("{}", e);
        }
    }
}
