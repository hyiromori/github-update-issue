#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::Response;

fn get_authorization_header_for_github() -> String {
    match env::var("GITHUB_ACCESS_TOKEN") {
        Ok(val) => format!("bearer {}", val),
        Err(_err) => ("").to_string(),
    }
}

#[derive(Serialize, Debug)]
pub struct GitHubIssueRequest {
    query: String,
}

pub async fn request_github_graphql_api(query: &String) -> Result<Response, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/graphql";
    let req = GitHubIssueRequest { query: String::from(query) };
    let body: String = serde_json::to_string(&req).unwrap();

    let res = reqwest::Client::new()
        .post(url)
        .header("User-Agent", "mryhryki/github-issue-cli")
        .header("Authorization", get_authorization_header_for_github())
        .body(body)
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }

    Ok(res)
}
