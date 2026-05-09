use crate::models::economy::UserEconomy;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub async fn get_user_economy(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> sqlx::Result<UserEconomy> {
    let economy = sqlx::query_as!(
        UserEconomy,
        r#"
        SELECT 
            guild_id, user_id, balance, bank, last_daily, last_work, 
            last_rob as "last_rob?", 
            last_crime as "last_crime?", 
            last_fish as "last_fish?", 
            last_hunt as "last_hunt?", 
            total_earned, total_spent, created_at, updated_at
        FROM users_economy
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
        // ensure guild exists first to satisfy foreign key
        crate::db::guild::get_or_create(pool, guild_id).await?;

        // create default
        sqlx::query!(
            "INSERT INTO users_economy (guild_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            guild_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(sqlx::query_as!(
            UserEconomy,
            "SELECT guild_id, user_id, balance, bank, last_daily, last_work, last_rob as \"last_rob?\", last_crime as \"last_crime?\", last_fish as \"last_fish?\", last_hunt as \"last_hunt?\", total_earned, total_spent, created_at, updated_at FROM users_economy WHERE guild_id = $1 AND user_id = $2",
            guild_id,
            user_id
        ).fetch_one(pool).await?)
    }
}

pub async fn update_balance(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    amount: i64,
) -> sqlx::Result<()> {
    // ensure user record exists first
    get_user_economy(pool, guild_id, user_id).await?;

    if amount > 0 {
        sqlx::query!(
            "UPDATE users_economy SET balance = balance + $1, total_earned = total_earned + $1 WHERE guild_id = $2 AND user_id = $3",
            amount,
            guild_id,
            user_id
        ).execute(pool).await?;
    } else {
        sqlx::query!(
            "UPDATE users_economy SET balance = balance + $1, total_spent = total_spent + $2 WHERE guild_id = $3 AND user_id = $4",
            amount,
            amount.abs(),
            guild_id,
            user_id
        ).execute(pool).await?;
    }

    // update global stat
    sqlx::query!(
        "UPDATE global_stats SET stat_value = stat_value + 1 WHERE stat_key = 'total_economy_transactions'"
    ).execute(pool).await?;

    Ok(())
}

pub async fn set_last_daily(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    // ensure user record exists first
    get_user_economy(pool, guild_id, user_id).await?;

    sqlx::query!(
        "UPDATE users_economy SET last_daily = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_last_work(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET last_work = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_last_rob(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET last_rob = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_last_crime(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET last_crime = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_last_fish(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET last_fish = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_last_hunt(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    time: DateTime<Utc>,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET last_hunt = $1 WHERE guild_id = $2 AND user_id = $3",
        time,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_leaderboard(
    pool: &PgPool,
    guild_id: i64,
    limit: i64,
) -> sqlx::Result<Vec<UserEconomy>> {
    sqlx::query_as!(
        UserEconomy,
        "SELECT guild_id, user_id, balance, bank, last_daily, last_work, last_rob as \"last_rob?\", last_crime as \"last_crime?\", last_fish as \"last_fish?\", last_hunt as \"last_hunt?\", total_earned, total_spent, created_at, updated_at FROM users_economy WHERE guild_id = $1 ORDER BY (balance + bank) DESC LIMIT $2",
        guild_id,
        limit
    ).fetch_all(pool).await
}

pub struct GlobalLeaderboardEntry {
    pub user_id: i64,
    pub total_balance: i64,
}

pub async fn get_global_leaderboard(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<GlobalLeaderboardEntry>, sqlx::Error> {
    sqlx::query_as!(
        GlobalLeaderboardEntry,
        "SELECT user_id, SUM(balance + bank)::BIGINT as \"total_balance!\" FROM users_economy GROUP BY user_id ORDER BY SUM(balance + bank) DESC LIMIT $1",
        limit
    )
    .fetch_all(pool)
    .await
}

pub async fn get_local_rank(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let rank = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT rank::BIGINT
        FROM (
            SELECT user_id, RANK() OVER (ORDER BY (balance + bank) DESC) AS rank
            FROM users_economy
            WHERE guild_id = $1
        ) ranked
        WHERE user_id = $2
        "#,
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(rank)
}

pub async fn get_global_rank(pool: &PgPool, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
    let rank = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT rank::BIGINT
        FROM (
            SELECT user_id, RANK() OVER (ORDER BY SUM(balance + bank) DESC) AS rank
            FROM users_economy
            GROUP BY user_id
        ) ranked
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(rank)
}

pub async fn get_inventory_item_count(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(SUM(quantity), 0)::BIGINT FROM economy_inventory WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_total_wealth(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!("SELECT SUM(balance + bank)::BIGINT as \"total!\" FROM users_economy")
        .fetch_one(pool)
        .await?;
    Ok(row.total)
}

pub async fn update_bank(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    amount: i64,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET bank = bank + $1 WHERE guild_id = $2 AND user_id = $3",
        amount,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn transfer_to_bank(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    amount: i64,
) -> sqlx::Result<()> {
    get_user_economy(pool, guild_id, user_id).await?;
    sqlx::query!(
        "UPDATE users_economy SET balance = balance - $1, bank = bank + $1 WHERE guild_id = $2 AND user_id = $3",
        amount,
        guild_id,
        user_id
    ).execute(pool).await?;
    Ok(())
}
