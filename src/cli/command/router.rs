use clap::Parser;

use self::new::NewRouterCommand;

pub(crate) mod new;

#[derive(clap::Subcommand, Debug)]
pub enum RouterCommand {
    /// Create new router
    New(NewRouterCommand),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RouterArgs {
    #[clap(subcommand)]
    pub command: RouterCommand,
}
