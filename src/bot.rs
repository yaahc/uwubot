use crate::cli::Args;
use color_eyre::eyre;
use serenity::prelude::*;

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
        let token = &self.args.api_key;
        let mut client = Client::builder(token).event_handler(self.handler()).await?;

        client.start().await?;

        Ok(())
    }

    fn handler(&self) -> event_handler::Handler {
        event_handler::Handler
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}
