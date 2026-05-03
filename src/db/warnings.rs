use sqlx::PgPool;
use crate::models::warning::Warning;

/// Insert a warning (assumes the matching mod_case is already created).
pub async fn create(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    moderator_id: i64,
    reason: &str,
    case_id: Option<i64>,
) -> sqlx::Result<Warning> {
    sqlx::query_as!(
        Warning,
        r#"
        INSERT INTO warnings (guild_id, user_id, moderator_id, reason, case_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        guild_id,
        user_id,
        moderator_id,
        reason,
        case_id
    )
    .fetch_one(pool)
    .await
}

/// Count active (non-pardoned) warnings for a user in a guild.
pub async fn count(pool: &PgPool, guild_id: i64, user_id: i64) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM warnings
        WHERE guild_id = $1 AND user_id = $2 AND pardoned = FALSE
        "#,
        guild_id,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

/// Fetch all warnings for a user in a guild.
pub async fn get_for_user(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> sqlx::Result<Vec<Warning>> {
    sqlx::query_as!(
        Warning,
        r#"
        SELECT * FROM warnings
        WHERE guild_id = $1 AND user_id = $2
        ORDER BY created_at DESC
        "#,
        guild_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

/// Pardon (soft-delete) a specific warning by ID.
pub async fn pardon(pool: &PgPool, warning_id: i64, guild_id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "UPDATE warnings SET pardoned = TRUE WHERE id = $1 AND guild_id = $2 AND pardoned = FALSE",
        warning_id,
        guild_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
