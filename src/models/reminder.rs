use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maps to the `reminders` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: i64,
    pub user_id: i64,
    pub channel_id: i64,
    pub guild_id: Option<i64>,
    pub message: String,
    pub fire_at: DateTime<Utc>,
    pub fired: bool,
    pub created_at: DateTime<Utc>,
}
