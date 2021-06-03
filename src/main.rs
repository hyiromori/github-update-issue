mod github;
mod zenhub;

use crate::github::github_issue::get_github_issue;
use crate::github::github_repo::get_github_repo_id;
use crate::zenhub::workspace::get_zenhub_workspaces;
use crate::zenhub::zenhub_issue::{get_zenhub_issue, move_pipeline};
use crate::zenhub::board::get_board;
use crate::zenhub::epic::get_epic_issues;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owner: String = String::from("Connehito");
    let repo: String = String::from("yorozu-issues");
    // let issue_number: i32 = 350;

    // let workspace_id: String = String::from("606c08cc26504900173dc46e");
    // let pipeline_id: String = String::from("Z2lkOi8vcmFwdG9yL1BpcGVsaW5lLzIzNjU5NTk"); // New Issue
    // let pipeline_id: String = String::from("Z2lkOi8vcmFwdG9yL1BpcGVsaW5lLzIzNjU5Njc"); // Task Current

    let repo_id = get_github_repo_id(&owner, &repo).await?;
    // let github_issue = get_github_issue(&owner, &repo, &issue_number).await?;
    // let boards = get_zenhub_workspaces(&repo_id).await?;
    // let pipelines = get_board(&workspace_id, &repo_id).await?;
    let epic_issues = get_epic_issues(&repo_id).await?;

    // move_pipeline(&workspace_id, &repo_id, &issue_number, &pipeline_id).await?;

    // let zenhub_issue = get_zenhub_issue(&repo_id, &issue_number).await?;
    // println!("{:#?}", github_issue);
    // println!("{:#?}", repo_id);
    // println!("{:#?}", boards);
    // println!("{:#?}", zenhub_issue);
    // println!("{:#?}", pipelines);
    println!("{:#?}", epic_issues);
    Ok(())
}
