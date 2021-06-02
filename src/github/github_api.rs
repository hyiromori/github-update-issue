#![allow(non_snake_case)]
use reqwest::Response;
use serde::Serialize;
use std::env;

fn get_authorization_header_for_github() -> String {
    match env::var("GITHUB_ACCESS_TOKEN") {
        Ok(val) => format!("bearer {}", val),
        Err(_err) => ("").to_string(),
    }
}

#[derive(Serialize, Debug)]
pub struct GraphQlRequest<T: Serialize> {
    query: String,
    variables: T,
}

pub async fn request_github_graphql_api<T: Serialize>(
    query: String,
    variables: T,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/graphql";
    let req = GraphQlRequest { query, variables };
    let body: String = serde_json::to_string(&req).unwrap();
    // println!("{}", &body);

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
