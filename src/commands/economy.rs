use crate::db::economy;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use rand::prelude::*;
use sqlx::Row;

/// economy stuff
#[poise::command(
    slash_command,
    subcommands(
        "balance",
        "daily",
        "work",
        "pay",
        "leaderboard",
        "global_leaderboard",
        "rob",
        "slots",
        "beg",
        "search",
        "deposit",
        "withdraw",
        "gamble_info",
        "shop",
        "buy",
        "inventory",
        "profile",
        "crime",
        "fish",
        "hunt",
        "blackjack",
        "coinflip",
        "dice",
        "work_list"
    ),
    category = "economy",
    guild_only
)]
pub async fn economy(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[derive(Clone, Copy)]
struct ShopItem {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    price: i64,
    description: &'static str,
    rarity: &'static str,
}

fn global_shop_items() -> &'static [ShopItem] {
    &[
        ShopItem {
            id: "copper_badge",
            name: "Copper Badge",
            category: "Profile",
            price: 250,
            rarity: "Common",
            description: "Starter profile badge for fresh economy players.",
        },
        ShopItem {
            id: "silver_badge",
            name: "Silver Badge",
            category: "Profile",
            price: 1_000,
            rarity: "Common",
            description: "Clean profile flex for consistent grinders.",
        },
        ShopItem {
            id: "gold_badge",
            name: "Gold Badge",
            category: "Profile",
            price: 5_000,
            rarity: "Rare",
            description: "Premium-looking badge for leaderboard climbers.",
        },
        ShopItem {
            id: "diamond_badge",
            name: "Diamond Badge",
            category: "Profile",
            price: 25_000,
            rarity: "Epic",
            description: "High-end badge for economy veterans.",
        },
        ShopItem {
            id: "forge_crown",
            name: "Forge Crown",
            category: "Profile",
            price: 100_000,
            rarity: "Legendary",
            description: "A crown for users with dangerous amounts of money.",
        },
        ShopItem {
            id: "neon_nameplate",
            name: "Neon Nameplate",
            category: "Cosmetic",
            price: 7_500,
            rarity: "Rare",
            description: "Bright profile styling for rank and economy displays.",
        },
        ShopItem {
            id: "carbon_nameplate",
            name: "Carbon Nameplate",
            category: "Cosmetic",
            price: 9_000,
            rarity: "Rare",
            description: "Dark brushed-metal profile styling.",
        },
        ShopItem {
            id: "aurora_frame",
            name: "Aurora Frame",
            category: "Cosmetic",
            price: 18_000,
            rarity: "Epic",
            description: "Animated-feeling profile frame for top users.",
        },
        ShopItem {
            id: "obsidian_frame",
            name: "Obsidian Frame",
            category: "Cosmetic",
            price: 30_000,
            rarity: "Epic",
            description: "Stealth profile frame with sharp forge styling.",
        },
        ShopItem {
            id: "founders_sigil",
            name: "Founder's Sigil",
            category: "Collectible",
            price: 50_000,
            rarity: "Legendary",
            description: "Permanent collectible for early supporters.",
        },
        ShopItem {
            id: "rustacean_relic",
            name: "Rustacean Relic",
            category: "Collectible",
            price: 75_000,
            rarity: "Legendary",
            description: "A rare relic honoring the Rust core.",
        },
        ShopItem {
            id: "lucky_charm",
            name: "Lucky Charm",
            category: "Boost",
            price: 2_500,
            rarity: "Common",
            description: "Cosmetic luck token for gamblers.",
        },
        ShopItem {
            id: "work_permit",
            name: "Work Permit",
            category: "Boost",
            price: 4_000,
            rarity: "Common",
            description: "Roleplay item for dedicated workers.",
        },
        ShopItem {
            id: "fishing_rod",
            name: "Fishing Rod",
            category: "Boost",
            price: 6_000,
            rarity: "Rare",
            description: "Fishing-themed collectible for lake regulars.",
        },
        ShopItem {
            id: "hunters_kit",
            name: "Hunter's Kit",
            category: "Boost",
            price: 8_000,
            rarity: "Rare",
            description: "Hunting-themed collectible for wilderness grinders.",
        },
        ShopItem {
            id: "vault_pass",
            name: "Vault Pass",
            category: "Utility",
            price: 12_000,
            rarity: "Rare",
            description: "Banking-themed flex item for cautious players.",
        },
        ShopItem {
            id: "anti_heist_tag",
            name: "Anti-Heist Tag",
            category: "Utility",
            price: 15_000,
            rarity: "Epic",
            description: "A warning label for anyone thinking about robbing you.",
        },
        ShopItem {
            id: "market_pass",
            name: "Market Pass",
            category: "Utility",
            price: 20_000,
            rarity: "Epic",
            description: "Collector pass for future marketplace features.",
        },
        ShopItem {
            id: "beta_tester_tag",
            name: "Beta Tester Tag",
            category: "Community",
            price: 3_000,
            rarity: "Common",
            description: "Show that you helped test AegisForge builds.",
        },
        ShopItem {
            id: "bug_hunter_tag",
            name: "Bug Hunter Tag",
            category: "Community",
            price: 6_500,
            rarity: "Rare",
            description: "For people who find bugs before production does.",
        },
        ShopItem {
            id: "support_hero_tag",
            name: "Support Hero Tag",
            category: "Community",
            price: 10_000,
            rarity: "Rare",
            description: "Recognition for helpful support-server regulars.",
        },
        ShopItem {
            id: "patch_notes_enjoyer",
            name: "Patch Notes Enjoyer",
            category: "Community",
            price: 1_500,
            rarity: "Common",
            description: "For the brave souls who actually read changelogs.",
        },
        ShopItem {
            id: "season_token",
            name: "Season Token",
            category: "Limited",
            price: 35_000,
            rarity: "Epic",
            description: "Rotating seasonal collectible slot.",
        },
        ShopItem {
            id: "hyperforge_core",
            name: "Hyperforge Core",
            category: "Limited",
            price: 250_000,
            rarity: "Mythic",
            description: "Ultra-expensive global chase item.",
        },
    ]
}

