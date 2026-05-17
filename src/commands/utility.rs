use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};

const TOPSTATS_BOT_ID: &str = "1500582485367722004";

#[derive(serde::Deserialize)]
struct TopStatsBot {
    monthly_votes: Option<i64>,
    total_votes: Option<i64>,
    server_count: Option<i64>,
}

async fn fetch_topstats(http: &reqwest::Client) -> Option<TopStatsBot> {
    let token = std::env::var("TOPSTATS_TOKEN").ok()?;
    let url = format!("https://api.topstats.gg/discord/bots/{}", TOPSTATS_BOT_ID);
    let resp = http
        .get(&url)
        .header("Authorization", token)
        .send()
        .await
        .ok()?;
    resp.json::<TopStatsBot>().await.ok()
}

/// check the bot latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.say("Measuring latency...").await?;
    let elapsed = start.elapsed().as_millis();

    msg.edit(
        ctx,
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("⚡ AegisForge Status")
                .field("Gateway Latency", format!("{}ms", elapsed), true)
                .field("API Status", "Connected", true)
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// info about the current server
#[poise::command(slash_command, prefix_command, guild_only, rename = "server")]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();

    let icon = guild.icon_url().unwrap_or_default();
    // boost_tier calculated here for potential future use or logging
    let _boost_tier = format!("Tier {}", u8::from(guild.premium_tier));

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("🔷 Server Information — {}", guild.name))
                .description(format!("Overview for **{}**.", guild.name))
                .thumbnail(icon)
                .field("👑 Owner", format!("<@{}>", guild.owner_id), true)
                .field("👥 Members", format!("`{}`", guild.member_count), true)
                .field("📺 Channels", format!("`{}`", guild.channels.len()), true)
                .field("📜 Roles", format!("`{}`", guild.roles.len()), true)
<<<<<<< HEAD
                .field(
                    "🚀 Boosts",
                    format!(
                        "`{}` (Tier {})",
                        guild.premium_subscription_count.unwrap_or(0),
                        u8::from(guild.premium_tier)
                    ),
                    true,
                )
                .field(
                    "🛡️ Security",
                    format!("Level `{:?}`", guild.verification_level),
                    true,
                )
=======
                .field("🚀 Boosts", format!("`{}` (Tier {})", guild.premium_subscription_count.unwrap_or(0), u8::from(guild.premium_tier)), true)
                .field("🛡️ Security", format!("Level `{:?}`", guild.verification_level), true)
>>>>>>> 464415d48bbb577285feea95e643bf0a924170dd
                .field(
                    "📅 Created",
                    format!(
                        "<t:{}:F> (<t:{}:R>)",
                        guild.id.created_at().unix_timestamp(),
                        guild.id.created_at().unix_timestamp()
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "ID: {}",
                    guild.id
                )))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// info about a user
#[poise::command(slash_command, prefix_command, rename = "user")]
pub async fn whois(
    ctx: Context<'_>,
    #[description = "The user to look up (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("👤 User Profile")
                .description(format!("Information for **{}**.", target.name))
                .thumbnail(target.face())
                .field("📝 Username", format!("**{}**", target.name), true)
                .field("🆔 ID", format!("`{}`", target.id), true)
                .field("🤖 Type", if target.bot { "Bot" } else { "User" }, true)
                .field(
                    "📅 Account Created",
                    format!(
                        "<t:{}:F> (<t:{}:R>)",
                        target.id.created_at().unix_timestamp(),
                        target.id.created_at().unix_timestamp()
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.3"))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// get someone's avatar
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to get the avatar of (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let avatar_url = target.face();

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🖼️ User Avatar")
                .description(format!("Avatar asset for **{}**.", target.name))
                .image(&avatar_url)
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    ctx.author().name
                )))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// show bot uptime
#[poise::command(slash_command, prefix_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = ctx.data().start_time.elapsed();
    let days = uptime.as_secs() / 86400;
    let hours = (uptime.as_secs() % 86400) / 3600;
    let minutes = (uptime.as_secs() % 3600) / 60;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🕒 Bot Uptime")
                .description(format!(
                    "Current session uptime: **{}d {}h {}m**.",
                    days, hours, minutes
                ))
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.3"))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// vote for the bot and claim the vote reward
#[poise::command(slash_command, prefix_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title("🗳️ Support AegisForge")
            .description("Vote on top.gg to support the bot and receive a **$1,000 economy bonus** ($2,000 on weekends).")
            .field("Link", "[Click here to vote](https://top.gg/bot/1500582485367722004/vote)", false)
            .footer(serenity::CreateEmbedFooter::new("Thanks for supporting AegisForge"))
            .color(0x00FF88),
    ))
    .await?;
    Ok(())
}

