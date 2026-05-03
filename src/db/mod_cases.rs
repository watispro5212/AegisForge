use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::models::mod_case::{ModCase, ModAction};

/// Atomically increment the guild case counter and return the new case number.
/// Uses `FOR UPDATE` to prevent duplicate case numbers under concurrent mods.
pub async fn next_case_number(pool: &PgPool, guild_id: i64) -> sqlx::Result<i32> {
    let row = sqlx::query!(
        r#"
        UPDATE guild_case_counters
        SET next_case = next_case + 1
        WHERE guild_id = $1
        RETURNING next_case - 1 AS "case_number!"
        "#,
        guild_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.case_number)
}

/// Insert a new moderation case and return it.
pub async fn create_case(
    pool: &PgPool,
    guild_id: i64,
    target_id: i64,
    moderator_id: i64,
    action: ModAction,
    reason: Option<&str>,
    duration_secs: Option<i64>,
    expires_at: Option<DateTime<Utc>>,
) -> sqlx::Result<ModCase> {
    let case_number = next_case_number(pool, guild_id).await?;
    let action_str = action.as_str();

    let case = sqlx::query_as!(
        ModCase,
        r#"
        INSERT INTO mod_cases
            (guild_id, case_number, target_id, moderator_id, action, reason, duration_secs, expires_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
        guild_id,
        case_number,
        target_id,
        moderator_id,
        action_str,
        reason,
        duration_secs,
        expires_at
    )
    .fetch_one(pool)
    .await?;

    Ok(case)
}

/// Fetch all mod cases for a specific user in a guild.
pub async fn get_cases_for_user(
    pool: &PgPool,
    guild_id: i64,
    target_id: i64,
) -> sqlx::Result<Vec<ModCase>> {
    sqlx::query_as!(
        ModCase,
        r#"
        SELECT * FROM mod_cases
        WHERE guild_id = $1 AND target_id = $2
        ORDER BY case_number ASC
        "#,
        guild_id,
        target_id
    )
    .fetch_all(pool)
    .await
}

/// Fetch a single case by its guild-scoped case number.
pub async fn get_case(
    pool: &PgPool,
    guild_id: i64,
    case_number: i32,
) -> sqlx::Result<Option<ModCase>> {
    sqlx::query_as!(
        ModCase,
        "SELECT * FROM mod_cases WHERE guild_id = $1 AND case_number = $2",
        guild_id,
        case_number
    )
    .fetch_optional(pool)
    .await
}

/// Fetch all cases that have expired and are still marked active.
/// Used by the background expiry worker to lift timed bans/mutes.
pub async fn get_expired_cases(pool: &PgPool) -> sqlx::Result<Vec<ModCase>> {
    sqlx::query_as!(
        ModCase,
        r#"
        SELECT * FROM mod_cases
        WHERE active = TRUE
          AND expires_at IS NOT NULL
          AND expires_at <= NOW()
        ORDER BY expires_at ASC
        LIMIT 500
        "#
    )
    .fetch_all(pool)
    .await
}

/// Mark a case as no longer active (e.g. after an unban or untimeout).
pub async fn deactivate_case(pool: &PgPool, case_id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE mod_cases SET active = FALSE WHERE id = $1",
        case_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
