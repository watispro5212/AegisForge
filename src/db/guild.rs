use sqlx::PgPool;
use crate::models::config::GuildConfig;

/// Fetch the guild config, or INSERT a default row if one doesn't exist yet.
/// Uses ON CONFLICT DO NOTHING so concurrent first-interactions are safe.
pub async fn get_or_create(pool: &PgPool, guild_id: i64) -> sqlx::Result<GuildConfig> {
    // Ensure a row exists (idempotent — safe under concurrency)
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

    // Also ensure the case counter row exists
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
            automod_mentions
        FROM guild_configs
        WHERE guild_id = $1
        "#,
        guild_id
    )
    .fetch_one(pool)
    .await?;

    Ok(config)
}

/// Update a specific field in guild_configs, then the caller should
/// invalidate the cache via `Database::invalidate_cache`.
pub async fn set_mod_log_channel(pool: &PgPool, guild_id: i64, channel_id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE guild_configs SET mod_log_channel = $1 WHERE guild_id = $2",
        channel_id,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_welcome_channel(pool: &PgPool, guild_id: i64, channel_id: i64, message: &str) -> sqlx::Result<()> {
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