fn format_money(amount: i64) -> String {
    format!("${}", amount)
}

fn find_shop_item(query: &str) -> Option<ShopItem> {
    let needle = query.trim().to_lowercase().replace(' ', "_");
    global_shop_items().iter().copied().find(|item| {
        item.id.eq_ignore_ascii_case(&needle)
            || item.name.eq_ignore_ascii_case(query.trim())
            || item.name.to_lowercase().replace(' ', "_") == needle
    })
}

/// view the global AegisForge shop catalog
#[poise::command(slash_command, guild_only)]
pub async fn shop(
    ctx: Context<'_>,
    #[description = "Optional category filter: profile, cosmetic, collectible, boost, utility, community, limited"]
    category: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let filter = category.as_ref().map(|c| c.to_lowercase());
    let items: Vec<ShopItem> = global_shop_items()
        .iter()
        .copied()
        .filter(|item| {
            filter
                .as_ref()
                .map(|needle| item.category.to_lowercase().contains(needle))
                .unwrap_or(true)
        })
        .collect();

    if items.is_empty() {
        return Err("No shop items matched that category. Try `profile`, `cosmetic`, `boost`, `utility`, `community`, or `limited`.".into());
    }

    let categories = [
        "Profile",
        "Cosmetic",
        "Collectible",
        "Boost",
        "Utility",
        "Community",
        "Limited",
    ];

    let mut embed = CreateEmbed::new()
        .title("AegisForge Global Shop")
        .description(format!(
            "Global catalog, auto-rendered from the current shop list. **{}** item(s) available{}.",
            items.len(),
            filter
                .as_ref()
                .map(|c| format!(" for `{}`", c))
                .unwrap_or_default()
        ))
        .footer(serenity::CreateEmbedFooter::new(
            "Use /economy buy <item_id> to purchase and /economy inventory to view owned items.",
        ))
        .timestamp(serenity::Timestamp::now())
        .color(0xFFD700);

    for category_name in categories {
        let lines: Vec<String> = items
            .iter()
            .filter(|item| item.category == category_name)
            .map(|item| {
                format!(
                    "**{}** (`{}`) - `{}` [{}]\n{}",
                    item.name,
                    item.id,
                    format_money(item.price),
                    item.rarity,
                    item.description
                )
            })
            .collect();

        if !lines.is_empty() {
            embed = embed.field(category_name, lines.join("\n\n"), false);
        }
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// buy an item from the global shop
#[poise::command(slash_command, guild_only)]
pub async fn buy(
    ctx: Context<'_>,
    #[description = "Shop item id or exact item name"] item: String,
    #[description = "Quantity to buy (defaults to 1)"] quantity: Option<i64>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let item = find_shop_item(&item).ok_or_else(|| {
        "Unknown shop item. Use `/economy shop` to view valid item ids.".to_string()
    })?;
    let quantity = quantity.unwrap_or(1).clamp(1, 25);
    let total_price = item.price * quantity;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if eco.balance < total_price {
        return Err(format!(
            "You need `{}` in your wallet to buy **{}x {}**, but you only have `${}`.",
            format_money(total_price),
            quantity,
            item.name,
            eco.balance
        )
        .into());
    }

    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -total_price).await?;

    sqlx::query(
        r#"
        INSERT INTO economy_inventory (guild_id, user_id, item_id, item_name, quantity)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (guild_id, user_id, item_id)
        DO UPDATE SET
            quantity = economy_inventory.quantity + EXCLUDED.quantity,
            item_name = EXCLUDED.item_name,
            last_purchased_at = NOW()
        "#,
    )
    .bind(guild_id)
    .bind(user_id)
    .bind(item.id)
    .bind(item.name)
    .bind(quantity)
    .execute(&ctx.data().database.pool)
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("Shop Purchase Complete")
                .description(format!(
                    "You bought **{}x {}** from the global shop.",
                    quantity, item.name
                ))
                .field("Category", item.category, true)
                .field("Rarity", item.rarity, true)
                .field(
                    "Total Cost",
                    format!("`{}`", format_money(total_price)),
                    true,
                )
                .field(
                    "Wallet Remaining",
                    format!("`${}`", eco.balance - total_price),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Use /economy inventory to view your items.",
                ))
                .color(0x00FF88),
        ),
    )
    .await?;

    Ok(())
}

