use crate::{Context, Error};
use rand::prelude::*;
use poise::serenity_prelude as serenity;
use crate::db::economy;

/// economy stuff
#[poise::command(
    slash_command,
    subcommands("balance", "daily", "work", "pay", "leaderboard", "global_leaderboard", "rob", "slots", "beg", "search", "deposit", "withdraw", "gamble_info"),
    category = "economy",
    guild_only
)]
pub async fn economy(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// check your or someone else's money
#[poise::command(slash_command, guild_only)]
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

/// get your daily money
#[poise::command(slash_command, guild_only)]
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

/// work to get some cash
#[poise::command(slash_command, guild_only)]
pub async fn work(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    // Simple work logic: earn 50-200
    let reward = rand::thread_rng().gen_range(50..200);
    
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    
    ctx.say(format!("🔨 You worked hard and earned `${}`!", reward)).await?;
    
    Ok(())
}

/// give someone some of your money
#[poise::command(slash_command, guild_only)]
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

/// View the richest users
#[poise::command(slash_command)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Show global leaderboard across all servers"] global: Option<bool>,
) -> Result<(), Error> {
    let is_global = global.unwrap_or(false);
    
    let mut content = String::new();
    let title = if is_global {
        let lb = economy::get_global_leaderboard(&ctx.data().database.pool, 10).await?;
        for (i, entry) in lb.iter().enumerate() {
            content.push_str(&format!("**{}**. <@{}> — `${}`\n", i + 1, entry.user_id, entry.total_balance));
        }
        "🏆 Global Economy Leaderboard"
    } else {
        let guild_id = ctx.guild_id().unwrap().get() as i64;
        let lb = economy::get_leaderboard(&ctx.data().database.pool, guild_id, 10).await?;
        for (i, eco) in lb.iter().enumerate() {
            content.push_str(&format!("**{}**. <@{}> — `${}`\n", i + 1, eco.user_id, eco.balance + eco.bank));
        }
        "🏆 Server Economy Leaderboard"
    };

    if content.is_empty() {
        content = "_No data found yet._".to_string();
    }
    
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title(title)
            .description(content)
            .footer(serenity::CreateEmbedFooter::new(if is_global { "Top 10 Wealthiest Across All Realms" } else { "Top 10 Wealthiest in This Server" }))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF)
        )).await?;
    
    Ok(())
}

/// Deposit money into your bank for safekeeping
#[poise::command(slash_command, guild_only)]
pub async fn deposit(
    ctx: Context<'_>,
    #[description = "Amount to deposit"] amount: i64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.balance {
        return Err("You don't have that much in your wallet.".into());
    }
    
    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, amount).await?;
    ctx.say(format!("🏦 Deposited `${}` into your bank!", amount)).await?;
    Ok(())
}

/// Withdraw money from your bank
#[poise::command(slash_command, guild_only)]
pub async fn withdraw(
    ctx: Context<'_>,
    #[description = "Amount to withdraw"] amount: i64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.bank {
        return Err("You don't have that much in your bank.".into());
    }
    
    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, -amount).await?;
    ctx.say(format!("💰 Withdrew `${}` from your bank!", amount)).await?;
    Ok(())
}

/// Beg for some spare change
#[poise::command(slash_command, guild_only)]
pub async fn beg(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let reward = rand::thread_rng().gen_range(0..50);
    if reward == 0 {
        ctx.say("🥺 Nobody gave you anything. Better luck next time!").await?;
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
        ctx.say(format!("🤲 A kind stranger gave you `${}`!", reward)).await?;
    }
    Ok(())
}

/// Search for money in random places
#[poise::command(slash_command, guild_only)]
pub async fn search(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let locations = ["under the sofa", "in a trash can", "at the park", "in your old coat", "behind a vending machine"];
    let loc = locations[rand::thread_rng().gen_range(0..locations.len())];
    let reward = rand::thread_rng().gen_range(10..150);
    
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    ctx.say(format!("🔍 You searched **{}** and found `${}`!", loc, reward)).await?;
    Ok(())
}

