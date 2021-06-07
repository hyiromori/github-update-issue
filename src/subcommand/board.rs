use crate::util::config::Config;
use crate::util::select::select_in_menu;
use crate::zenhub::board::get_pipelines;
use crate::zenhub::structs::Board;

#[derive(Debug, Clone)]
enum BoardAction {
    Pipeline,
}

impl ToString for BoardAction {
    fn to_string(&self) -> String {
        match self {
            BoardAction::Pipeline => String::from("Pipeline"),
        }
    }
}

pub async fn board(config: &Config, _args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let actions = vec![BoardAction::Pipeline];
    let action = select_in_menu(&String::from("Choose action:"), &actions).unwrap();

    match action {
        BoardAction::Pipeline => {
            let pipelines = get_pipelines(&config.workspace_id, &config.repo_id).await?;
            let pipeline = select_in_menu(&String::from("Select pipeline"), &pipelines);
            println!("{:?}", pipeline);
        }
    };

    Ok(())
}