/// view your purchased shop items
#[poise::command(slash_command, guild_only)]
pub async fn inventory(
    ctx: Context<'_>,
    #[description = "User to inspect (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let target_id = target.id.get() as i64;

    economy::get_user_economy(&ctx.data().database.pool, guild_id, target_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT item_id, item_name, quantity
        FROM economy_inventory
        WHERE guild_id = $1 AND user_id = $2
        ORDER BY item_name ASC
        "#,
    )
    .bind(guild_id)
    .bind(target_id)
    .fetch_all(&ctx.data().database.pool)
    .await?;

    let description = if rows.is_empty() {
        "_No items yet. Browse `/economy shop` and buy something shiny._".to_string()
    } else {
        rows.iter()
            .map(|row| {
                let item_id: String = row.get("item_id");
                let item_name: String = row.get("item_name");
                let quantity: i64 = row.get("quantity");
                let rarity = find_shop_item(&item_id)
                    .map(|item| item.rarity)
                    .unwrap_or("Legacy");
                format!(
                    "**{}** x{} (`{}`) - {}",
                    item_name, quantity, item_id, rarity
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("{}'s Inventory", target.name))
                .description(description)
                .thumbnail(target.face())
                .footer(serenity::CreateEmbedFooter::new(
                    "Inventory is stored per server economy.",
                ))
                .color(0xBF5AF2),
        ),
    )
    .await?;

    Ok(())
}

/// view a compact economy profile with balances, ranks, and inventory count
#[poise::command(slash_command, guild_only)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "User to inspect (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let target_id = target.id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, target_id).await?;
    let net_worth = eco.balance + eco.bank;
    let local_rank = economy::get_local_rank(&ctx.data().database.pool, guild_id, target_id)
        .await?
        .map(|rank| format!("#{}", rank))
        .unwrap_or_else(|| "Unranked".to_string());
    let global_rank = economy::get_global_rank(&ctx.data().database.pool, target_id)
        .await?
        .map(|rank| format!("#{}", rank))
        .unwrap_or_else(|| "Unranked".to_string());
    let inventory_items =
        economy::get_inventory_item_count(&ctx.data().database.pool, guild_id, target_id).await?;

    let safety_ratio = if net_worth <= 0 {
        0.0
    } else {
        (eco.bank as f64 / net_worth as f64 * 100.0).clamp(0.0, 100.0)
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("{}'s Economy Profile", target.name))
                .description("A v4.1 snapshot of wallet health, rankings, and owned shop items.")
                .thumbnail(target.face())
                .field("Wallet", format!("`${}`", eco.balance), true)
                .field("Bank", format!("`${}`", eco.bank), true)
                .field("Net Worth", format!("`${}`", net_worth), true)
                .field("Server Rank", local_rank, true)
                .field("Global Rank", global_rank, true)
                .field("Inventory Items", format!("`{}`", inventory_items), true)
                .field(
                    "Lifetime Flow",
                    format!(
                        "Earned: `${}`\nSpent: `${}`\nBank Safety: `{:.1}%`",
                        eco.total_earned, eco.total_spent, safety_ratio
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Use /economy shop, /economy buy, and /economy inventory to build your profile.",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;

    Ok(())
}

/// check your or someone else's balance
#[poise::command(slash_command, guild_only)]
pub async fn balance(
    ctx: Context<'_>,
    #[description = "User to check (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    let eco =
        economy::get_user_economy(&ctx.data().database.pool, guild_id, target.id.get() as i64)
            .await?;
    let total = eco.balance + eco.bank;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("💰 {}'s Balance", target.name))
                .thumbnail(target.face())
                .field("👛 Wallet", format!("`${}`", eco.balance), true)
                .field("🏦 Bank", format!("`${}`", eco.bank), true)
                .field("📊 Net Worth", format!("`${}`", total), true)
                .field(
                    "📈 Statistics",
                    format!(
                        "Total Earned: `${}`\nTotal Spent: `${}`",
                        eco.total_earned, eco.total_spent
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.2 Economy"))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// claim your daily reward
#[poise::command(slash_command, guild_only)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
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
            )
            .into());
        }
    }

    let reward: i64 = 500;
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_daily(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        chrono::Utc::now(),
    )
    .await?;

    let new_total = eco.balance + reward;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("✨ Daily Reward Claimed!")
                .description(format!("You claimed your daily reward of **${}**!", reward))
                .field("💰 Earned", format!("`+${}`", reward), true)
                .field("👛 New Wallet", format!("`${}`", new_total), true)
                .field(
                    "⏰ Next Daily",
                    "<t:{}:R>".replace("{}", &(chrono::Utc::now().timestamp() + 86400).to_string()),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Come back every 24h — don't miss a streak!",
                ))
                .color(0x00FF88),
        ),
    )
    .await?;
    Ok(())
}

