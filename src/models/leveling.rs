use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserLeveling {
    pub guild_id: i64,
    pub user_id: i64,
    pub xp: i64,
    pub level: i32,
    pub last_msg: DateTime<Utc>,
    pub rank_card_background: String,
    pub rank_card_color: String,
    pub rank_card_text_color: String,
    pub rank_card_badge: String,
    pub rank_card_frame: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LevelRole {
    pub guild_id: i64,
    pub level: i32,
    pub role_id: i64,
}
