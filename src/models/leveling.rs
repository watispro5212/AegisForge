use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLeveling {
    pub guild_id: i64,
    pub user_id: i64,
    pub xp: i64,
    pub level: i32,
    pub last_msg: DateTime<Utc>,
    pub rank_card_background: String,
    pub rank_card_color: String,
    pub rank_card_text_color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelRole {
    pub guild_id: i64,
    pub level: i32,
    pub role_id: i64,
}

