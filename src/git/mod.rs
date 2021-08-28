use crate::project::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;

pub mod github;

#[async_trait]
pub trait GitPlatform<'a> {
    fn new(token: &'a str) -> Self
    where
        Self: Sized;
    async fn open_issue(&self, owner: &'a str, repository: &'a str, todo: Todo) -> Result<usize>;
}