/// work to earn some cash (30min cooldown)
#[poise::command(slash_command, guild_only)]
pub async fn work(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
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
            )
            .into());
        }
    }

    let jobs = [
        (
            "🔧 Fixed a production bug",
            "Saved the day — client didn't even notice.",
        ),
        (
            "🚚 Delivered packages",
            "Your back hurts but the tips were worth it.",
        ),
        (
            "🍕 Delivered pizzas",
            "Three cold calls and one generous tipper.",
        ),
        (
            "💻 Freelanced a website",
            "Client wanted it \"simple\" — 47 revisions later...",
        ),
        (
            "🔨 Fixed plumbing",
            "Charged extra for the 2am emergency call.",
        ),
        (
            "🎨 Designed a logo",
            "They said the first one was perfect, then changed it.",
        ),
        (
            "📦 Worked at the warehouse",
            "Hit the quota, earned the bonus.",
        ),
        (
            "🛡️ Forged armor for the realm",
            "The guild appreciated your craftsmanship.",
        ),
    ];
    let (job, flavor) = jobs[rand::thread_rng().gen_range(0..jobs.len())];
    let reward = rand::thread_rng().gen_range(50..=200i64);

    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_work(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        chrono::Utc::now(),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("🔨 Work Complete — {}", job))
                .description(flavor)
                .field("💵 Earned", format!("`+${}`", reward), true)
                .field("⏰ Next Shift", "<t:{}:R>".replace("{}", &(chrono::Utc::now().timestamp() + 1800).to_string()), true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Work smarter, gamble harder",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// view all possible jobs for the work command
#[poise::command(slash_command, guild_only)]
pub async fn work_list(ctx: Context<'_>) -> Result<(), Error> {
    let jobs = [
        "🔧 Fixed a production bug",
        "🚚 Delivered packages",
        "🍕 Delivered pizzas",
        "💻 Freelanced a website",
        "🔨 Fixed plumbing",
        "🎨 Designed a logo",
        "📦 Worked at the warehouse",
        "🛡️ Forged armor for the realm",
    ];
    
    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title("📋 Job Catalog")
            .description(jobs.iter().map(|j| format!("• {}", j)).collect::<Vec<_>>().join("\n"))
            .footer(serenity::CreateEmbedFooter::new("Payouts range from $50 to $200"))
            .color(0x00E5FF)
    )).await?;
    Ok(())
}

/// pay another user from your wallet
#[poise::command(slash_command, guild_only)]
pub async fn pay(
    ctx: Context<'_>,
    #[description = "User to pay"] user: serenity::User,
    #[description = "Amount to send"] amount: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    if user.id == ctx.author().id {
        return Err("You can't pay yourself.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;

    let author_eco =
        economy::get_user_economy(&ctx.data().database.pool, guild_id, author_id).await?;
    if author_eco.balance < amount {
        return Err(format!(
            "Insufficient funds. You only have `${}` in your wallet.",
            author_eco.balance
        )
        .into());
    }

    economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -amount).await?;
    economy::update_balance(
        &ctx.data().database.pool,
        guild_id,
        user.id.get() as i64,
        amount,
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("💸 Transfer Successful")
                .field("Recipient", format!("**{}**", user.name), true)
                .field("Amount", format!("`${}` sent", amount), true)
                .field(
                    "Your Balance",
                    format!("`${}`", author_eco.balance - amount),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Wallet-to-wallet transfer",
                ))
                .color(0x00FF88),
        ),
    )
    .await?;
    Ok(())
}

