mod cli;
mod constants;
mod run;
mod utils;

use clap::Parser;
use cli::{
    command::{router::RouterCommand, Command},
    Args,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::New(command) => {
            let project_name = command.project_name;
            let template_name = command.option.template_name.unwrap_or("default".to_owned());

            run::new::run_new(project_name, template_name).await;
        }
        Command::Init(command) => {
            let project_name = command.project_name.unwrap_or("sample".to_owned());
            let template_name = command.option.template_name.unwrap_or("default".to_owned());

            run::init::run_init(project_name, template_name).await;
        }
        Command::Router(command) => {
            match command.command {
                RouterCommand::New(command) => {
                    let router_name = command.router_name;
                    let template_name = command.option.template_name.unwrap_or("crud".to_owned());

                    println!("router_name: {}", router_name);
                    println!("template_name: {}", template_name);

                    //run::router::run_new_router(router_name, template_name).await;
                }
            }
        }
    }

    Ok(())
}