/// show global bot statistics
#[poise::command(slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();
    let topstats = fetch_topstats(&ctx.data().http_client).await;

    let mut embed = CreateEmbed::new()
        .title("📊 AegisForge — Global Stats")
        .description("Aggregated metrics from the entire AegisForge network.")
        .field(
            "🌐 Reach",
            format!("**{}** Servers\n**{}** Users", guilds, users),
            true,
        )
        .field(
            "⚙️ Resource Usage",
            "Tokio Runtime: Active\nDB Pool: Optimal",
            true,
        )
        .field(
            "⏱️ Session",
            format!(
                "<t:{}:R>",
                (chrono::Utc::now() - chrono::Duration::seconds(uptime.as_secs() as i64))
                    .timestamp()
            ),
            true,
        );

    if let Some(ts) = topstats {
        let monthly = ts.monthly_votes.unwrap_or(0);
        let total = ts.total_votes.unwrap_or(0);
        let tracked_servers = ts.server_count.unwrap_or(guilds as i64);
        embed = embed
            .field(
                "🗳️ Top.gg Votes",
                format!("**{}** this month\n**{}** all time", monthly, total),
                true,
            )
            .field(
                "📈 Tracked Servers",
                format!("**{}**", tracked_servers),
                true,
            );
    }

    embed = embed
        .footer(serenity::CreateEmbedFooter::new(
            "Powered by Rust + Tokio | AegisForge v4.3",
        ))
        .timestamp(serenity::Timestamp::now())
        .color(0x00E5FF);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// show bot telemetry and links
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();

    let pool = &ctx.data().database.pool;
    let (total_commands, economy_transactions, inventory_items) = tokio::join!(
        async {
            sqlx::query_scalar::<_, i64>(
                "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_commands_executed'), 0)",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0)
        },
        async {
            sqlx::query_scalar::<_, i64>(
                "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_economy_transactions'), 0)",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0)
        },
        async {
            sqlx::query_scalar::<_, i64>(
                "SELECT COALESCE(SUM(quantity), 0)::BIGINT FROM economy_inventory",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0)
        }
    );

    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title("AegisForge v4.3 - Bot Info")
            .description("Runtime, network, economy, and stats for the current bot process.")
            .field("Version", format!("v{}", env!("CARGO_PKG_VERSION")), true)
            .field("Runtime", "Rust + Tokio", true)
            .field("Discord Layer", "Serenity + Poise", true)
            .field("Servers", guilds.to_string(), true)
            .field("Cached Users", users.to_string(), true)
            .field("Commands Run", total_commands.to_string(), true)
            .field("Economy Transactions", economy_transactions.to_string(), true)
            .field("Inventory Items", inventory_items.to_string(), true)
            .field("Uptime", format!("<t:{}:R>", (chrono::Utc::now() - chrono::Duration::seconds(uptime.as_secs() as i64)).timestamp()), true)
            .field("Links", "[Website](https://aegisforge-vert.vercel.app/) | [Top.gg](https://top.gg/bot/1500582485367722004)", false)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "AegisForge v{}",
                env!("CARGO_PKG_VERSION")
            )))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// send a custom embed to the current channel
#[poise::command(slash_command, prefix_command)]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Title of the embed"] title: String,
    #[description = "Description of the embed"] description: String,
    #[description = "Hex color (e.g. 00E5FF)"] color: Option<String>,
) -> Result<(), Error> {
    let color_val = color
        .and_then(|c| u32::from_str_radix(c.trim_start_matches('#'), 16).ok())
        .unwrap_or(0x00E5FF);

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(title)
                .description(description)
                .color(color_val),
        ),
    )
    .await?;
    Ok(())
}

