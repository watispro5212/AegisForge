use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use crate::db::economy;

/// Economy commands
#[poise::command(
    slash_command,
    subcommands("balance", "daily", "work", "pay", "leaderboard"),
    category = "Economy"
)]
pub async fn economy(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Check your or someone else's balance
#[poise::command(slash_command)]
pub async fn balance(
    ctx: Context<'_>,
    #[description = "User to check balance of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, target.id.get() as i64).await?;
    
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title(format!("💰 {}'s Balance", target.name))
            .field("Wallet", format!("`${}`", eco.balance), true)
            .field("Bank", format!("`${}`", eco.bank), true)
            .field("Total", format!("`${}`", eco.balance + eco.bank), true)
            .color(0x00E5FF)
        )).await?;
    
    Ok(())
}

/// Claim your daily reward
#[poise::command(slash_command)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    
    if let Some(last) = eco.last_daily {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_hours() < 24 {
            let wait = 24 - diff.num_hours();
            return Err(format!("You already claimed your daily! Wait {} more hours.", wait).into());
        }
    }
    
    let reward = 500;
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_daily(&ctx.data().database.pool, guild_id, user_id, chrono::Utc::now()).await?;
    
    ctx.say(format!("✨ You claimed your daily reward of `${}`!", reward)).await?;
    
    Ok(())
}

/// Work to earn some money
#[poise::command(slash_command)]
pub async fn work(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    // Simple work logic: earn 50-200
    use rand::Rng;
    let reward = rand::thread_rng().gen_range(50..200);
    
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    
    ctx.say(format!("🔨 You worked hard and earned `${}`!", reward)).await?;
    
    Ok(())
}

/// Pay another user from your wallet
#[poise::command(slash_command)]
pub async fn pay(
    ctx: Context<'_>,
    #[description = "User to pay"] user: serenity::User,
    #[description = "Amount to pay"] amount: i64,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;
    
    let author_eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, author_id).await?;
    
    if author_eco.balance < amount {
        return Err("You don't have enough money in your wallet.".into());
    }
    
    economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -amount).await?;
    economy::update_balance(&ctx.data().database.pool, guild_id, user.id.get() as i64, amount).await?;
    
    ctx.say(format!("💸 Sent `${}` to **{}**!", amount, user.name)).await?;
    
    Ok(())
}

/// View the richest users in the server
#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let lb = economy::get_leaderboard(&ctx.data().database.pool, guild_id, 10).await?;
    
    let mut content = String::new();
    for (i, eco) in lb.iter().enumerate() {
        content.push_str(&format!("{}. <@{}> — `${}`\n", i + 1, eco.user_id, eco.balance + eco.bank));
    }
    
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("🏆 Economy Leaderboard")
            .description(content)
            .color(0x00E5FF)
        )).await?;
    
    Ok(())
}
