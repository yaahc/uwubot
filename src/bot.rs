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

pub struct Bot {
    config: Config,
}

impl Bot {
    pub fn new<T>(config: T) -> Self
    where
        Config: From<T>,
    {
        let config = Config::from(config);
        Self { config }
    }

    #[tokio::main]
    pub async fn run(&self) -> eyre::Result<()> {
        if let Some(guild_id) = self.config.guild_id {
            self.register_slash_commands_guild(GuildId(guild_id))
                .await?;
        } else {
            self.register_slash_commands_global().await?;
        }

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
    async fn register_slash_commands_guild(&self, guild_id: GuildId) -> eyre::Result<()> {
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

    async fn register_slash_commands_global(&self) -> eyre::Result<()> {
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
