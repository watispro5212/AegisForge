use serde::{Deserialize, Serialize};

/// Mirrors the `guild_configs` table exactly so `sqlx::query_as!` works.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub prefix: String,

    // Logging channels
    pub mod_log_channel: Option<i64>,
    pub message_log_channel: Option<i64>,
    pub member_log_channel: Option<i64>,

    // Welcome / goodbye
    pub welcome_channel: Option<i64>,
    pub welcome_message: String,
    pub goodbye_channel: Option<i64>,
    pub goodbye_message: String,

    // Auto-role / mute-role
    pub auto_role_id: Option<i64>,
    pub mute_role_id: Option<i64>,

    // Auto-moderation feature flags
    pub automod_enabled: bool,
    pub automod_spam: bool,
    pub automod_invites: bool,
    pub automod_caps: bool,
    pub automod_mentions: bool,
}
