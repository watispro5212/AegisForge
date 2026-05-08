use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::models::reminder::Reminder;

/// create a new reminder.
pub async fn create(
    pool: &PgPool,
    user_id: i64,
    channel_id: i64,
    guild_id: Option<i64>,
    message: &str,
    fire_at: DateTime<Utc>,
) -> sqlx::Result<Reminder> {
    sqlx::query_as!(
        Reminder,
        r#"
        INSERT INTO reminders (user_id, channel_id, guild_id, message, fire_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        user_id,
        channel_id,
        guild_id,
        message,
        fire_at
    )
    .fetch_one(pool)
    .await
}

/// fetch all pending (unfired) reminders due at or before `now`.
/// called by the background polling task every ~5 seconds.
pub async fn get_pending(pool: &PgPool) -> sqlx::Result<Vec<Reminder>> {
    sqlx::query_as!(
        Reminder,
        r#"
        SELECT * FROM reminders
        WHERE fired = FALSE AND fire_at <= NOW()
        ORDER BY fire_at ASC
        LIMIT 200
        "#
    )
    .fetch_all(pool)
    .await
}

/// mark a reminder as fired.
pub async fn mark_fired(pool: &PgPool, reminder_id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE reminders SET fired = TRUE WHERE id = $1",
        reminder_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// fetch all reminders for a specific user (for a list command).
pub async fn get_for_user(pool: &PgPool, user_id: i64) -> sqlx::Result<Vec<Reminder>> {
    sqlx::query_as!(
        Reminder,
        r#"
        SELECT * FROM reminders
        WHERE user_id = $1 AND fired = FALSE
        ORDER BY fire_at ASC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

/// cancel (delete) a specific reminder — only if it belongs to the user.
pub async fn delete(pool: &PgPool, reminder_id: i64, user_id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "DELETE FROM reminders WHERE id = $1 AND user_id = $2 AND fired = FALSE",
        reminder_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