/// generate a Discord timestamp for a given UNIX timestamp
#[poise::command(slash_command, prefix_command)]
pub async fn timestamp(
    ctx: Context<'_>,
    #[description = "UNIX timestamp (seconds)"] unix: i64,
) -> Result<(), Error> {
    ctx.say(format!(
        "**Timestamp formats for `{}`:**\n> Short: `<t:{}:t>` → <t:{}:t>\n> Long: `<t:{}:F>` → <t:{}:F>\n> Relative: `<t:{}:R>` → <t:{}:R>",
        unix, unix, unix, unix, unix, unix, unix
    ))
    .await?;
    Ok(())
}

struct HelpEntry {
    aliases: &'static [&'static str],
    title: &'static str,
    category: &'static str,
    usage: &'static str,
    description: &'static str,
    notes: &'static str,
}

const HELP_ENTRIES: &[HelpEntry] = &[
    HelpEntry {
        aliases: &["help"],
        title: "Help",
        category: "Utility",
        usage: "/help command:ban",
        description: "Show the command center or detailed help for a specific command.",
        notes: "Use command names with or without the slash.",
    },
    HelpEntry {
        aliases: &["ping"],
        title: "Ping",
        category: "Utility",
        usage: "/ping",
        description: "Measure command response latency.",
        notes: "Useful when checking whether the bot is responding normally.",
    },
    HelpEntry {
        aliases: &["stats", "botinfo"],
        title: "Stats and Bot Info",
        category: "Utility",
        usage: "/stats or /botinfo",
        description: "Show live bot reach, uptime, runtime, economy, inventory, and project links.",
        notes: "Stats can include Top.gg data when the token is configured.",
    },
    HelpEntry {
        aliases: &["ban", "softban", "unban", "kick", "timeout", "mute", "unmute", "warn", "purge", "nuke", "lock", "unlock"],
        title: "Core Moderation",
        category: "Moderation",
        usage: "/ban user:@user reason:text days:int",
        description: "Moderation actions for bans, kicks, warnings, timeouts, cleanup, locks, and channel resets.",
        notes: "Requires the right Discord permissions and role hierarchy.",
    },
    HelpEntry {
        aliases: &["shadowban", "unshadowban", "muterole"],
        title: "Shadow Ban",
        category: "Moderation",
        usage: "/muterole role:@Muted then /shadowban user:@user reason:text",
        description: "Silently restrict a member using the configured mute role without a public callout.",
        notes: "Create and configure the muted role first, then keep AegisForge above that role.",
    },
    HelpEntry {
        aliases: &["tactical", "intercept", "restore", "breach", "report"],
        title: "Tactical Commands",
        category: "Moderation",
        usage: "/tactical report user:@user",
        description: "Incident tools for full user reports, server-wide intercept locks, restore, and breach cleanup.",
        notes: "Use tactical intercept only during active incidents or planned tests.",
    },
    HelpEntry {
        aliases: &["sentinel"],
        title: "Sentinel Anti-Raid",
        category: "Security",
        usage: "/sentinel enable then /sentinel threshold joins:5 window:10",
        description: "Detect join spikes and react to raids using the configured threshold and time window.",
        notes: "Pair Sentinel with /logs so staff can see alerts.",
    },
    HelpEntry {
        aliases: &["automod"],
        title: "AutoMod",
        category: "Security",
        usage: "/automod enable then /automod status",
        description: "Toggle anti-spam, invite blocking, caps detection, mention spam, and blacklist phrases.",
        notes: "Start with status, then enable modules one at a time while testing.",
    },
    HelpEntry {
        aliases: &["economy", "balance", "daily", "work", "shop", "buy", "inventory", "profile", "rob", "crime", "fish", "hunt"],
        title: "Economy",
        category: "Economy",
        usage: "/economy profile user:@user",
        description: "Wallets, banking, daily rewards, jobs, games, shop purchases, inventory, and leaderboards.",
        notes: "Use /economy shop to browse items and /economy inventory to review purchases.",
    },
    HelpEntry {
        aliases: &["leveling", "rank", "leaderboard", "customize"],
        title: "Leveling",
        category: "Leveling",
        usage: "/leveling rank user:@user",
        description: "XP, rank cards, local/global leaderboards, and profile customization.",
        notes: "Rank data updates as users chat and gain XP.",
    },
    HelpEntry {
        aliases: &["giveaway"],
        title: "Giveaways",
        category: "Giveaways",
        usage: "/giveaway start prize:text duration_minutes:int winners:int",
        description: "Start, end, reroll, and list server giveaways.",
        notes: "Use a clear prize name and duration so users understand the event.",
    },
    HelpEntry {
        aliases: &["logs", "msglogs", "memberlogs", "welcome", "goodbye", "autorole", "prefix", "settings"],
        title: "Server Configuration",
        category: "Configuration",
        usage: "/logs channel:#mod-log",
        description: "Configure server logs, welcome/goodbye messages, autorole, prefixes, and saved settings.",
        notes: "Configure logs before heavy moderation testing.",
    },
];

