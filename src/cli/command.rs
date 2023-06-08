use self::init::InitCommand;
use self::new::NewCommand;
use self::router::MiddlewareArgs;
use self::router::RouterArgs;

pub(crate) mod init;
pub(crate) mod middleware;
pub(crate) mod new;
pub(crate) mod router;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Create new prodject
    New(NewCommand),
    /// Init new project
    Init(InitCommand),
    /// Router related commands
    Router(RouterArgs),
    /// Middleware related commands
    Middleware(MiddlewareArgs),
}
