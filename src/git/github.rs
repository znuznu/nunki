use crate::git::GitPlatform;
use crate::project::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::StatusCode;
use reqwest::{header::HeaderMap, Client};

use serde::Deserialize;

#[derive(Deserialize)]
struct OpenIssueResponse {
    number: usize,
}

pub struct Github<'a> {
    token: &'a str,
    client: Client,
}

impl<'a> Github<'a> {
    const API_PREFIX: &'a str = "https://api.github.com/";
}

#[async_trait]
impl<'a> GitPlatform<'a> for Github<'a> {
    fn new(token: &'a str) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    async fn open_issue(&self, owner: &'a str, repository: &'a str, todo: Todo) -> Result<usize> {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Accept", "application/vnd.github.v3+json".parse()?);
        headers.insert("Authorization", format!("token {}", self.token).parse()?);

        let response = self
            .client
            .post(format!(
                "{}/repos/{}/{}/issues",
                Github::API_PREFIX,
                owner,
                repository
            ))
            .body(todo.content)
            .headers(headers)
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => {
                let issue = response.json::<OpenIssueResponse>().await?;
                return Ok(issue.number);
            }
            _ => panic!(
                "Unexpected Github response status code: {}",
                response.status()
            ),
        }
    }
}