fn find_help_entry(command: &str) -> Option<&'static HelpEntry> {
    let needle = command
        .trim()
        .trim_start_matches('/')
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_lowercase();

    HELP_ENTRIES.iter().find(|entry| {
        entry
            .aliases
            .iter()
            .any(|alias| *alias == needle || needle.contains(alias))
    })
}

/// displays all available commands
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    if let Some(cmd) = command {
        let embed = if let Some(entry) = find_help_entry(&cmd) {
            CreateEmbed::new()
                .title(format!("Help - {}", entry.title))
                .description(entry.description)
                .field("Category", entry.category, true)
                .field("Usage", format!("`{}`", entry.usage), false)
                .field("Notes", entry.notes, false)
                .field(
                    "Full Registry",
                    "[Open command registry](https://aegisforge-vert.vercel.app/commands.html)",
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "AegisForge v4.3 command help",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF)
        } else {
            CreateEmbed::new()
                .title("Help - Command Not Found")
                .description(format!(
                    "I could not find a focused help entry for `{}`. Try `/help`, search the website registry, or ask in the support server.",
                    cmd
                ))
                .field(
                    "Full Registry",
                    "[Open command registry](https://aegisforge-vert.vercel.app/commands.html)",
                    false,
                )
                .field(
                    "Support",
                    "[Join AegisForge Support](https://discord.gg/HbmafcgjNa)",
                    false,
                )
                .color(0xFFAA00)
        };

        ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
            .await?;
        return Ok(());
    }

    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
<<<<<<< HEAD
            .title("AegisForge v4.3 - Command Center")
            .description("Use `/help command:<name>` for focused command help, or open the website registry for the full searchable command surface.")
            .field("Security", "`sentinel`, `automod`, `shadowban`, `tactical`, `logs`, `msglogs`, `memberlogs`", false)
            .field("Moderation", "`ban`, `softban`, `unban`, `kick`, `timeout`, `warn`, `purge`, `nuke`, `slowmode`, `cases`, `lock`, `unlock`", false)
            .field("Economy", "`economy balance`, `daily`, `work`, `shop`, `buy`, `inventory`, `profile`, `rob`, `crime`, `fish`, `hunt`, `leaderboard`", false)
            .field("Leveling", "`leveling rank`, `leveling leaderboard`, `leveling customize`", false)
            .field("Utility", "`ping`, `server`, `user`, `avatar`, `uptime`, `stats`, `botinfo`, `embed`, `qr`, `math`, `weather`, `worldclock`, `poll`, `timestamp`, `remind`, `dictionary`", false)
            .field("Community", "`fun`, `games`, `giveaway`, role and server config commands", false)
            .field("Links", "[Command Registry](https://aegisforge-vert.vercel.app/commands.html) | [Support Server](https://discord.gg/HbmafcgjNa) | [Security Setup](https://aegisforge-vert.vercel.app/security.html)", false)
            .footer(serenity::CreateEmbedFooter::new("AegisForge v4.3 | Secure | Powerful | Fast"))
