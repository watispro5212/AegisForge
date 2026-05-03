use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maps to the `warnings` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub id: i64,
    pub guild_id: i64,
    pub user_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub case_id: Option<i64>,
    pub pardoned: bool,
    pub created_at: DateTime<Utc>,
}