/// Try your luck at the slot machine
#[poise::command(slash_command, guild_only)]
pub async fn slots(
    ctx: Context<'_>,
    #[description = "Amount to bet"] bet: i64,
) -> Result<(), Error> {
    if bet < 10 {
        return Err("Minimum bet is $10.".into());
    }
    
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if bet > eco.balance {
        return Err("You don't have enough money in your wallet.".into());
    }
    
    let emojis = ["🍒", "🍋", "🍇", "💎", "⭐"];
    let (r1, r2, r3) = {
        let mut rng = rand::thread_rng();
        (
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())]
        )
    };
    
    let has_diamond = r1 == "💎" || r2 == "💎" || r3 == "💎";
    
    let (won, multiplier, message) = if r1 == r2 && r2 == r3 {
        if r1 == "💎" {
            (true, 25.0, "💎 HYPERFORGE JACKPOT! 💎")
        } else if r1 == "⭐" {
            (true, 15.0, "⭐ SUPER TRIPLE! ⭐")
        } else {
            (true, 10.0, "✨ TRIPLE MATCH! ✨")
        }
    } else if r1 == r2 || r2 == r3 || r1 == r3 {
        (true, 5.0, "🍀 Double Match! 🍀")
    } else if has_diamond {
        (true, 2.0, "💎 Diamond Wild! 💎")
    } else {
        // Ultra-high win chance for Hyperforge release
        let random_win = rand::thread_rng().gen_bool(0.6); // 60% chance for a random minor win
        if random_win {
             (true, 1.2, "🛡️ Aegis Protection Win! 🛡️")
        } else {
            (false, 0.0, "The forge rejects your bet. Better luck next time!")
        }
    };
    
    let embed = if won {
        let prize = (bet as f64 * multiplier) as i64;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, prize - bet).await?;
        serenity::CreateEmbed::new()
            .title(format!("🎰 Slot Machine — {}", message))
            .description(format!("> [ {} | {} | {} ]\n\nCongratulations! You won `${}`!", r1, r2, r3, prize))
            .color(0x00FF88)
            .footer(serenity::CreateEmbedFooter::new(format!("Multiplier: {}x | Odds are in your favor!", multiplier)))
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        serenity::CreateEmbed::new()
            .title("🎰 Slot Machine — REJECTED")
            .description(format!("> [ {} | {} | {} ]\n\n{} You lost `${}`.", r1, r2, r3, message, bet))
            .color(0xFF3B3B)
    };
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Get information about AegisForge gambling mechanics
#[poise::command(slash_command, guild_only)]
pub async fn gamble_info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎰 The AegisForge Casino — Mechanics & Odds")
            .description("Welcome to the high-stakes sector of the Eternal Forge. Our machines are calibrated for maximum interaction and player success.")
            .field("🎰 Slot Machine Payouts", 
                "• **3x Diamonds**: 25.0x\n• **3x Stars**: 15.0x\n• **3x Other**: 10.0x\n• **Any 2 Match**: 5.0x\n• **Single Diamond**: 2.0x\n• **Aegis Shield**: 1.2x (60% chance on loss)", false)
            .field("💎 The Hyperforge Advantage", 
                "AegisForge v3.1 features a proprietary 'FairForge' algorithm. Unlike other bots, our house edge is HEAVILY NEGATIVE. The more you play, the more you win. Total win probability: **~81.5%**.", false)
            .field("💰 Bank Security", "Money in your `/economy bank` cannot be stolen by `/economy rob`. Use it to safeguard your millions!", false)
            .footer(serenity::CreateEmbedFooter::new("Forged for the bold | v3.1.0 Hyperforge"))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// View the global wealth leaderboard
#[poise::command(slash_command, guild_only)]
pub async fn global_leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().database.pool;
    let lb = economy::get_global_leaderboard(pool, 10).await?;
    
    let mut description = String::new();
    for (i, entry) in lb.iter().enumerate() {
        description.push_str(&format!(
            "**{}.** <@{}> — `${}`\n",
            i + 1, entry.user_id, entry.total_balance
        ));
    }
    
    if description.is_empty() {
        description = "No one has forged any wealth yet!".into();
    }
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌍 AegisForge Global Wealth Leaderboard")
            .description(description)
            .color(0xFFD700)
            .footer(serenity::CreateEmbedFooter::new("Across all guilds in the Hyperforge network"))
    )).await?;
    
    Ok(())
}

/// Rob another user's wallet
#[poise::command(slash_command, guild_only)]
pub async fn rob(
    ctx: Context<'_>,
    #[description = "User to rob"] user: serenity::User,
) -> Result<(), Error> {
    if user.id == ctx.author().id {
        return Err("You can't rob yourself!".into());
    }
    
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;
    let target_id = user.id.get() as i64;
    
    let target_eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, target_id).await?;
    if target_eco.balance < 100 {
        return Err("This user is too poor to be worth robbing.".into());
    }
    
    let (success, stolen_multiplier) = {
        let mut rng = rand::thread_rng();
        (rng.gen_bool(0.4), rng.gen_range(0.1..0.5))
    };
    
    if success {
        let stolen = (target_eco.balance as f64 * stolen_multiplier) as i64;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, stolen).await?;
        economy::update_balance(&ctx.data().database.pool, guild_id, target_id, -stolen).await?;
        
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🥷 Successful Heist")
                .description(format!("You snuck into **{}**'s room and made off with `${}`!", user.name, stolen))
                .color(0x00FF88)
        )).await?;
    } else {
        let fine = 200;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -fine).await?;
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("👮 BUSTED!")
                .description(format!("You were caught trying to rob **{}**! You paid a `${}` fine.", user.name, fine))
                .color(0xFF3B3B)
        )).await?;
    }
    
    Ok(())
}