/// view the server's richest members
#[poise::command(slash_command)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Show the global leaderboard instead"] global: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let is_global = global.unwrap_or(false);

    let (title, footer, content) = if is_global {
        let lb = economy::get_global_leaderboard(&ctx.data().database.pool, 10).await?;
        let mut desc = String::new();
        let medals = ["🥇", "🥈", "🥉"];
        for (i, entry) in lb.iter().enumerate() {
            let prefix = medals.get(i).copied().unwrap_or("🏅");
            desc.push_str(&format!(
                "{} <@{}> — `${}`\n",
                prefix, entry.user_id, entry.total_balance
            ));
        }
        if desc.is_empty() {
            desc = "_No data yet — start earning!_".to_string();
        }
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
            desc.push_str(&format!(
                "{} <@{}> — `${}`\n",
                prefix,
                eco.user_id,
                eco.balance + eco.bank
            ));
        }
        if desc.is_empty() {
            desc = "_No data yet — start earning!_".to_string();
        }
        (
            "🏆 Server Wealth Leaderboard",
            "Top 10 wealthiest in this server",
            desc,
        )
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(title)
                .description(content)
                .footer(serenity::CreateEmbedFooter::new(footer))
                .timestamp(serenity::Timestamp::now())
                .color(0xFFD700),
        ),
    )
    .await?;
    Ok(())
}

/// deposit money into your bank
#[poise::command(slash_command, guild_only)]
pub async fn deposit(
    ctx: Context<'_>,
    #[description = "Amount to deposit"] amount: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.balance {
        return Err(format!("You only have `${}` in your wallet.", eco.balance).into());
    }

    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, amount).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🏦 Deposit Successful")
                .field("💰 Deposited", format!("`${}` → Bank", amount), true)
                .field(
                    "👛 Wallet Remaining",
                    format!("`${}`", eco.balance - amount),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Banked funds are safe from robbery",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// withdraw money from your bank
#[poise::command(slash_command, guild_only)]
pub async fn withdraw(
    ctx: Context<'_>,
    #[description = "Amount to withdraw"] amount: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if amount <= 0 {
        return Err("Amount must be positive.".into());
    }
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if amount > eco.bank {
        return Err(format!("You only have `${}` in your bank.", eco.bank).into());
    }

    economy::transfer_to_bank(&ctx.data().database.pool, guild_id, user_id, -amount).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("💰 Withdrawal Successful")
                .field("💵 Withdrawn", format!("`${}` → Wallet", amount), true)
                .field(
                    "🏦 Bank Remaining",
                    format!("`${}`", eco.bank - amount),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Be careful — wallet funds can be robbed",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// beg for spare change
#[poise::command(slash_command, guild_only)]
pub async fn beg(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let givers = [
        "A kind stranger",
        "A passing merchant",
        "An old wizard",
        "A generous guild member",
        "A cloaked figure",
    ];
    let giver = givers[rand::thread_rng().gen_range(0..givers.len())];
    let reward = rand::thread_rng().gen_range(0i64..=50);

    let embed = if reward == 0 {
        CreateEmbed::new()
            .title("🥺 Nobody's Feeling Generous")
            .description("You held out your hand but everyone walked by. Try again later.")
            .color(0xFF5722)
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
        CreateEmbed::new()
            .title("🤲 Begging Results")
            .description(format!(
                "**{}** took pity and tossed you some coins!",
                giver
            ))
            .field("💰 Received", format!("`+${}`", reward), true)
            .color(0x00E5FF)
    };

    ctx.send(
        poise::CreateReply::default().embed(embed.footer(serenity::CreateEmbedFooter::new(
            "Consider working for a living",
        ))),
    )
    .await?;
    Ok(())
}

/// search around for hidden money
#[poise::command(slash_command, guild_only)]
pub async fn search(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
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

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("{} Search Results", emoji))
                .description(format!("You rummaged around **{}**...", location))
                .field("💰 Found", format!("`+${}`", reward), true)
                .footer(serenity::CreateEmbedFooter::new("Finders keepers"))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// spin the slot machine
#[poise::command(slash_command, guild_only)]
pub async fn slots(
    ctx: Context<'_>,
    #[description = "Amount to bet (minimum $10)"] bet: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
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
        )
        .into());
    }

    let emojis = ["🍒", "🍋", "🍇", "🍊", "💎", "⭐", "🔔"];
    let (r1, r2, r3) = {
        let mut rng = rand::thread_rng();
        (
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())],
            emojis[rng.gen_range(0..emojis.len())],
        )
    };

    let (won, multiplier, label) = if r1 == r2 && r2 == r3 {
        match r1 {
            "💎" => (true, 50.0, "💎 ELITE JACKPOT! 💎"),
            "⭐" => (true, 25.0, "⭐ STAR POWER! ⭐"),
            "🔔" => (true, 15.0, "🔔 GOLDEN BELLS! 🔔"),
            _ => (true, 10.0, "✨ TRIPLE MATCH! ✨"),
        }
    } else if r1 == r2 || r2 == r3 || r1 == r3 {
        (true, 3.0, "🍀 Double Match!")
    } else {
        (false, 0.0, "The forge rejects your bet.")
    };

    let slot_display = format!("**[ {} | {} | {} ]**", r1, r2, r3);

    if won {
        let prize = (bet as f64 * multiplier) as i64;
        let profit = prize - bet;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, profit).await?;
        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title(format!("🎰 {}", label))
                    .description(format!("{}\n\nCongratulations! You won big.", slot_display))
                    .field("🎯 Bet", format!("`${}`", bet), true)
                    .field("💰 Payout", format!("`${}`", prize), true)
                    .field("📈 Profit", format!("`+${}`", profit), true)
                    .footer(serenity::CreateEmbedFooter::new(format!(
                        "{}x multiplier | AegisForge Slots",
                        multiplier
                    )))
                    .color(0x00FF88),
            ),
        )
        .await?;
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🎰 Better luck next time!")
                    .description(format!("{}\n\n{}", slot_display, label))
                    .field("💸 Lost", format!("`-${}`", bet), true)
                    .footer(serenity::CreateEmbedFooter::new("AegisForge Slots"))
                    .color(0xFF3B3B),
            ),
        )
        .await?;
    }
    Ok(())
}