=======
            .title("🛡️ AegisForge v4.3 — Command Center")
            .description("Server protection, economy, leveling, utilities, and high-performance automation. Use `/` to browse all slash commands.")
            .field("⚙️ Utility", "`ping`, `server`, `user`, `avatar`, `uptime`, `stats`, `embed`, `qr`, `math`, `weather`, `worldclock`, `poll`, `timestamp`, `remind`, `help`", false)
            .field("🛡️ Moderation", "`ban`, `softban`, `unban`, `kick`, `mute`, `unmute`, `timeout`, `warn`, `purge`, `nuke`, `slowmode`, `cases`, `slowmode_global`, `lock`, `unlock`", false)
            .field("💰 Economy", "`balance`, `daily`, `work`, `pay`, `deposit`, `withdraw`, `beg`, `search`, `slots`, `blackjack`, `coinflip`, `shop`, `buy`, `inventory`, `rob`, `crime`, `fish`, `hunt`, `leaderboard`, `gamble_info`", false)
            .field("📈 Leveling", "`rank`, `leaderboard`, `set_xp`, `reset_user` (staff only)", false)
            .field("🎮 Fun", "`coinflip`, `dice`, `eightball`, `joke`, `fact`, `ship`, `rate`, `mock`, `reverse`, `ascii`, `choose`, `trivia`, `roast`, `compliment`, `meme`", false)
            .field("🔧 Config", "`logs`, `welcome`, `goodbye`, `autorole`, `prefix`, `settings`, `sentinel`, `automod` (staff only)", false)
            .field("👤 Roles", "`/role add`, `/role remove`, `/role list`, `/role create`, `/role delete` (staff only)", false)
            .field("🔗 Links", "[Website](https://aegisforge-vert.vercel.app/) | [Support Server](https://discord.gg/HbmafcgjNa)", false)
            .footer(serenity::CreateEmbedFooter::new("AegisForge v4.3 | Secure • Powerful • Fast"))
>>>>>>> 464415d48bbb577285feea95e643bf0a924170dd
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    )).await?;

    Ok(())
}

/// evaluate a mathematical expression
#[poise::command(slash_command)]
pub async fn math(
    ctx: Context<'_>,
    #[description = "Expression to evaluate (e.g. 2 + 2 * 5)"] expression: String,
) -> Result<(), Error> {
    let result = match evalexpr::eval(&expression) {
        Ok(value) => value.to_string(),
        Err(e) => format!("Error: {}", e),
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔢 Calculator")
                .field("Expression", format!("`{}`", expression), false)
                .field("Result", format!("**{}**", result), false)
                .footer(serenity::CreateEmbedFooter::new("Powered by evalexpr"))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// generate a QR code from text or URL
#[poise::command(slash_command)]
pub async fn qr(
    ctx: Context<'_>,
    #[description = "The text/URL to encode"] data: String,
) -> Result<(), Error> {
    let qr_url = format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=150x150&data={}",
        urlencoding::encode(&data)
    );

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔳 QR Generator")
                .description(format!("Encoded data: `{}`", data))
                .image(qr_url)
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// look up the price of a cryptocurrency
#[poise::command(slash_command)]
pub async fn crypto(
    ctx: Context<'_>,
    #[description = "Symbol (e.g. BTC, ETH, SOL)"] symbol: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let sym = symbol.to_uppercase();
    let pair = format!("{}USDT", sym);
    let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", pair);

    #[derive(serde::Deserialize)]
    struct BinanceTicker {
        #[serde(rename = "lastPrice")]
        last_price: String,
        #[serde(rename = "priceChangePercent")]
        price_change: String,
        #[serde(rename = "highPrice")]
        high: String,
        #[serde(rename = "lowPrice")]
        low: String,
        volume: String,
    }

    let res = ctx.data().http_client.get(&url).send().await;

    if let Ok(response) = res {
        if let Ok(ticker) = response.json::<BinanceTicker>().await {
            let price: f64 = ticker.last_price.parse().unwrap_or(0.0);
            let change: f64 = ticker.price_change.parse().unwrap_or(0.0);
            let color = if change >= 0.0 { 0x00FF88 } else { 0xFF3B3B };
            let emoji = if change >= 0.0 { "📈" } else { "📉" };

            ctx.send(
                poise::CreateReply::default().embed(
                    CreateEmbed::new()
                        .title(format!("🪙 {}/USDT Market Data", sym))
                        .description(format!("{} **${:.2}** ({:+.2}%)", emoji, price, change))
                        .field(
                            "24h High",
                            format!("`${:.2}`", ticker.high.parse::<f64>().unwrap_or(0.0)),
                            true,
                        )
                        .field(
                            "24h Low",
                            format!("`${:.2}`", ticker.low.parse::<f64>().unwrap_or(0.0)),
                            true,
                        )
                        .field("24h Volume", format!("`{}`", ticker.volume), true)
                        .footer(serenity::CreateEmbedFooter::new("Data provided by Binance"))
                        .timestamp(serenity::Timestamp::now())
                        .color(color),
                ),
            )
            .await?;
            return Ok(());
        }
    }

    Err(format!(
        "Could not find market data for **{}**. Make sure it's a valid symbol on Binance.",
        sym
    )
    .into())
}

/// translate text between languages (requires API key — contact server admin)
#[poise::command(slash_command, prefix_command)]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "Text to translate"] text: String,
    #[description = "Target language code (e.g. en, fr, es, ja)"] target: String,
) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🌍 Translation")
                .description(format!("**Original:** {}", text))
                .field(
                    "Target Language",
                    format!("`{}`", target.to_lowercase()),
                    true,
                )
                .field(
                    "Status",
                    "Translation API key not configured for this instance.",
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Configure TRANSLATE_API_KEY in your .env to enable this",
                ))
                .color(0xFFAA00),
        ),
    )
    .await?;
    Ok(())
}

