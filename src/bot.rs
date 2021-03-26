use crate::cli::Args;
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
    args: Args,
}

impl Bot {
    pub fn new() -> Self {
        let args = Args::from_args();
        Self { args }
    }

    #[tokio::main]
    pub async fn run(&self) -> eyre::Result<()> {
        if let Some(guild_id) = self.args.guild_id {
            self.register_slash_commands(GuildId(guild_id)).await?;
        }

        let mut client = Client::builder(self.token())
            .event_handler(self.handler())
            .await?;

        client.start().await?;

        Ok(())
    }

    fn token(&self) -> &str {
        &self.args.api_key
    }

    fn handler(&self) -> event_handler::Handler {
        event_handler::Handler
    }

    fn application_id(&self) -> u64 {
        self.args.client_id
    }

    // Commands registered here are handled in the `event_handlers` module
    async fn register_slash_commands(&self, guild_id: GuildId) -> eyre::Result<()> {
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

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}
