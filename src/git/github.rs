use crate::git::GitPlatform;
use crate::project::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::StatusCode;
use reqwest::{header::HeaderMap, Body, Client as ReqwestClient};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct OpenIssueResponse {
    number: usize,
}

#[derive(Serialize)]
struct OpenIssueRequestBody<'a> {
    title: &'a str,
}

impl<'a> OpenIssueRequestBody<'a> {
    fn from(title: &'a str) -> Self {
        Self { title }
    }
}

pub struct Github<'a> {
    token: &'a str,
    client: ReqwestClient,
}

impl<'a> Github<'a> {
    const API_PREFIX: &'a str = "https://api.github.com";
}

#[async_trait]
impl<'a> GitPlatform<'a> for Github<'a> {
    fn new(token: &'a str) -> Self {
        Self {
            token,
            client: ReqwestClient::new(),
        }
    }

    async fn open_issue(&self, owner: &'a str, repository: &'a str, todo: Todo) -> Result<usize> {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Accept", "application/vnd.github.v3+json".parse()?);
        headers.insert("Authorization", format!("token {}", self.token).parse()?);
        headers.insert("User-agent", "nunki".parse()?);

        let body = OpenIssueRequestBody::from(&todo.content);

        dbg!(&headers);

        let response = self
            .client
            .post(format!(
                "{}/repos/{}/{}/issues",
                Github::API_PREFIX,
                owner,
                repository
            ))
            .json(&body)
            .headers(headers)
            .send()
            .await?;

        dbg!(&response);

        match response.status() {
            StatusCode::CREATED => {
                let issue = response.json::<OpenIssueResponse>().await?;
                return Ok(issue.number);
            }
            _ => {
                panic!(
                    "Unexpected Github response status code: {}",
                    response.status()
                )
            }
        }
    }
}
