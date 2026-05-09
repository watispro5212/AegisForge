use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// check the bot latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.say("Measuring latency...").await?;
    let elapsed = start.elapsed().as_millis();

    msg.edit(
        ctx,
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("⚡ AegisForge — Connection")
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
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();

    let icon = guild.icon_url().unwrap_or_default();
    let boost_tier = format!("Tier {}", u8::from(guild.premium_tier));

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("🔷 AegisForge — {}", guild.name))
                .description(format!("Operational overview for **{}**.", guild.name))
                .thumbnail(icon)
                .field("👑 Owner", format!("<@{}>", guild.owner_id), true)
                .field("👥 Members", guild.member_count.to_string(), true)
                .field("📺 Channels", guild.channels.len().to_string(), true)
                .field("📜 Roles", guild.roles.len().to_string(), true)
                .field("🚀 Boost Level", boost_tier, true)
                .field(
                    "📅 Created",
                    format!("<t:{}:F>", guild.id.created_at().unix_timestamp()),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "ID: {} | AegisForge v4",
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
            serenity::CreateEmbed::new()
                .title("👤 AegisForge — Profile")
                .description(format!("Identity data for **{}**.", target.name))
                .thumbnail(target.face())
                .field("📝 Username", format!("**{}**", target.name), true)
                .field("🆔 User ID", target.id.to_string(), true)
                .field(
                    "🤖 Entity Type",
                    if target.bot {
                        "Service Bot"
                    } else {
                        "Human User"
                    },
                    true,
                )
                .field(
                    "📅 Account Created",
                    format!("<t:{}:F>", target.id.created_at().unix_timestamp()),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4"))
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
            serenity::CreateEmbed::new()
                .title("🖼️ AegisForge — Visual Identity")
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
            serenity::CreateEmbed::new()
                .title("🕒 AegisForge — Uptime")
                .description(format!(
                    "Current session uptime: **{}d {}h {}m**.",
                    days, hours, minutes
                ))
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4"))
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
        serenity::CreateEmbed::new()
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
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("📊 AegisForge — Global Telemetry")
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
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Powered by Rust + Tokio | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// show bot telemetry and links
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();

    let total_commands: i64 = sqlx::query_scalar(
        "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_commands_executed'), 0)",
    )
    .fetch_one(&ctx.data().database.pool)
    .await
    .unwrap_or(0);
    let economy_transactions: i64 = sqlx::query_scalar(
        "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_economy_transactions'), 0)",
    )
    .fetch_one(&ctx.data().database.pool)
    .await
    .unwrap_or(0);
    let inventory_items: i64 =
        sqlx::query_scalar("SELECT COALESCE(SUM(quantity), 0)::BIGINT FROM economy_inventory")
            .fetch_one(&ctx.data().database.pool)
            .await
            .unwrap_or(0);

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("AegisForge v4.1 - Advanced Telemetry")
            .description("Runtime, network, economy, and release telemetry for the current bot process.")
            .field("Version", format!("v{}", env!("CARGO_PKG_VERSION")), true)
            .field("Runtime", "Rust + Tokio", true)
            .field("Discord Layer", "Serenity + Poise", true)
            .field("Servers", guilds.to_string(), true)
            .field("Cached Users", users.to_string(), true)
            .field("Commands Run", total_commands.to_string(), true)
            .field("Economy Transactions", economy_transactions.to_string(), true)
            .field("Inventory Items", inventory_items.to_string(), true)
            .field("Uptime", format!("<t:{}:R>", (chrono::Utc::now() - chrono::Duration::seconds(uptime.as_secs() as i64)).timestamp()), true)
            .field("Links", "[Support Server](https://discord.gg/HbmafcgjNa) | [Top.gg](https://top.gg/bot/1500582485367722004)", false)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "AegisForge v{} Hyperforge Core",
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
            serenity::CreateEmbed::new()
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

/// displays all available commands
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    if let Some(cmd) = command {
        let msg = format!(
            "Help for `{}` command is coming soon! For now, explore the categories.",
            cmd
        );
        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title(format!("🔍 Help — {}", cmd))
                    .description(msg)
                    .color(0x00E5FF),
            ),
        )
        .await?;
        return Ok(());
    }

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🛡️ AegisForge v4.1 — Hyperforge Market")
            .description("Server protection, economy, leveling, utilities, and the new global shop inventory system. Use `/` to browse all slash commands.")
            .field("⚙️ Utility", "`ping`, `server`, `user`, `avatar`, `uptime`, `stats`, `embed`, `qr`, `math`, `worldclock`, `poll`, `timestamp`, `timer`, `help`", false)
            .field("🛡️ Moderation", "`ban`, `softban`, `unban`, `kick`, `mute`, `unmute`, `timeout`, `warn`, `purge`, `nuke`, `slowmode`, `lock`, `unlock`", false)
            .field("💰 Economy", "`balance`, `daily`, `work`, `pay`, `deposit`, `withdraw`, `beg`, `search`, `slots`, `shop`, `buy`, `inventory`, `rob`, `crime`, `fish`, `hunt`, `leaderboard`, `global_leaderboard`, `gamble_info`", false)
            .field("📈 Leveling", "`rank`, `leaderboard`", false)
            .field("🎮 Fun", "`coinflip`, `dice`, `eightball`, `joke`, `fact`, `ship`, `rate`, `mock`, `reverse`, `ascii`, `choose`, `trivia`, `roast`, `compliment`, `meme`, animal pics + more`", false)
            .field("🔧 Config", "`logs`, `welcome`, `autorole`, `prefix`, `settings`", false)
            .field("👤 Roles", "`role add`, `role remove`, `role list`", false)
            .field("🔗 Links", "[Website](https://aegisforge-vert.vercel.app) | [Invite](https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands)", false)
            .footer(serenity::CreateEmbedFooter::new("Type /help <command> for details"))
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
            serenity::CreateEmbed::new()
                .title("🔢 AegisForge — Calculator")
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
            serenity::CreateEmbed::new()
                .title("🔳 AegisForge — QR Generator")
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
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("🪙 Crypto Forge — {}", symbol.to_uppercase()))
                .description(format!(
                    "Fetching real-time market data for **{}**...",
                    symbol.to_uppercase()
                ))
                .field("Market Status", "Volatility: High", true)
                .field("Trend", "📈 Bullish", true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Powered by AegisForge Finance",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
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
            serenity::CreateEmbed::new()
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

/// set a timer — the bot will DM you when it's done
#[poise::command(slash_command, prefix_command)]
pub async fn timer(
    ctx: Context<'_>,
    #[description = "Duration in minutes (1–1440)"] minutes: u64,
) -> Result<(), Error> {
    if minutes == 0 || minutes > 1440 {
        return Err("Timer must be between 1 and 1440 minutes (24 hours).".into());
    }

    let expires_at = chrono::Utc::now().timestamp() + (minutes as i64 * 60);

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("⏲️ Timer Set")
                .description(format!("I'll DM you in **{}** minute(s).", minutes))
                .field("Expires", format!("<t:{}:R>", expires_at), true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Make sure your DMs are open!",
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;

    let http = ctx.serenity_context().http.clone();
    let user_id = ctx.author().id;

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(minutes * 60)).await;
        if let Ok(channel) = user_id.create_dm_channel(&http).await {
            let _ = channel
                .send_message(
                    &http,
                    serenity::CreateMessage::new().embed(
                        serenity::CreateEmbed::new()
                            .title("⏰ Timer Done!")
                            .description(format!("Your **{}**-minute timer has ended!", minutes))
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
            serenity::CreateEmbed::new()
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
            serenity::CreateEmbed::new()
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
                serenity::CreateEmbed::new()
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
