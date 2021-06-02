mod github;
use crate::github::github_issue::get_github_issue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let issue = get_github_issue(String::from("mryhryki"), String::from("HOME"), 50 as i32).await?;
    println!("{:#?}", issue);
    Ok(())
}
