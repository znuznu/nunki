use std::env;

#[derive(Debug, PartialEq)]
pub enum Provider {
    Github,
}

impl Provider {
    pub fn from_domain(domain: &str) -> Self {
        match domain {
            "github.com" => Provider::Github,
            _ => panic!("Unknown git domain."),
        }
    }

    /// Get the token related to Self.
    /// Panic if the token isn't set as an env variable.
    pub fn get_token(provider: Provider) -> String {
        match provider {
            Provider::Github => env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect(
                "No token found. Did you forget to set your GITHUB_PERSONAL_ACCESS_TOKEN variable?",
            ),
        }
    }
}
