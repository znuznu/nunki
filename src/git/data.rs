use crate::git::provider::Provider;
use lazy_static::lazy_static;
use regex::Regex;

// TODO(#3) rename GitData to GitRemote ?
#[derive(Debug)]
pub struct GitData<'a> {
    provider: Provider,
    pub owner: &'a str,
    pub repo: &'a str,
}

impl<'a> PartialEq for GitData<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.repo == other.repo && self.provider == other.provider
    }
}

impl<'a> GitData<'a> {
    /// Extract provider, owner and repo from a git URL and return a GitData.
    ///
    /// URL example: git@github.com:znuznu/nunki.git
    pub fn from_url(remote_url: &'a str) -> Self {
        lazy_static! {
            // TODO(#4) enhance the git remote URL regex
            static ref RE: Regex = Regex::new(r"^git@(.*)+:(.*)/(.*).git$").unwrap();
        }

        match RE.captures(&remote_url) {
            Some(caps) => {
                let provider = Provider::from_domain(caps.get(1).map(|p| p.as_str()).unwrap());

                let owner = caps.get(2).map(|o| o.as_str()).unwrap();
                let repo = caps.get(3).map(|r| r.as_str()).unwrap();

                Self {
                    provider,
                    owner,
                    repo,
                }
            }
            None => panic!("Invalid remote URL {}", &remote_url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GitData, Provider};

    #[test]
    fn from_valid_url() {
        assert_eq!(
            GitData::from_url("git@github.com:znuznu/nunki.git"),
            GitData {
                provider: Provider::Github,
                owner: "znuznu",
                repo: "nunki"
            }
        );
    }

    #[test]
    #[should_panic(expected = "Invalid remote URL such_fake")]
    fn from_invalid_url() {
        GitData::from_url("such_fake");
    }

    #[test]
    #[should_panic(expected = "Unknown git domain.")]
    fn unknown_domain() {
        GitData::from_url("git@fake.com:znuznu/nunki.git");
    }
}
