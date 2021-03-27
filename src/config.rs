/// Uwubot configuration values
pub struct Config {
    /// Discord Bot's Token
    pub bot_token: String,
    /// Discord Bot Client ID to register slash commands for
    pub client_id: u64,
    /// Discord guild ID to register slash commands for
    pub guild_id: Option<u64>,
}
