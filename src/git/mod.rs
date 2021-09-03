use crate::project::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use ini::Ini;

pub mod data;
pub mod github;
pub mod provider;

#[async_trait]
pub trait Git<'a> {
    fn new(token: &'a str) -> Self
    where
        Self: Sized;
    async fn open_issue(&self, owner: &'a str, repository: &'a str, todo: Todo) -> Result<usize>;
}

pub fn get_remote_url(remote: &str) -> String {
    let config = Ini::load_from_file(".git/config")
        .expect("Couldn't find the .git/config file. Are you inside a git repository?");

    let section = config
        .section(Some(format!("remote \"{}\"", &remote)))
        .expect("Couldn't find the remote section in git config file.");

    section
        .get("url")
        .expect(&format!(
            "Couldn't find the remote URL in git section `{}`.",
            &remote
        ))
        .to_string()
}