/// bet on a dice roll (1-6)
#[poise::command(slash_command, guild_only)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Your guess (1-6)"] guess: u8,
    #[description = "Amount to bet"] bet: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if guess < 1 || guess > 6 { return Err("Guess must be between 1 and 6.".into()); }
    if bet < 10 { return Err("Minimum bet is $10.".into()); }
    
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if bet > eco.balance { return Err("Insufficient funds.".into()); }

    let roll = rand::thread_rng().gen_range(1..=6);
    let win = roll == guess;

    let dice_emojis = ["", "⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    let mut embed = CreateEmbed::new()
        .title("🎲 Dice Roll")
        .description(format!("The dice landed on **{} {}**!", dice_emojis[roll as usize], roll))
        .timestamp(serenity::Timestamp::now());

    if win {
        let reward = bet * 5;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
        embed = embed.field("Result", format!("Correct! You won **${}** (5x multiplier)!", reward), false).color(0x00FF88);
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        embed = embed.field("Result", format!("Wrong guess. You lost **${}**.", bet), false).color(0xFF3B3B);
    }

    ctx.send(poise::CreateReply::default().embed(embed.footer(serenity::CreateEmbedFooter::new("AegisForge v4.2 Economy")))).await?;
    Ok(())
}

/// view slot machine odds and payout table
#[poise::command(slash_command, guild_only)]
pub async fn gamble_info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎰 Hyperforge Slot Machine — Payout Table")
                .description(
                    "The FairForge™ algorithm ensures ~81.5% total win probability per spin.",
                )
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
                .footer(serenity::CreateEmbedFooter::new(
                    "Bank your winnings to protect them from /economy rob",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// view the global wealth leaderboard
#[poise::command(slash_command, guild_only)]
pub async fn global_leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
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

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🌍 Global Wealth Leaderboard")
                .description(description)
                .footer(serenity::CreateEmbedFooter::new(
                    "Across all guilds in the AegisForge network",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFFD700),
        ),
    )
    .await?;
    Ok(())
}

