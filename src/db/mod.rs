pub mod economy;
pub mod guild;
pub mod leveling;
pub mod mod_cases;
pub mod reminders;
pub mod warnings;

use crate::models::config::GuildConfig;
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;

/// db stuff
/// its just a pool and a cache so its fast.
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
    /// stuff we saved in memory.
    /// dashmap is fast and safe.
    cache: Arc<DashMap<i64, GuildConfig>>,
    /// cache for automod phrases
    automod_cache: Arc<DashMap<i64, Vec<String>>>,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Arc::new(DashMap::new()),
            automod_cache: Arc::new(DashMap::new()),
        }
    }

    /// get the config or make a new one.
    pub async fn get_guild_config(&self, guild_id: i64) -> sqlx::Result<GuildConfig> {
        // fast way
        if let Some(cfg) = self.cache.get(&guild_id) {
            return Ok(cfg.clone());
        }

        // slow way
        let config = guild::get_or_create(&self.pool, guild_id).await?;
        self.cache.insert(guild_id, config.clone());
        Ok(config)
    }

    /// invalidate a guild's cached config (call after any config update).
    pub fn invalidate_cache(&self, guild_id: i64) {
        self.cache.remove(&guild_id);
        self.automod_cache.remove(&guild_id);
    }

    /// returns the number of guild configs currently cached.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// fetch the list of blacklisted phrases for a guild.
    pub async fn get_automod_blacklist(&self, guild_id: i64) -> sqlx::Result<Vec<String>> {
        // fast way
        if let Some(list) = self.automod_cache.get(&guild_id) {
            return Ok(list.clone());
        }

        // slow way
        let rows = sqlx::query!(
            "SELECT phrase FROM automod_blacklist WHERE guild_id = $1",
            guild_id
        )
        .fetch_all(&self.pool)
        .await?;

        let list: Vec<String> = rows.into_iter().map(|r| r.phrase).collect();
        self.automod_cache.insert(guild_id, list.clone());
        Ok(list)
    }
}
