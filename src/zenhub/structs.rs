use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Board {
    pub pipelines: Vec<Pipeline>,
}

#[derive(Deserialize, Debug)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub issues: Vec<ZenHubIssueForPipeline>,
}

#[derive(Deserialize, Debug)]
pub struct ZenHubIssueForPipeline {
    pub estimate: Option<Estimate>,
    pub is_epic: bool,
    pub issue_number: i32,
    pub position: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ZenHubIssue {
    pub pipelines: Vec<PipelineForZenHubIssue>,
    pub estimate: Option<PipelineForZenHubIssue>,
    pub is_epic: bool,
}

#[derive(Deserialize, Debug)]
pub struct PipelineForZenHubIssue {
    pub name: String,
    pub pipeline_id: String,
    pub workspace_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Estimate {
    pub value: i32,
}

#[derive(Deserialize, Debug)]
pub struct EpicIssue {
    issue_number: i32,
    repo_id: i32,
    issue_url: String,
}
