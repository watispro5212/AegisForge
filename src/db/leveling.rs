use sqlx::PgPool;
use crate::models::leveling::{UserLeveling, LevelRole};
use chrono::{DateTime, Utc};

pub async fn get_user_leveling(pool: &PgPool, guild_id: i64, user_id: i64) -> sqlx::Result<UserLeveling> {
    let leveling = sqlx::query_as!(
        UserLeveling,
        r#"
        SELECT * FROM users_leveling
        WHERE guild_id = $1 AND user_id = $2
        "#,
        guild_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(lvl) = leveling {
        Ok(lvl)
    } else {
        // Ensure guild exists first to satisfy foreign key
        crate::db::guild::get_or_create(pool, guild_id).await?;

        // Create default
        sqlx::query!(
            "INSERT INTO users_leveling (guild_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            guild_id,
            user_id
        ).execute(pool).await?;

        Ok(sqlx::query_as!(
            UserLeveling,
            "SELECT * FROM users_leveling WHERE guild_id = $1 AND user_id = $2",
            guild_id,
            user_id
        ).fetch_one(pool).await?)
    }
}

pub async fn add_xp(pool: &PgPool, guild_id: i64, user_id: i64, amount: i64) -> sqlx::Result<bool> {
    let mut leveling = get_user_leveling(pool, guild_id, user_id).await?;
    
    // 1 minute cooldown for XP gain to prevent spam
    if Utc::now().signed_duration_since(leveling.last_msg).num_seconds() < 60 {
        return Ok(false);
    }

    leveling.xp += amount;
    let new_level = (leveling.xp as f64).sqrt() as i32 / 5; // Simple formula
    let leveled_up = new_level > leveling.level;
    leveling.level = new_level;

    sqlx::query!(
        "UPDATE users_leveling SET xp = $1, level = $2, last_msg = NOW() WHERE guild_id = $3 AND user_id = $4",
        leveling.xp,
        leveling.level,
        guild_id,
        user_id
    ).execute(pool).await?;

    Ok(leveled_up)
}

pub async fn get_leaderboard(pool: &PgPool, guild_id: i64, limit: i64) -> sqlx::Result<Vec<UserLeveling>> {
    sqlx::query_as!(
        UserLeveling,
        "SELECT * FROM users_leveling WHERE guild_id = $1 ORDER BY xp DESC LIMIT $2",
        guild_id,
        limit
    ).fetch_all(pool).await
}

pub struct GlobalLevelingEntry {
    pub user_id: i64,
    pub total_xp: i64,
}

pub async fn get_global_leaderboard(pool: &PgPool, limit: i64) -> Result<Vec<GlobalLevelingEntry>, sqlx::Error> {
    sqlx::query_as!(
        GlobalLevelingEntry,
        "SELECT user_id, SUM(xp)::BIGINT as \"total_xp!\" FROM users_leveling GROUP BY user_id ORDER BY SUM(xp) DESC LIMIT $1",
        limit
    )
    .fetch_all(pool)
    .await
}

pub async fn get_total_xp(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!("SELECT SUM(xp)::BIGINT as \"total!\" FROM users_leveling")
        .fetch_one(pool)
        .await?;
    Ok(row.total)
}

pub async fn get_level_roles(pool: &PgPool, guild_id: i64) -> sqlx::Result<Vec<LevelRole>> {
    sqlx::query_as!(
        LevelRole,
        "SELECT * FROM level_roles WHERE guild_id = $1 ORDER BY level ASC",
        guild_id
    ).fetch_all(pool).await
}
