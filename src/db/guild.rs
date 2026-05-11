use crate::models::config::GuildConfig;
use sqlx::PgPool;

/// fetch the guild config, or INSERT a default row if one doesn't exist yet.
/// uses ON CONFLICT DO NOTHING so concurrent first-interactions are safe.
pub async fn get_or_create(pool: &PgPool, guild_id: i64) -> sqlx::Result<GuildConfig> {
    // ensure a row exists (idempotent — safe under concurrency)
    sqlx::query!(
        r#"
        INSERT INTO guild_configs (guild_id)
        VALUES ($1)
        ON CONFLICT (guild_id) DO NOTHING
        "#,
        guild_id
    )
    .execute(pool)
    .await?;

    // also ensure the case counter row exists
    sqlx::query!(
        r#"
        INSERT INTO guild_case_counters (guild_id)
        VALUES ($1)
        ON CONFLICT (guild_id) DO NOTHING
        "#,
        guild_id
    )
    .execute(pool)
    .await?;

    let config = sqlx::query_as!(
        GuildConfig,
        r#"
        SELECT
            guild_id,
            prefix,
            mod_log_channel,
            message_log_channel,
            member_log_channel,
            welcome_channel,
            welcome_message,
            goodbye_channel,
            goodbye_message,
            auto_role_id,
            mute_role_id,
            automod_enabled,
            automod_spam,
            automod_invites,
            automod_caps,
            automod_mentions,
            economy_enabled,
            leveling_enabled,
            level_up_message
        FROM guild_configs
        WHERE guild_id = $1
        "#,
        guild_id
    )
    .fetch_one(pool)
    .await?;

    Ok(config)
}

/// update a specific field in guild_configs, then the caller should
/// invalidate the cache via `Database::invalidate_cache`.
pub async fn set_mod_log_channel(
    pool: &PgPool,
    guild_id: i64,
    channel_id: i64,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE guild_configs SET mod_log_channel = $1 WHERE guild_id = $2",
        channel_id,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_welcome_channel(
    pool: &PgPool,
    guild_id: i64,
    channel_id: i64,
    message: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE guild_configs SET welcome_channel = $1, welcome_message = $2 WHERE guild_id = $3",
        channel_id,
        message,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_auto_role(pool: &PgPool, guild_id: i64, role_id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE guild_configs SET auto_role_id = $1 WHERE guild_id = $2",
        role_id,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_prefix(pool: &PgPool, guild_id: i64, prefix: &str) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE guild_configs SET prefix = $1 WHERE guild_id = $2",
        prefix,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_mute_role(pool: &PgPool, guild_id: i64, role_id: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE guild_configs SET mute_role_id = $1 WHERE guild_id = $2",
    )
    .bind(role_id)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_shadow_ban(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    moderator_id: i64,
    reason: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO shadow_bans (guild_id, user_id, moderator_id, reason)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (guild_id, user_id) DO UPDATE SET
             moderator_id = EXCLUDED.moderator_id,
             reason       = EXCLUDED.reason,
             created_at   = NOW()",
    )
    .bind(guild_id)
    .bind(user_id)
    .bind(moderator_id)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_shadow_ban(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        "DELETE FROM shadow_bans WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(guild_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn is_shadow_banned(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> sqlx::Result<bool> {
    let row = sqlx::query(
        "SELECT 1 FROM shadow_bans WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}
