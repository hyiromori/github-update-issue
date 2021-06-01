#![allow(non_snake_case)]
use crate::github_api::request_github_graphql_api;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Debug)]
struct Variables {
    login: String,
    repo: String,
    issue: i32,
}

pub async fn get_github_issue(
    login: String,
    repo: String,
    issue: i32,
) -> Result<GitHubIssue, Box<dyn std::error::Error>> {
    let query = String::from(
        "query ($login: String!, $repo: String!, $issue: Int!) {
           user(login: $login) {
             repository(name: $repo){
               issue(number: $issue) {
                 title
                 url
               }
             }
           }
         }",
    );
    let variables = Variables { login, repo, issue };

    let response = request_github_graphql_api(query, variables).await?;
    // let data = response.text().await?;
    // println!("{:#?}", data);
    let data = response.json::<ResponseRoot>().await?;
    Ok(data.data.user.repository.issue)
}
