use clap::Parser;

use self::new::NewMiddlewareCommand;

pub(crate) mod new;

#[derive(clap::Subcommand, Debug)]
pub enum MiddlewareCommand {
    /// Create new middleware
    New(NewMiddlewareCommand),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct MiddlewareArgs {
    #[clap(subcommand)]
    pub command: MiddlewareCommand,
}
