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

/// check your or someone else's balance
#[poise::command(slash_command, guild_only)]
pub async fn balance(
    ctx: Context<'_>,
    #[description = "User to check (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, target.id.get() as i64).await?;
    let total = eco.balance + eco.bank;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("💰 {}'s Balance", target.name))
            .thumbnail(target.face())
            .field("👛 Wallet", format!("`${}`", eco.balance), true)
            .field("🏦 Bank", format!("`${}`", eco.bank), true)
            .field("📊 Net Worth", format!("`${}`", total), true)
            .footer(serenity::CreateEmbedFooter::new("Bank funds are safe from robbery"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// claim your daily reward
#[poise::command(slash_command, guild_only)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;

    if let Some(last) = eco.last_daily {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_hours() < 24 {
            let remaining_mins = 1440 - diff.num_minutes();
            let hours = remaining_mins / 60;
            let mins = remaining_mins % 60;
            return Err(format!(
                "Daily already claimed! Come back in **{}h {}m**.",
                hours, mins
            ).into());
        }
    }

    let reward: i64 = 500;
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_daily(&ctx.data().database.pool, guild_id, user_id, chrono::Utc::now()).await?;

    let new_total = eco.balance + reward;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✨ Daily Reward Claimed!")
            .description(format!(
                "You claimed your daily reward of **${}**!",
                reward
            ))
            .field("💰 Earned", format!("`+${}`", reward), true)
            .field("👛 New Wallet", format!("`${}`", new_total), true)
            .field("⏰ Next Daily", "<t:{}:R>".replace(
                "{}",
                &(chrono::Utc::now().timestamp() + 86400).to_string()
            ), true)
            .footer(serenity::CreateEmbedFooter::new("Come back every 24h — don't miss a streak!"))
            .color(0x00FF88),
    ))
    .await?;
    Ok(())
}

/// work to earn some cash (30min cooldown)
#[poise::command(slash_command, guild_only)]
pub async fn work(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;

    if let Some(last) = eco.last_work {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_minutes() < 30 {
            let remaining = 30 - diff.num_minutes();
            return Err(format!(
                "You're still tired from last time! Come back in **{}m**.",
                remaining
            ).into());
        }
    }

    let jobs = [
        ("🔧 Fixed a production bug", "Saved the day — client didn't even notice."),
        ("🚚 Delivered packages", "Your back hurts but the tips were worth it."),
        ("🍕 Delivered pizzas", "Three cold calls and one generous tipper."),
        ("💻 Freelanced a website", "Client wanted it \"simple\" — 47 revisions later..."),
        ("🔨 Fixed plumbing", "Charged extra for the 2am emergency call."),
        ("🎨 Designed a logo", "They said the first one was perfect, then changed it."),
        ("📦 Worked at the warehouse", "Hit the quota, earned the bonus."),
        ("🛡️ Forged armor for the realm", "The guild appreciated your craftsmanship."),
    ];
    let (job, flavor) = jobs[rand::thread_rng().gen_range(0..jobs.len())];
    let reward = rand::thread_rng().gen_range(50..=200i64);

    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_work(&ctx.data().database.pool, guild_id, user_id, chrono::Utc::now()).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🔨 Work Complete — {}", job))
            .description(flavor)
            .field("💵 Earned", format!("`+${}`", reward), true)
            .field("⏰ Next Shift", "In 30 minutes", true)
            .footer(serenity::CreateEmbedFooter::new("Work smarter, gamble harder"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// pay another user from your wallet
#[poise::command(slash_command, guild_only)]
pub async fn pay(
    ctx: Context<'_>,
    #[description = "User to pay"] user: serenity::User,
    #[description = "Amount to send"] amount: i64,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    if user.id == ctx.author().id {
        return Err("You can't pay yourself.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;

    let author_eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, author_id).await?;
    if author_eco.balance < amount {
        return Err(format!(
            "Insufficient funds. You only have `${}` in your wallet.",
            author_eco.balance
        ).into());
    }

    economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -amount).await?;
    economy::update_balance(&ctx.data().database.pool, guild_id, user.id.get() as i64, amount).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💸 Transfer Successful")
            .field("Recipient", format!("**{}**", user.name), true)
            .field("Amount", format!("`${}` sent", amount), true)
            .field("Your Balance", format!("`${}`", author_eco.balance - amount), true)
            .footer(serenity::CreateEmbedFooter::new("Wallet-to-wallet transfer"))
            .color(0x00FF88),
    ))
    .await?;
    Ok(())
}

