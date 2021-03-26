use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Discord API key
    pub api_key: String,
    /// Discord guild ID to register slash commands for
    #[structopt(short, long)]
    pub guild_id: Option<u64>,
}

impl Args {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
