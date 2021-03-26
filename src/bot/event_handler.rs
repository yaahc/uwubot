use crate::uwu::uwuify;
use crate::ResultExt;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{Interaction, InteractionResponseType},
    },
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = if let Some(data) = &interaction.data {
            data
        } else {
            return;
        };

        match data.name.as_str() {
            "uwuify" => {}
            _ => {
                println!("unknown interaction");
                dbg!(&interaction);
                return;
            }
        }

        let option = &data.options[0];

        assert_eq!("text", option.name);

        let value = option
            .value
            .as_ref()
            .expect("text is a required argument")
            .as_str()
            .expect("text is always a String type");

        let uwud = uwuify(value);

        let http = &ctx.http;

        interaction
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| data.content(uwud))
            })
            .await
            .unwrap_or_report();
    }
}
