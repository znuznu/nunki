use platform::Platform;
use std::env;

pub mod platform;

/// Get the token related to the provided git platform.
/// Panic if the token isn't set as an env variable.
pub fn get_token(platform: Platform) -> String {
    match platform {
        Platform::Github => env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect(
            "No token found. Did you forget to set your GITHUB_PERSONAL_ACCESS_TOKEN variable ?",
        ),
    }
}
