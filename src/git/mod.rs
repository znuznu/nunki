use crate::{project::todo::Todo, token::platform::Platform};
use anyhow::Result;
use async_trait::async_trait;
use ini::Ini;
use lazy_static::lazy_static;
use regex::Regex;

pub mod github;

#[async_trait]
pub trait GitPlatform<'a> {
    fn new(token: &'a str) -> Self
    where
        Self: Sized;
    async fn open_issue(&self, owner: &'a str, repository: &'a str, todo: Todo) -> Result<usize>;
}

#[derive(Debug)]
pub struct GitData<'a> {
    platform: Platform,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> PartialEq for GitData<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.repo == other.repo && self.platform == other.platform
    }
}

impl<'a> GitData<'a> {
    /// Extract platform, owner and repo from a git URL and return a GitData.
    ///
    /// URL example: git@github.com:znuznu/nunki.git
    fn from(url: &'a str) -> Self {
        lazy_static! {
            // TODO enhance the git URL regex
            static ref RE: Regex = Regex::new(r"^git@(.*)+:(.*)/(.*).git$").unwrap();
        }

        match RE.captures(&url) {
            Some(caps) => {
                let platform = match caps.get(1).map(|p| p.as_str()).unwrap() {
                    // TODO enhance with a ::from()
                    "github.com" => Platform::Github,
                    _ => panic!("Unknown git domain."),
                };

                let owner = caps.get(2).map(|o| o.as_str()).unwrap();
                let repo = caps.get(3).map(|r| r.as_str()).unwrap();

                Self {
                    platform,
                    owner,
                    repo,
                }
            }
            None => panic!("Invalid url {}", &url),
        }
    }
}

pub fn find_git_data(remote: &str) -> GitData {
    let config = Ini::load_from_file("src/.git/config")
        .expect("Couldn't find the .git/config file. Are you inside a git repository?");

    let section = config
        .section(Some(format!("remove {}", &remote)))
        .expect("Couldn't find the remote section in git config file.");

    let remote_url = section.get("url").expect(&format!(
        "Couldn't find the remote URL in git section `{}`.",
        &remote
    ));

    GitData::from(&remote_url)
}

/// Extract platform, owner and repo from a git URL.
///
/// URL example: git@github.com:znuznu/nunki.git
// fn extract_git_data(url: &str) -> Option<GitData> {
//     lazy_static! {
//         // TODO enhance the git URL regex
//         static ref RE: Regex = Regex::new(r"^git@(.*)+:(.*)/(.*).git$").unwrap();
//     }

//     match RE.captures(&url) {
//         Some(caps) => {
//             let platform = match caps.get(1).map(|p| p.as_str()).unwrap() {
//                 // TODO enhance with a ::from()
//                 "github.com" => Platform::Github,
//                 _ => panic!("Unknown git domain."),
//             };

//             let owner = caps.get(2).map(|o| o.as_str()).unwrap();
//             let repo = caps.get(3).map(|r| r.as_str()).unwrap();

//             Some(GitData {
//                 platform,
//                 owner,
//                 repo,
//             })
//         }
//         None => None,
//     }
// }

#[cfg(test)]
mod tests {
    use super::{GitData, Platform};

    #[test]
    fn from_valid_url() {
        assert_eq!(
            GitData::from("git@github.com:znuznu/nunki.git",),
            GitData {
                platform: Platform::Github,
                owner: "znuznu",
                repo: "nunki"
            }
        );
    }
}
