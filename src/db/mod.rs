pub mod guild;
pub mod mod_cases;
pub mod reminders;
pub mod warnings;

use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use crate::models::config::GuildConfig;

/// The central database handle passed everywhere via `Arc`.
/// Wraps the PostgreSQL connection pool and a hot in-memory cache
/// for guild configs so every slash command doesn't hit the DB.
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
    /// In-memory cache keyed by guild_id.
    /// DashMap is a lock-free concurrent hashmap — safe to share across
    /// thousands of concurrent command executions without contention.
    cache: Arc<DashMap<i64, GuildConfig>>,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Fetch guild config from cache; fall back to DB and populate cache.
    /// Creates a default row if the guild has never interacted with the bot.
    pub async fn get_guild_config(&self, guild_id: i64) -> sqlx::Result<GuildConfig> {
        // Fast path: serve from cache
        if let Some(cfg) = self.cache.get(&guild_id) {
            return Ok(cfg.clone());
        }

        // Slow path: query DB, upsert default if missing, then cache
        let config = guild::get_or_create(&self.pool, guild_id).await?;
        self.cache.insert(guild_id, config.clone());
        Ok(config)
    }

    /// Invalidate a guild's cached config (call after any config update).
    pub fn invalidate_cache(&self, guild_id: i64) {
        self.cache.remove(&guild_id);
    }

    /// Returns the number of guild configs currently cached.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