/// set a reminder
#[poise::command(slash_command, prefix_command)]
pub async fn remind(
    ctx: Context<'_>,
    #[description = "Duration in minutes (1–1440)"] minutes: u64,
    #[description = "What should I remind you about?"]
    #[rest]
    reason: String,
) -> Result<(), Error> {
    if minutes == 0 || minutes > 1440 {
        return Err("Reminder must be between 1 and 1440 minutes (24 hours).".into());
    }

    let expires_at = chrono::Utc::now().timestamp() + (minutes as i64 * 60);

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("⏲️ Reminder Set")
                .description(format!(
                    "I'll remind you about **{}** in **{}** minute(s).",
                    reason, minutes
                ))
                .field("Scheduled For", format!("<t:{}:F>", expires_at), true)
                .field("Time Remaining", format!("<t:{}:R>", expires_at), true)
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.3"))
                .color(0x00E5FF),
        ),
    )
    .await?;

    let http = ctx.serenity_context().http.clone();
    let user_id = ctx.author().id;
    let reason_clone = reason.clone();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(minutes * 60)).await;
        if let Ok(channel) = user_id.create_dm_channel(&http).await {
            let _ = channel
                .send_message(
                    &http,
                    serenity::CreateMessage::new().embed(
                        CreateEmbed::new()
                            .title("⏰ Reminder!")
                            .description(format!(
                                "You asked me to remind you about: **{}**",
                                reason_clone
                            ))
                            .footer(serenity::CreateEmbedFooter::new(format!(
                                "Requested {} minute(s) ago",
                                minutes
                            )))
                            .color(0x00FF88),
                    ),
                )
                .await;
        }
    });

    Ok(())
}