/// view the server's richest members
#[poise::command(slash_command)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Show the global leaderboard instead"] global: Option<bool>,
) -> Result<(), Error> {
    let is_global = global.unwrap_or(false);

    let (title, footer, content) = if is_global {
        let lb = economy::get_global_leaderboard(&ctx.data().database.pool, 10).await?;
        let mut desc = String::new();
        let medals = ["🥇", "🥈", "🥉"];
        for (i, entry) in lb.iter().enumerate() {
            let prefix = medals.get(i).copied().unwrap_or("🏅");
            desc.push_str(&format!("{} <@{}> — `${}`\n", prefix, entry.user_id, entry.total_balance));
        }
        if desc.is_empty() { desc = "_No data yet — start earning!_".to_string(); }
        (
            "🌍 Global Wealth Leaderboard",
            "Top 10 across all servers in the AegisForge network",
            desc,
        )
    } else {
        let guild_id = ctx.guild_id().unwrap().get() as i64;
        let lb = economy::get_leaderboard(&ctx.data().database.pool, guild_id, 10).await?;
        let mut desc = String::new();
        let medals = ["🥇", "🥈", "🥉"];
        for (i, eco) in lb.iter().enumerate() {
            let prefix = medals.get(i).copied().unwrap_or("🏅");
            desc.push_str(&format!("{} <@{}> — `${}`\n", prefix, eco.user_id, eco.balance + eco.bank));
        }
        if desc.is_empty() { desc = "_No data yet — start earning!_".to_string(); }
        (
            "🏆 Server Wealth Leaderboard",
            "Top 10 wealthiest in this server",
            desc,
        )
    };

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(title)
            .description(content)
            .footer(serenity::CreateEmbedFooter::new(footer))
            .timestamp(serenity::Timestamp::now())
            .color(0xFFD700),
    ))
    .await?;
    Ok(())
}

