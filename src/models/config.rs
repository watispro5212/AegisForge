use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub mod_log_channel: Option<i64>,
    pub welcome_channel: Option<i64>,
    pub auto_role_id: Option<i64>,
    pub prefix: String,
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            guild_id: 0,
            mod_log_channel: None,
            welcome_channel: None,
            auto_role_id: None,
            prefix: "!".to_string(),
        }
    }
}