/// rob another user's wallet
#[poise::command(slash_command, guild_only)]
pub async fn rob(
    ctx: Context<'_>,
    #[description = "User to rob"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;
    if user.id == ctx.author().id {
        return Err("You can't rob yourself.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let author_id = ctx.author().id.get() as i64;
    let target_id = user.id.get() as i64;

    let target_eco =
        economy::get_user_economy(&ctx.data().database.pool, guild_id, target_id).await?;
    if target_eco.balance < 100 {
        return Err(format!(
            "**{}** doesn't have enough in their wallet to bother (need at least `$100`).",
            user.name
        )
        .into());
    }

    // drop ThreadRng before any await — ThreadRng is !Send
    let (success, stolen_fraction) = {
        let mut rng = rand::thread_rng();
        (rng.gen_bool(0.4), rng.gen_range(0.1_f64..0.5))
    };

    if success {
        let stolen = (target_eco.balance as f64 * stolen_fraction) as i64;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, stolen).await?;
        economy::update_balance(&ctx.data().database.pool, guild_id, target_id, -stolen).await?;

        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🥷 Heist Successful!")
                    .description(format!(
                        "You slipped into **{}**'s room undetected.",
                        user.name
                    ))
                    .field(
                        "💰 Stolen",
                        format!(
                            "`${}` ({:.0}% of their wallet)",
                            stolen,
                            stolen_fraction * 100.0
                        ),
                        false,
                    )
                    .footer(serenity::CreateEmbedFooter::new(
                        "You got away clean — this time",
                    ))
                    .color(0x00FF88),
            ),
        )
        .await?;
    } else {
        let fine: i64 = 200;
        economy::update_balance(&ctx.data().database.pool, guild_id, author_id, -fine).await?;
        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("👮 Caught Red-Handed!")
                    .description(format!("You were caught trying to rob **{}**!", user.name))
                    .field("💸 Fine Paid", format!("`-${}`", fine), true)
                    .footer(serenity::CreateEmbedFooter::new(
                        "Crime doesn't pay — 60% of the time",
                    ))
                    .color(0xFF3B3B),
            ),
        )
        .await?;
    }
    Ok(())
}

/// commit a crime to earn big (high risk!)
#[poise::command(slash_command, guild_only)]
pub async fn crime(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;

    if let Some(last) = eco.last_crime {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_minutes() < 60 {
            return Err(format!(
                "The heat is still on! Wait **{}m** before your next crime.",
                60 - diff.num_minutes()
            )
            .into());
        }
    }

    let success = rand::random::<f64>() > 0.65;
    if success {
        let reward = rand::thread_rng().gen_range(500..=2000i64);
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
        economy::set_last_crime(
            &ctx.data().database.pool,
            guild_id,
            user_id,
            chrono::Utc::now(),
        )
        .await?;

        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🥷 Crime Successful")
                    .description("You pulled off a high-stakes heist!")
                    .field("💰 Loot", format!("`+${}`", reward), true)
                    .color(0x00FF88),
            ),
        )
        .await?;
    } else {
        let fine = 400;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -fine).await?;
        economy::set_last_crime(
            &ctx.data().database.pool,
            guild_id,
            user_id,
            chrono::Utc::now(),
        )
        .await?;

        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("👮 Busted!")
                    .description("You were caught by the Aegis Sentinels!")
                    .field("💸 Fine", format!("`-${}`", fine), true)
                    .color(0xFF3B3B),
            ),
        )
        .await?;
    }
    Ok(())
}