/// deposit money into your bank
#[poise::command(slash_command, guild_only)]
pub async fn deposit(
    ctx: Context<'_>,
    #[description = "Amount to deposit"] amount: i64,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.balance {
        return Err(format!(
            "You only have `${}` in your wallet.",
            eco.balance
        ).into());
    }

    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, amount).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🏦 Deposit Successful")
            .field("💰 Deposited", format!("`${}` → Bank", amount), true)
            .field("👛 Wallet Remaining", format!("`${}`", eco.balance - amount), true)
            .footer(serenity::CreateEmbedFooter::new("Banked funds are safe from robbery"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// withdraw money from your bank
#[poise::command(slash_command, guild_only)]
pub async fn withdraw(
    ctx: Context<'_>,
    #[description = "Amount to withdraw"] amount: i64,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.bank {
        return Err(format!(
            "You only have `${}` in your bank.",
            eco.bank
        ).into());
    }

    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, -amount).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💰 Withdrawal Successful")
            .field("💵 Withdrawn", format!("`${}` → Wallet", amount), true)
            .field("🏦 Bank Remaining", format!("`${}`", eco.bank - amount), true)
            .footer(serenity::CreateEmbedFooter::new("Be careful — wallet funds can be robbed"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// beg for spare change
#[poise::command(slash_command, guild_only)]
pub async fn beg(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let givers = ["A kind stranger", "A passing merchant", "An old wizard", "A generous guild member", "A cloaked figure"];
    let giver = givers[rand::thread_rng().gen_range(0..givers.len())];
    let reward = rand::thread_rng().gen_range(0i64..=50);

    let embed = if reward == 0 {
        serenity::CreateEmbed::new()
            .title("🥺 Nobody's Feeling Generous")
            .description("You held out your hand but everyone walked by. Try again later.")
            .color(0xFF5722)
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
        serenity::CreateEmbed::new()
            .title("🤲 Begging Results")
            .description(format!("**{}** took pity and tossed you some coins!", giver))
            .field("💰 Received", format!("`+${}`", reward), true)
            .color(0x00E5FF)
    };

    ctx.send(poise::CreateReply::default().embed(
        embed.footer(serenity::CreateEmbedFooter::new("Consider working for a living")),
    ))
    .await?;
    Ok(())
}

/// search around for hidden money
#[poise::command(slash_command, guild_only)]
pub async fn search(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let locations = [
        ("under the sofa cushions", "🛋️"),
        ("in a dumpster behind the forge", "🗑️"),
        ("at the bottom of an old coat pocket", "🧥"),
        ("behind a vending machine", "🎰"),
        ("in the park near the fountain", "🏞️"),
        ("inside an old piggy bank", "🐷"),
        ("under a loose floorboard", "🪵"),
        ("in a crumpled receipt", "🧾"),
    ];
    let (location, emoji) = locations[rand::thread_rng().gen_range(0..locations.len())];
    let reward = rand::thread_rng().gen_range(10i64..=150);

    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("{} Search Results", emoji))
            .description(format!("You rummaged around **{}**...", location))
            .field("💰 Found", format!("`+${}`", reward), true)
            .footer(serenity::CreateEmbedFooter::new("Finders keepers"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// spin the slot machine
#[poise::command(slash_command, guild_only)]
pub async fn slots(
    ctx: Context<'_>,
    #[description = "Amount to bet (minimum $10)"] bet: i64,
) -> Result<(), Error> {
    if bet < 10 {
        return Err("Minimum bet is **$10**.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if bet > eco.balance {
        return Err(format!(
            "Insufficient funds. You only have `${}` in your wallet.",
            eco.balance
        ).into());
    }

    let emojis = ["🍒", "🍋", "🍇", "💎", "⭐"];
    // Compute all random values inside a block so ThreadRng is dropped before any await
    let (r1, r2, r3, protection_win) = {
        let mut rng = rand::thread_rng();
        (
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())],
            rng.gen_bool(0.6),
        )
    };

    let has_diamond = r1 == "💎" || r2 == "💎" || r3 == "💎";

    let (won, multiplier, label) = if r1 == r2 && r2 == r3 {
        match r1 {
            "💎" => (true, 25.0f64, "💎 HYPERFORGE JACKPOT! 💎"),
            "⭐" => (true, 15.0, "⭐ SUPER TRIPLE! ⭐"),
            _ => (true, 10.0, "✨ TRIPLE MATCH! ✨"),
        }
    } else if r1 == r2 || r2 == r3 || r1 == r3 {
        (true, 5.0, "🍀 Double Match!")
    } else if has_diamond {
        (true, 2.0, "💎 Diamond Wild!")
    } else if protection_win {
        (true, 1.2, "🛡️ Aegis Protection Win!")
    } else {
        (false, 0.0, "The forge rejects your bet.")
    };

    let slot_display = format!("[ {} | {} | {} ]", r1, r2, r3);

    if won {
        let prize = (bet as f64 * multiplier) as i64;
        let profit = prize - bet;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, profit).await?;
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("🎰 {}", label))
                .description(format!("> {}", slot_display))
                .field("🎯 Bet", format!("`${}`", bet), true)
                .field("💰 Won", format!("`${}`", prize), true)
                .field("📈 Profit", format!("`+${}`", profit), true)
                .footer(serenity::CreateEmbedFooter::new(format!("{}x multiplier | FairForge™ algorithm", multiplier)))
                .color(0x00FF88),
        ))
        .await?;
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🎰 No Match")
                .description(format!("> {}\n\n{}", slot_display, label))
                .field("💸 Lost", format!("`-${}`", bet), true)
                .footer(serenity::CreateEmbedFooter::new("Only 18.5% of spins fully lose — you'll get there"))
                .color(0xFF3B3B),
        ))
        .await?;
    }
    Ok(())
}

