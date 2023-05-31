use self::init::InitCommand;
use self::new::NewCommand;

pub(crate) mod init;
pub(crate) mod new;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Create new prodject
    New(NewCommand),
    /// Init new project
    Init(InitCommand),
}
