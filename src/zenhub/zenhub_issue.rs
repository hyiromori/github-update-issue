use serde::Deserialize;
use crate::zenhub::zenhub_api::request_zenhub_api;

#[derive(Deserialize, Debug)]
pub struct Estimate {
    value: i32,
}

#[derive(Deserialize, Debug)]
pub struct Pipeline {
    name: String,
    pipeline_id: String,
    workspace_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ZenHubIssue {
    estimate: Option<Estimate>,
    pipelines: Vec<Pipeline>,
    is_epic: bool,
}

pub async fn get_zenhub_issue(repo_id: &String, issue_number: &i32) -> Result<ZenHubIssue, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-zenhub-workspaces-for-a-repository
    let path = format!("/p1/repositories/{repo_id}/issues/{issue_number}", repo_id = repo_id, issue_number = issue_number);
    let response = request_zenhub_api(&path).await?;
    // let data = response.text().await?;
    // println!("{:#?}", data);
    let data: ZenHubIssue = response.json::<ZenHubIssue>().await?;
    Ok(data)
}


