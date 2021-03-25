use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Discord API key
    pub api_key: String,
}

impl Args {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