/// go fishing for some quick cash
#[poise::command(slash_command, guild_only)]
pub async fn fish(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if let Some(last) = eco.last_fish {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_minutes() < 5 {
            return Err(format!(
                "The fish aren't biting yet. Wait **{}m**.",
                5 - diff.num_minutes()
            )
            .into());
        }
    }

    let fish_types = [
        ("🐟 Common Carp", 20, 50),
        ("🐠 Tropical Fish", 60, 120),
        ("🐡 Blowfish", 150, 300),
        ("🦈 Shark", 500, 1200),
        ("✨ Legendary Golden Fish", 2000, 5000),
    ];

    let roll = rand::random::<f64>();
    let (name, min, max) = if roll > 0.99 {
        fish_types[4]
    } else if roll > 0.90 {
        fish_types[3]
    } else if roll > 0.70 {
        fish_types[2]
    } else if roll > 0.40 {
        fish_types[1]
    } else {
        fish_types[0]
    };

    let reward = rand::thread_rng().gen_range(min..=max);
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_fish(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        chrono::Utc::now(),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎣 Fishing Results")
                .description(format!("You cast your line and caught a **{}**!", name))
                .field("💰 Value", format!("`+${}`", reward), true)
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// go hunting in the wild
#[poise::command(slash_command, guild_only)]
pub async fn hunt(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if let Some(last) = eco.last_hunt {
        let diff = chrono::Utc::now().signed_duration_since(last);
        if diff.num_minutes() < 10 {
            return Err(format!(
                "The animals are hiding. Wait **{}m**.",
                10 - diff.num_minutes()
            )
            .into());
        }
    }

    let animals = [
        ("🐰 Rabbit", 30, 80),
        ("🦊 Fox", 100, 250),
        ("🦌 Deer", 300, 600),
        ("🐻 Bear", 800, 1500),
        ("🐉 Dragon (!!!)", 5000, 15000),
    ];

    let roll = rand::random::<f64>();
    let (name, min, max) = if roll > 0.995 {
        animals[4]
    } else if roll > 0.95 {
        animals[3]
    } else if roll > 0.80 {
        animals[2]
    } else if roll > 0.50 {
        animals[1]
    } else {
        animals[0]
    };

    let reward = rand::thread_rng().gen_range(min..=max);
    economy::update_balance(&ctx.data().database.pool, guild_id, user_id, reward).await?;
    economy::set_last_hunt(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        chrono::Utc::now(),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🏹 Hunt Results")
                .description(format!(
                    "You ventured into the woods and took down a **{}**!",
                    name
                ))
                .field("💰 Value", format!("`+${}`", reward), true)
                .color(0xFF5722),
        ),
    )
    .await?;
    Ok(())
}

/// play a game of blackjack against the house
#[poise::command(slash_command, guild_only)]
pub async fn blackjack(
    ctx: Context<'_>,
    #[description = "Amount to bet (minimum $50)"] bet: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if bet < 50 {
        return Err("Minimum bet is **$50**.".into());
    }

    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if bet > eco.balance {
        return Err(format!("Insufficient funds. You only have `${}`.", eco.balance).into());
    }

    // Simplified Blackjack
    let (p_score, d_score) = {
        let mut rng = rand::thread_rng();
        let p1 = rng.gen_range(2..=11);
        let p2 = rng.gen_range(2..=11);
        let d1 = rng.gen_range(2..=11);
        let d2 = rng.gen_range(2..=11);
        (p1 + p2, d1 + d2)
    };

    let mut embed = CreateEmbed::new()
        .title("🃏 AegisForge Blackjack")
        .field("Your Score", format!("`{}`", p_score), true)
        .field("Dealer Score", format!("`{}`", d_score), true)
        .timestamp(serenity::Timestamp::now());

    if p_score > 21 {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        embed = embed
            .description(format!("Bust! You went over 21 and lost **${}**.", bet))
            .color(0xFF3B3B);
    } else if d_score > 21 || p_score > d_score {
        let profit = bet;
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, profit).await?;
        embed = embed
            .description(format!("Congratulations! You won **${}**!", bet))
            .color(0x00FF88);
    } else if p_score == d_score {
        embed = embed
            .description("It's a push! You tied with the dealer.")
            .color(0xFEE75C);
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        embed = embed
            .description(format!("The dealer wins. You lost **${}**.", bet))
            .color(0xFF3B3B);
    }

    ctx.send(poise::CreateReply::default().embed(embed.footer(serenity::CreateEmbedFooter::new("AegisForge v4.2 Economy")))).await?;
    Ok(())
}

/// flip a coin to double or lose your bet
#[poise::command(slash_command, guild_only)]
pub async fn coinflip(
    ctx: Context<'_>,
    #[description = "Heads or Tails"] choice: String,
    #[description = "Amount to bet"] bet: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    if bet <= 0 { return Err("Bet must be positive.".into()); }
    
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    let eco = economy::get_user_economy(&ctx.data().database.pool, guild_id, user_id).await?;
    if bet > eco.balance { return Err("Insufficient funds.".into()); }

    let win = rand::random::<bool>();
    let side = if win { choice.clone() } else { if choice.to_lowercase() == "heads" { "Tails".into() } else { "Heads".into() } };

    let mut embed = CreateEmbed::new()
        .title("🪙 Coin Flip")
        .description(format!("The coin landed on **{}**!", side))
        .timestamp(serenity::Timestamp::now());

    if win {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, bet).await?;
        embed = embed.field("Result", format!("You won **${}**!", bet), false).color(0x00FF88);
    } else {
        economy::update_balance(&ctx.data().database.pool, guild_id, user_id, -bet).await?;
        embed = embed.field("Result", format!("You lost **${}**.", bet), false).color(0xFF3B3B);
    }

    ctx.send(poise::CreateReply::default().embed(embed.footer(serenity::CreateEmbedFooter::new("AegisForge v4.2 Economy")))).await?;
    Ok(())
}
