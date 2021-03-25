use color_eyre::eyre;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let bot = uwubot::Bot::new();
    bot.run()?;

    Ok(())
}
