use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maps to the `mod_cases` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModCase {
    pub id: i64,
    pub guild_id: i64,
    pub case_number: i32,
    pub target_id: i64,
    pub moderator_id: i64,
    pub action: String,
    pub reason: Option<String>,
    pub duration_secs: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

/// Typed enum for all supported moderation actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModAction {
    Ban,
    Unban,
    Kick,
    Timeout,
    Untimeout,
    Warn,
    Note,
    Mute,
    Unmute,
    TempBan,
}

impl ModAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ban      => "ban",
            Self::Unban    => "unban",
            Self::Kick     => "kick",
            Self::Timeout  => "timeout",
            Self::Untimeout=> "untimeout",
            Self::Warn     => "warn",
            Self::Note     => "note",
            Self::Mute     => "mute",
            Self::Unmute   => "unmute",
            Self::TempBan  => "tempban",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Ban      => "🔨 Banned",
            Self::Unban    => "🔓 Unbanned",
            Self::Kick     => "👢 Kicked",
            Self::Timeout  => "🔇 Timed Out",
            Self::Untimeout=> "🔊 Timeout Removed",
            Self::Warn     => "⚠️ Warned",
            Self::Note     => "📝 Note Added",
            Self::Mute     => "🔇 Muted",
            Self::Unmute   => "🔊 Unmuted",
            Self::TempBan  => "⏳ Temp-Banned",
        }
    }
}
