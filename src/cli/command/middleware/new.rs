use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct NewMiddlewareCommandOption {}

#[derive(Clone, Debug, Args)]
#[clap(name = "new")]
pub struct NewMiddlewareCommand {
    /// Middleware name
    #[clap(name = "MIDDLEWARE_NAME")]
    pub middleware_name: String,

    #[clap(flatten)]
    pub option: NewMiddlewareCommandOption,
}
