use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEconomy {
    pub guild_id: i64,
    pub user_id: i64,
    pub balance: i64,
    pub bank: i64,
    pub last_daily: Option<DateTime<Utc>>,
    pub last_work: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
