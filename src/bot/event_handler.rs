use color_eyre::eyre;
use eyre::eyre;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{Interaction, InteractionResponseType},
    },
    prelude::*,
    utils::MessageBuilder,
};

use crate::uwu::uwuify;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.starts_with("!uwuify ") {
            let original_msg = msg.content.trim_start_matches("!uwuify ");
            let uwud = uwuify(original_msg);

            dbg!(msg.guild_id);

            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new().push(uwud).build();

            if let Err(e) = msg.channel_id.say(&context.http, &response).await {
                let e = eyre!(e);
                eprintln!("Error: {:?}", e)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // dbg!(ctx);
        dbg!(&interaction);
        let data = if let Some(data) = &interaction.data {
            data
        } else {
            return;
        };

        match data.name.as_str() {
            "uwuify" => {
                println!("got uwuify command");
                let option = &data.options[0];
                assert_eq!("text", option.name);
                let value = option
                    .value
                    .as_ref()
                    .expect("text is a required argument")
                    .as_str()
                    .expect("text is always a String type");
                dbg!(value);
                let uwud = uwuify(value);

                let http = &ctx.http;
                if let Err(e) = interaction
                    .create_interaction_response(http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|data| data.content(uwud))
                    })
                    .await
                {
                    let e = eyre!(e);
                    eprintln!("Error: {:?}", e)
                }
            }
            _ => println!("unknown interaction"),
        }
    }
}
