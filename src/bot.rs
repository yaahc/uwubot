use crate::Config;
use color_eyre::eyre;
use serenity::{
    builder::CreateInteraction,
    http::Http,
    model::{
        id::GuildId,
        interactions::{ApplicationCommandOptionType, Interaction},
    },
    prelude::*,
};

mod event_handler;

/// The bot, the myth, the legend, uwubot
pub struct Bot {
    config: Config,
}

impl Bot {
    /// Construct an uwubot given a config type that can be converted into
    /// `uwubot::Config`.
    pub fn new<T>(config: T) -> Self
    where
        Config: From<T>,
    {
        let config = Config::from(config);
        Self { config }
    }

    /// Run uwubot.
    pub async fn run(&self) -> eyre::Result<()> {
        let mut client = Client::builder(self.token())
            .event_handler(self.handler())
            .await?;

        client.start().await?;

        Ok(())
    }

    fn token(&self) -> &str {
        &self.config.bot_token
    }

    fn handler(&self) -> event_handler::Handler {
        event_handler::Handler
    }

    fn application_id(&self) -> u64 {
        self.config.client_id
    }

    // Commands registered here are handled in the `event_handlers` module

    /// Register a slash command for a given Guild.
    pub async fn register_slash_commands_guild(&self, guild_id: GuildId) -> eyre::Result<()> {
        let http = Http::new_with_token(self.token());

        Interaction::create_guild_application_command(
            http,
            guild_id,
            self.application_id(),
            Self::uwuify_command,
        )
        .await?;

        Ok(())
    }

    /// List all slash commands for a given Guild.
    pub async fn list_slash_commands_guild(&self, guild_id: GuildId) -> eyre::Result<()> {
        let http = Http::new_with_token(self.token());

        let guild_cmds = http
            .get_guild_application_commands(self.application_id(), guild_id.0)
            .await?;

        dbg!(guild_cmds);

        Ok(())
    }

    /// Delete a slash command for a given Guild.
    pub async fn delete_slash_commands_guild(
        &self,
        guild_id: GuildId,
        command_id: u64,
    ) -> eyre::Result<()> {
        let http = Http::new_with_token(self.token());

        http.delete_guild_application_command(self.application_id(), guild_id.0, command_id)
            .await?;

        println!("{}", crate::uwu::uwuify("command deleted ^_^"));

        Ok(())
    }

    /// Register a slash command globally for all guilds.
    ///
    /// # Details
    ///
    /// Global commands are cached with a 1 hour update interval, it is
    /// recommended that you use `register_slash_commands_guild` when testing.
    pub async fn register_slash_commands_global(&self) -> eyre::Result<()> {
        let http = Http::new_with_token(self.token());

        Interaction::create_global_application_command(
            http,
            self.application_id(),
            Self::uwuify_command,
        )
        .await?;

        Ok(())
    }

    fn uwuify_command(interaction: &mut CreateInteraction) -> &mut CreateInteraction {
        interaction
            .name("uwuify")
            .description("uwuify an impowtant m-message")
            .create_interaction_option(|option| {
                option
                    .name("text")
                    .description("text to be uwuified")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    }
}
