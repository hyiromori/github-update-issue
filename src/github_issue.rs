#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::env;
use crate::github_api::request_github_graphql_api;

#[derive(Serialize, Debug)]
pub struct GitHubIssueRequest {
    query: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    issue: GitHubIssue,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub title: String,
    pub url: String,
}

pub async fn get_github_issue(
    issue_number: i32,
) -> Result<GitHubIssue, Box<dyn std::error::Error>> {
    let query = format!(
        "{{
      user(login: \"mryhryki\") {{
        repository(name: \"HOME\"){{
          issue(number: {}) {{
            title
            url
          }}
        }}
      }}
    }}",
        issue_number
    );

    let data = request_github_graphql_api(&query).await?.json::<ResponseRoot>().await?;
    Ok(data.data.user.repository.issue)
}