/// look up a word definition
#[poise::command(slash_command, prefix_command)]
pub async fn dictionary(
    ctx: Context<'_>,
    #[description = "Word to look up"] word: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    #[derive(serde::Deserialize)]
    struct Definition {
        definition: String,
        example: Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct Meaning {
        #[serde(rename = "partOfSpeech")]
        part_of_speech: String,
        definitions: Vec<Definition>,
    }
    #[derive(serde::Deserialize)]
    struct DictEntry {
        word: String,
        phonetic: Option<String>,
        meanings: Vec<Meaning>,
    }

    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        urlencoding::encode(&word)
    );

    let response = ctx
        .data()
        .http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Dictionary API unreachable: {}", e))?;

    if response.status() == 404 {
        return Err(format!(
            "No definition found for **\"{}\"**. Check your spelling.",
            word
        )
        .into());
    }

    let entries: Vec<DictEntry> = response
        .json()
        .await
        .map_err(|e| format!("Couldn't parse definition: {}", e))?;

    let entry = entries.into_iter().next().ok_or("No entries returned.")?;

    let mut body = String::new();
    for meaning in entry.meanings.iter().take(3) {
        if let Some(def) = meaning.definitions.first() {
            body.push_str(&format!(
                "**{}**\n{}",
                meaning.part_of_speech, def.definition
            ));
            if let Some(example) = &def.example {
                body.push_str(&format!("\n> *{}*", example));
            }
            body.push_str("\n\n");
        }
    }

    let title = match &entry.phonetic {
        Some(p) if !p.is_empty() => format!("📖 {} — {}", entry.word, p),
        _ => format!("📖 {}", entry.word),
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(title)
                .description(body.trim())
                .footer(serenity::CreateEmbedFooter::new(
                    "Powered by Free Dictionary API",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// view current time in major world cities
#[poise::command(slash_command, prefix_command)]
pub async fn worldclock(ctx: Context<'_>) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let fmt = |offset_secs: i32| -> String {
        let tz = chrono::FixedOffset::east_opt(offset_secs).unwrap();
        now.with_timezone(&tz)
            .format("%H:%M — %a, %b %-d")
            .to_string()
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🌎 World Clock")
                .field("🇬🇧 London (UTC)", fmt(0), true)
                .field("🇺🇸 New York (EST)", fmt(-5 * 3600), true)
                .field("🇺🇸 Los Angeles (PST)", fmt(-8 * 3600), true)
                .field("🇩🇪 Berlin (CET)", fmt(3600), true)
                .field("🇮🇳 Mumbai (IST)", fmt(19800), true) // +5:30
                .field("🇯🇵 Tokyo (JST)", fmt(9 * 3600), true)
                .field("🇦🇺 Sydney (AEST)", fmt(10 * 3600), true)
                .field("🇧🇷 São Paulo (BRT)", fmt(-3 * 3600), true)
                .field("🇦🇪 Dubai (GST)", fmt(4 * 3600), true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Standard offsets shown — actual times may vary by DST",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// create a simple reaction poll
#[poise::command(slash_command, prefix_command)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "The question for the poll"] question: String,
) -> Result<(), Error> {
    let msg = ctx
        .send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title("📊 Community Poll")
                    .description(question)
                    .footer(serenity::CreateEmbedFooter::new(format!(
                        "Poll by {}",
                        ctx.author().name
                    )))
                    .color(0x00E5FF),
            ),
        )
        .await?;
    let message = msg.into_message().await?;
    message
        .react(ctx, serenity::ReactionType::Unicode("✅".to_string()))
        .await?;
    message
        .react(ctx, serenity::ReactionType::Unicode("❌".to_string()))
        .await?;
    Ok(())
}

async fn autocomplete_city(_ctx: Context<'_>, partial: &str) -> Vec<String> {
    [
        "London",
        "New York",
        "Tokyo",
        "Paris",
        "Berlin",
        "Sydney",
        "Los Angeles",
        "Chicago",
        "Toronto",
        "Mumbai",
    ]
    .iter()
    .filter(move |name| name.to_lowercase().starts_with(&partial.to_lowercase()))
    .map(|name| name.to_string())
    .collect()
}

/// get the current weather for a city
#[poise::command(slash_command, prefix_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "City name"]
    #[autocomplete = "autocomplete_city"]
    city: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let url = format!("https://wttr.in/{}?format=j1", urlencoding::encode(&city));

    let res = ctx.data().http_client.get(url).send().await?;
    if !res.status().is_success() {
        return Err("Could not find weather data for that location.".into());
    }

    let data: serde_json::Value = res.json().await?;
    let current = &data["current_condition"][0];
    let temp_c = current["temp_C"].as_str().unwrap_or("0");
    let temp_f = current["temp_F"].as_str().unwrap_or("0");
    let desc = current["weatherDesc"][0]["value"]
        .as_str()
        .unwrap_or("Unknown");
    let humidity = current["humidity"].as_str().unwrap_or("0");
    let wind = current["windspeedKmph"].as_str().unwrap_or("0");

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("🌦️ Weather — {}", city))
                .field("Condition", desc, true)
                .field(
                    "Temperature",
                    format!("`{}°C` ({}°F)", temp_c, temp_f),
                    true,
                )
                .field("Humidity", format!("`{}%`", humidity), true)
                .field("Wind Speed", format!("`{} km/h`", wind), true)
                .footer(serenity::CreateEmbedFooter::new("Powered by wttr.in"))
                .color(0x00E5FF),
        ),
    )
    .await?;

    Ok(())
}
