use color_eyre::eyre;
use structopt::StructOpt;
use uwubot::{Bot, Config};

#[derive(Debug, StructOpt)]
struct Args {
    /// Discord Bot's token
    bot_token: String,

    /// Discord Bot Client ID to register slash commands for
    #[structopt(short, long)]
    client_id: u64,

    /// Discord guild ID to register slash commands for
    #[structopt(short, long)]
    guild_id: Option<u64>,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let Args {
            bot_token,
            guild_id,
            client_id,
        } = args;

        Self {
            bot_token,
            guild_id,
            client_id,
        }
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = Args::from_args();
    let bot = Bot::new(config);
    bot.run()?;

    Ok(())
}
