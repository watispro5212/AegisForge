use sqlx::PgPool;
use crate::models::economy::UserEconomy;
use chrono::{DateTime, Utc};

pub async fn get_user_economy(pool: &PgPool, guild_id: i64, user_id: i64) -> sqlx::Result<UserEconomy> {
    let economy = sqlx::query_as!(
        UserEconomy,
        r#"
        SELECT * FROM users_economy
        WHERE guild_id = $1 AND user_id = $2
        "#,
        guild_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(eco) = economy {
        Ok(eco)
    } else {
        // Ensure guild exists first to satisfy foreign key
        crate::db::guild::get_or_create(pool, guild_id).await?;

        // Create default
        sqlx::query!(
            "INSERT INTO users_economy (guild_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            guild_id,
            user_id
        ).execute(pool).await?;

        Ok(sqlx::query_as!(
            UserEconomy,
            "SELECT * FROM users_economy WHERE guild_id = $1 AND user_id = $2",
            guild_id,
            user_id
        ).fetch_one(pool).await?)
    }
}

pub async fn update_balance(pool: &PgPool, guild_id: i64, user_id: i64, amount: i64) -> sqlx::Result<()> {
    // Ensure user record exists first
    get_user_economy(pool, guild_id, user_id).await?;

    sqlx::query!(
        "UPDATE users_economy SET balance = balance + $1 WHERE guild_id = $2 AND user_id = $3",
        amount,
        guild_id,
        user_id
    ).execute(pool).await?;
    Ok(())
}

pub async fn set_last_daily(pool: &PgPool, guild_id: i64, user_id: i64, time: DateTime<Utc>) -> sqlx::Result<()> {
    // Ensure user record exists first
    get_user_economy(pool, guild_id, user_id).await?;

    sqlx::query!(
        "UPDATE users_economy SET last_daily = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    ).execute(pool).await?;
    Ok(())
}

pub async fn get_leaderboard(pool: &PgPool, guild_id: i64, limit: i64) -> sqlx::Result<Vec<UserEconomy>> {
    sqlx::query_as!(
        UserEconomy,
        "SELECT * FROM users_economy WHERE guild_id = $1 ORDER BY balance DESC LIMIT $2",
        guild_id,
        limit
    ).fetch_all(pool).await
}
