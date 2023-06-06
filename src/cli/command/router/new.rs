use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct NewRouterCommandOption {
    /// Template name
    #[clap(long, short)]
    pub template_name: Option<String>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "new")]
pub struct NewRouterCommand {
    /// Router name
    #[clap(name = "ROUTER_NAME")]
    pub router_name: String,

    #[clap(flatten)]
    pub option: NewRouterCommandOption,
}
