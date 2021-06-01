mod github_issue;
mod github_api;

use crate::github_issue::get_github_issue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let issue = get_github_issue(50 as i32).await?;
    println!("{:#?}", issue);
    Ok(())
}
