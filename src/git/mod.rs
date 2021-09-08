use crate::project::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use ini::Ini;
use std::path::Path;
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

pub fn get_remote_url(remote: &str, git_cfg_path: &Path) -> String {
    let config = Ini::load_from_file(&git_cfg_path)
        .expect("Couldn't find the .git/config file. Are you inside a git repository?");

    let section = config
        .section(Some(format!("remote \"{}\"", &remote)))
        .expect("Couldn't find the remote section in git config file.");

    section
        .get("url")
        .unwrap_or_else(|| panic!("Couldn't find the remote URL in git section `{}`.", &remote))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{get_remote_url, Path};
    use std::fs::{DirBuilder, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    #[should_panic(
        expected = "Couldn't find the .git/config file. Are you inside a git repository?"
    )]
    fn config_not_found() {
        get_remote_url("remote", &Path::new("some_url"));
    }

    #[test]
    #[should_panic(expected = "Couldn't find the remote section in git config file.")]
    fn remote_section_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join(".git");
        let file_path = dir_path.join("config");
        DirBuilder::new().recursive(true).create(dir_path).unwrap();

        File::create(&file_path).unwrap();

        get_remote_url("remote", &file_path);
    }

    #[test]
    #[should_panic(expected = "Couldn't find the remote URL in git section `origin`.")]
    fn url_in_section_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join(".git");
        let file_path = dir_path.join("config");
        DirBuilder::new().recursive(true).create(dir_path).unwrap();

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"[ remote \"origin\" ]")
            .expect("Couldn't write to the tmp file in url_in_section_not_found");
        get_remote_url("origin", &file_path);
    }

    #[test]
    fn valid_config() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join(".git");
        let file_path = dir_path.join("config");
        DirBuilder::new().recursive(true).create(dir_path).unwrap();

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"[ remote \"origin\" ]\nurl = git@github.com:znuznu/nunki.git")
            .expect("Couldn't write to the tmp file in url_in_section_not_found");
        let url = get_remote_url("origin", &file_path);

        assert_eq!(url, "git@github.com:znuznu/nunki.git");
    }
}
