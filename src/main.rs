use color_eyre::eyre;
use serenity::model::id::GuildId;
use structopt::StructOpt;
use uwubot::{Bot, Config};

#[derive(Debug, StructOpt)]
struct Args {
    /// Discord Bot's token
    #[structopt(env)]
    bot_token: String,

    /// Discord Bot Client ID to register slash commands for
    #[structopt(short, long, env)]
    client_id: u64,

    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    /// Commands for manipulating guild specific slash commands
    Guild(GuildSubcommand),
}

#[derive(Debug, StructOpt)]
enum GuildSubcommand {
    /// Register slash commands for a given guild
    Register {
        /// Discord guild ID to register slash commands for
        #[structopt(short, long, env)]
        guild_id: u64,
    },
    /// Delete slash commands for a given guild
    Delete {
        /// Discord guild ID to delete slash command for
        #[structopt(short, long, env)]
        guild_id: u64,

        /// Discord command ID to delete
        #[structopt(short, long, env)]
        command_id: u64,
    },
    /// List all slash commands registered for a given guild
    Get {
        /// Discord guild ID to delete slash commands for
        #[structopt(short, long, env)]
        guild_id: u64,
    },
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let Args {
            bot_token,
            client_id,
            ..
        } = args;

        Self {
            bot_token,
            client_id,
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut config = Args::from_args();
    let subcmd = config.subcommand.take();
    let bot = Bot::new(config);

    match subcmd {
        Some(Subcommand::Guild(GuildSubcommand::Register { guild_id })) => {
            bot.register_slash_commands_guild(GuildId(guild_id)).await?
        }
        Some(Subcommand::Guild(GuildSubcommand::Delete {
            guild_id,
            command_id,
        })) => {
            bot.delete_slash_commands_guild(GuildId(guild_id), command_id)
                .await?;
            return Ok(());
        }
        Some(Subcommand::Guild(GuildSubcommand::Get { guild_id })) => {
            bot.list_slash_commands_guild(GuildId(guild_id)).await?;
            return Ok(());
        }
        None => bot.register_slash_commands_global().await?,
    }

    bot.run().await?;

    Ok(())
}
