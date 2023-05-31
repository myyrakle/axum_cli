mod cli;
mod constants;

use clap::Parser;
use cli::{command::Command, Args};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::New(command) => {
            let project_name = command.project_name;
            let template_name = command.option.template_name.unwrap_or("default".to_owned());
            // let template_manager = TemplateManager::new(project_name.clone(), template_name);

            // template_manager.new_template(project_name).await?;
        }
        Command::Init(command) => {
            let project_name = command.project_name.unwrap_or("sample".to_owned());
            let template_name = command.option.template_name.unwrap_or("default".to_owned());
            // let template_manager = TemplateManager::new(project_name, template_name);

            // template_manager.new_template(".".to_owned()).await?;
        }
    }

    Ok(())
}