/// view slot machine odds and payout table
#[poise::command(slash_command, guild_only)]
pub async fn gamble_info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎰 Hyperforge Slot Machine — Payout Table")
            .description("The FairForge™ algorithm ensures ~81.5% total win probability per spin.")
            .field(
                "Payouts",
                "💎💎💎 Three Diamonds → **25.0x**\n\
                 ⭐⭐⭐ Three Stars → **15.0x**\n\
                 🍒🍒🍒 Any other triple → **10.0x**\n\
                 X X — Any two matching → **5.0x**\n\
                 — 💎 — Single Diamond (no match) → **2.0x**\n\
                 🛡️ Aegis Protection (60% on loss) → **1.2x**\n\
                 ✖️ Full loss → **0x** (~18.5% of spins)",
                false,
            )
            .field("Minimum Bet", "`$10`", true)
            .field("Win Rate", "~81.5%", true)
            .field("Max Payout", "25x bet (jackpot)", true)
            .footer(serenity::CreateEmbedFooter::new("Bank your winnings to protect them from /economy rob"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// view the global wealth leaderboard
#[poise::command(slash_command, guild_only)]
pub async fn global_leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let lb = economy::get_global_leaderboard(&ctx.data().database.pool, 10).await?;

    let medals = ["🥇", "🥈", "🥉"];
    let mut description = String::new();
    for (i, entry) in lb.iter().enumerate() {
        let prefix = medals.get(i).copied().unwrap_or("🏅");
        description.push_str(&format!(
            "{} <@{}> — `${}`\n",
            prefix, entry.user_id, entry.total_balance
        ));
    }
    if description.is_empty() {
        description = "No wealth forged yet — be the first!".into();
    }

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌍 Global Wealth Leaderboard")
            .description(description)
            .footer(serenity::CreateEmbedFooter::new("Across all guilds in the AegisForge network"))
            .timestamp(serenity::Timestamp::now())
            .color(0xFFD700),
    ))
    .await?;
    Ok(())
}

/// rob another user's wallet
#[poise::command(slash_command, guild_only)]
pub async fn rob(
    ctx: Context<'_>,
    #[description = "User to rob"] user: serenity::User,
) -> Result<(), Error> {
    if user.id == ctx.author().id {
        return Err("You can't rob yourself.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;
    let target_id = user.id.get() as i64;

    let target_eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, target_id).await?;
    if target_eco.balance < 100 {
        return Err(format!(
            "**{}** doesn't have enough in their wallet to bother (need at least `$100`).",
            user.name
        ).into());
    }

    // Drop ThreadRng before any await — ThreadRng is !Send
    let (success, stolen_fraction) = {
        let mut rng = rand::thread_rng();
        (rng.gen_bool(0.4), rng.gen_range(0.1_f64..0.5))
    };

    if success {
        let stolen = (target_eco.balance as f64 * stolen_fraction) as i64;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, stolen).await?;
        economy::update_balance(&ctx.data().database.pool, guild_id, target_id, -stolen).await?;

        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🥷 Heist Successful!")
                .description(format!("You slipped into **{}**'s room undetected.", user.name))
                .field("💰 Stolen", format!("`${}` ({:.0}% of their wallet)", stolen, stolen_fraction * 100.0), false)
                .footer(serenity::CreateEmbedFooter::new("You got away clean — this time"))
                .color(0x00FF88),
        ))
        .await?;
    } else {
        let fine: i64 = 200;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -fine).await?;
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("👮 Caught Red-Handed!")
                .description(format!("You were caught trying to rob **{}**!", user.name))
                .field("💸 Fine Paid", format!("`-${}`", fine), true)
                .footer(serenity::CreateEmbedFooter::new("Crime doesn't pay — 60% of the time"))
                .color(0xFF3B3B),
        ))
        .await?;
    }
    Ok(())
}
