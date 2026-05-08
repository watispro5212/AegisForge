use serde::{Deserialize, Serialize};

/// mirrors the `guild_configs` table exactly so `sqlx::query_as!` works.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub prefix: String,

    // logging channels
    pub mod_log_channel: Option<i64>,
    pub message_log_channel: Option<i64>,
    pub member_log_channel: Option<i64>,

    // welcome / goodbye
    pub welcome_channel: Option<i64>,
    pub welcome_message: String,
    pub goodbye_channel: Option<i64>,
    pub goodbye_message: String,

    // auto-role / mute-role
    pub auto_role_id: Option<i64>,
    pub mute_role_id: Option<i64>,

    // auto-moderation feature flags
    pub automod_enabled: bool,
    pub automod_spam: bool,
    pub automod_invites: bool,
    pub automod_caps: bool,
    pub automod_mentions: bool,

    // v3 additions
    pub economy_enabled: bool,
    pub leveling_enabled: bool,
    pub level_up_message: String,
}

