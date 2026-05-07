pub mod guild;
pub mod mod_cases;
pub mod reminders;
pub mod warnings;
pub mod economy;
pub mod leveling;

use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use crate::models::config::GuildConfig;

/// db stuff
/// its just a pool and a cache so its fast.
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
    /// stuff we saved in memory.
    /// dashmap is fast and safe.
    cache: Arc<DashMap<i64, GuildConfig>>,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Arc::new(DashMap::new()),
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

    /// Invalidate a guild's cached config (call after any config update).
    pub fn invalidate_cache(&self, guild_id: i64) {
        self.cache.remove(&guild_id);
    }

    /// Returns the number of guild configs currently cached.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Fetch the list of blacklisted phrases for a guild.
    pub async fn get_automod_blacklist(&self, guild_id: i64) -> sqlx::Result<Vec<String>> {
        let rows = sqlx::query!(
            "SELECT phrase FROM automod_blacklist WHERE guild_id = $1",
            guild_id
        ).fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| r.phrase).collect())
    }
}
