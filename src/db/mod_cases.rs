use crate::models::mod_case::{ModAction, ModCase};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

/// atomically increment the guild case counter and return the new case number.
pub async fn next_case_number(pool: &PgPool, guild_id: i64) -> sqlx::Result<i32> {
    let row: (i32,) = sqlx::query_as(
        r#"
        UPDATE guild_case_counters
        SET next_case = next_case + 1
        WHERE guild_id = $1
        RETURNING next_case - 1
        "#,
    )
    .bind(guild_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// insert a new moderation case and return it.
pub struct NewModCase<'a> {
    pub guild_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub action: ModAction,
    pub reason: Option<&'a str>,
    pub duration_secs: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub async fn create_case(pool: &PgPool, new_case: NewModCase<'_>) -> sqlx::Result<ModCase> {
    let case_number = next_case_number(pool, new_case.guild_id).await?;
    let action_str = new_case.action.as_str();

    let case = sqlx::query_as::<_, ModCase>(
        r#"
        INSERT INTO mod_cases
            (guild_id, case_number, target_id, moderator_id, action, reason, duration_secs, expires_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(new_case.guild_id)
    .bind(case_number)
    .bind(new_case.target_id)
    .bind(new_case.moderator_id)
    .bind(action_str)
    .bind(new_case.reason)
    .bind(new_case.duration_secs)
    .bind(new_case.expires_at)
    .fetch_one(pool)
    .await?;

    Ok(case)
}

/// fetch all mod cases for a specific user in a guild.
pub async fn get_cases_for_user(
    pool: &PgPool,
    guild_id: i64,
    target_id: i64,
) -> sqlx::Result<Vec<ModCase>> {
    sqlx::query_as::<_, ModCase>(
        r#"
        SELECT * FROM mod_cases
        WHERE guild_id = $1 AND target_id = $2
        ORDER BY case_number ASC
        "#,
    )
    .bind(guild_id)
    .bind(target_id)
    .fetch_all(pool)
    .await
}

/// fetch a single case by its guild-scoped case number.
pub async fn get_case(
    pool: &PgPool,
    guild_id: i64,
    case_number: i32,
) -> sqlx::Result<Option<ModCase>> {
    sqlx::query_as::<_, ModCase>(
        "SELECT * FROM mod_cases WHERE guild_id = $1 AND case_number = $2",
    )
    .bind(guild_id)
    .bind(case_number)
    .fetch_optional(pool)
    .await
}

/// fetch all cases that have expired and are still marked active.
pub async fn get_expired_cases(pool: &PgPool) -> sqlx::Result<Vec<ModCase>> {
    sqlx::query_as::<_, ModCase>(
        r#"
        SELECT * FROM mod_cases
        WHERE active = TRUE
          AND expires_at IS NOT NULL
          AND expires_at <= NOW()
        ORDER BY expires_at ASC
        LIMIT 500
        "#,
    )
    .fetch_all(pool)
    .await
}

/// mark a case as no longer active.
pub async fn deactivate_case(pool: &PgPool, case_id: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE mod_cases SET active = FALSE WHERE id = $1")
        .bind(case_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// deactivate all active warnings for a user in a guild.
pub async fn clear_warns_for_user(pool: &PgPool, guild_id: i64, target_id: i64) -> sqlx::Result<u64> {
    let result = sqlx::query(
        "UPDATE mod_cases SET active = FALSE WHERE guild_id = $1 AND target_id = $2 AND action = 'warn' AND active = TRUE",
    )
    .bind(guild_id)
    .bind(target_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/// fetch all active warnings for a specific user in a guild.
pub async fn get_warns_for_user(
    pool: &PgPool,
    guild_id: i64,
    target_id: i64,
) -> sqlx::Result<Vec<ModCase>> {
    sqlx::query_as::<_, ModCase>(
        r#"
        SELECT * FROM mod_cases
        WHERE guild_id = $1 AND target_id = $2 AND action = 'warn' AND active = TRUE
        ORDER BY case_number ASC
        "#,
    )
    .bind(guild_id)
    .bind(target_id)
    .fetch_all(pool)
    .await
}
