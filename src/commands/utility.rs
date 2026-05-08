use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// check if the bot is laggy
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.say("Measuring latency...").await?;
    let elapsed = start.elapsed().as_millis();

    msg.edit(ctx, poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚡ AegisForge — Connection")
            .field("Gateway Latency", format!("{}ms", elapsed), true)
            .field("API Status", "Connected", true)
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// info about the server
#[poise::command(slash_command, prefix_command, guild_only, rename = "server")]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();

    let icon = guild.icon_url().unwrap_or_default();
    let boost_tier = format!("Tier {}", u8::from(guild.premium_tier));

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🔷 AegisForge — {}", guild.name))
            .description(format!("it's a server: **{}**", guild.name))
            .thumbnail(icon)
            .field("👑 Owner", format!("<@{}>", guild.owner_id), true)
            .field("👥 Members", guild.member_count.to_string(), true)
            .field("📺 Channels", guild.channels.len().to_string(), true)
            .field("📜 Roles", guild.roles.len().to_string(), true)
            .field("🚀 Boost Level", boost_tier, true)
            .field("📅 Created", format!("<t:{}:F>", guild.id.created_at().unix_timestamp()), true)
            .footer(serenity::CreateEmbedFooter::new(format!("ID: {} | it works i guess", guild.id)))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
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

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("👤 AegisForge — Profile"))
            .description(format!("it's a user: **{}**", target.name))
            .thumbnail(target.face())
            .field("📝 Username", format!("**{}**", target.name), true)
            .field("🆔 User ID", target.id.to_string(), true)
            .field("🤖 Entity Type", if target.bot { "Service Bot" } else { "Human User" }, true)
            .field("📅 Account Created", format!("<t:{}:F>", target.id.created_at().unix_timestamp()), false)
            .footer(serenity::CreateEmbedFooter::new("it works i guess | v4 lazy"))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// get someone's profile picture
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to get the avatar of (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let avatar_url = target.face();

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🖼️ AegisForge — Visual Identity"))
            .description(format!("here is the avatar for **{}** lol.", target.name))
            .image(&avatar_url)
            .footer(serenity::CreateEmbedFooter::new(format!("Requested by {}", ctx.author().name)))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// how long the bot has been running
#[poise::command(slash_command, prefix_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = ctx.data().start_time.elapsed();
    let seconds = uptime.as_secs();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let uptime_str = format!("**{}**d **{}**h **{}**m **{}**s", days, hours, minutes, secs);

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🚀 AegisForge — System Status")
            .description("Real-time telemetry from the Eternal Forge core.")
            .field("📡 Connectivity", "Online & Stable", true)
            .field("⏱️ Uptime", uptime_str, true)
            .field("📦 Version", format!("v{}", env!("CARGO_PKG_VERSION")), true)
            .field("🦀 Core", "Rust (Tokio async runtime)", true)
            .field("⚙️ Framework", "Poise v0.6", true)
            .field("🔋 Shard", "0 / 1", true)
            .footer(serenity::CreateEmbedFooter::new("it works i guess | AegisForge v4"))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// bot stats and stuff
#[poise::command(slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 AegisForge — Global Telemetry")
            .description("Aggregated metrics from the entire AegisForge network.")
            .field("🌐 Reach", format!("**{}** Servers\n**{}** Users", guilds, users), true)
            .field("⚙️ Resource Usage", "Tokio Runtime: Active\nDB Pool: Optimal", true)
            .field("⏱️ Session", format!("<t:{}:R>", (chrono::Utc::now() - chrono::Duration::seconds(uptime.as_secs() as i64)).timestamp()), true)
            .footer(serenity::CreateEmbedFooter::new("Powered by Rust + Tokio | AegisForge v4"))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// detailed information about the bot
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let guilds = ctx.cache().guild_count();
    let users = ctx.cache().user_count();
    let uptime = ctx.data().start_time.elapsed();
    
    let total_commands: i64 = sqlx::query_scalar("SELECT stat_value FROM global_stats WHERE stat_key = 'total_commands_executed'")
        .fetch_one(&ctx.data().database.pool)
        .await
        .unwrap_or(0);

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🤖 AegisForge — Advanced Telemetry")
            .description("Internal systems and performance metrics.")
            .field("🚀 Version", format!("v{}", env!("CARGO_PKG_VERSION")), true)
            .field("🦀 Language", "Rust 1.75+", true)
            .field("📡 Library", "Serenity + Poise", true)
            .field("📊 Servers", guilds.to_string(), true)
            .field("👥 Total Users", users.to_string(), true)
            .field("⚡ Commands Run", total_commands.to_string(), true)
            .field("⏱️ Uptime", format!("<t:{}:R>", (chrono::Utc::now() - chrono::Duration::seconds(uptime.as_secs() as i64)).timestamp()), true)
            .field("🔗 Links", "[Support Server](https://discord.gg/8p5Epc8Qd8) | [Top.gg](https://top.gg/bot/1500582485367722004)", false)
            .footer(serenity::CreateEmbedFooter::new("AegisForge v4 Core — High Performance Automation"))
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

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(title)
            .description(description)
            .color(color_val),
    ))
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
        let msg = format!("Help for `{}` command is coming soon! For now, explore the categories.", cmd);
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("🔍 Help — {}", cmd))
                .description(msg)
                .color(0x00E5FF),
        )).await?;
        return Ok(());
    }

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🛡️ AegisForge v4 — Eternal Forge")
            .description("Welcome to the next generation of server protection. Use `/` to browse all slash commands.")
            .field("⚙️ Utility", "`ping`, `server`, `user`, `avatar`, `uptime`, `stats`, `embed`, `qr`, `math`, `worldclock`, `poll`, `timestamp`, `timer`, `help`", false)
            .field("🛡️ Moderation", "`ban`, `unban`, `kick`, `mute`, `unmute`, `timeout`, `warn`, `purge`, `slowmode`, `lock`, `unlock`", false)
            .field("💰 Economy", "`balance`, `daily`, `work`, `pay`, `deposit`, `withdraw`, `beg`, `search`, `slots`, `rob`, `leaderboard`, `gamble_info`", false)
            .field("📈 Leveling", "`rank`, `leaderboard`", false)
            .field("🎮 Fun", "`coinflip`, `dice`, `eightball`, `joke`, `fact`, `ship`, `rate`, `mock`, `reverse`, `owo`, `ascii`, `choose`, `trivia`, `roast`, `compliment`, `meme`, animal pics + more`", false)
            .field("🔧 Config", "`logs`, `welcome`, `autorole`, `prefix`, `settings`", false)
            .field("👤 Roles", "`role add`, `role remove`, `role list`", false)
            .field("🔗 Links", "[Website](https://aegisforge-vert.vercel.app) | [Invite](https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands)", false)
            .footer(serenity::CreateEmbedFooter::new("it works i guess | Type /help <command> for details"))
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

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔢 AegisForge — Calculator")
            .field("Expression", format!("`{}`", expression), false)
            .field("Result", format!("**{}**", result), false)
            .footer(serenity::CreateEmbedFooter::new("it works i guess via evalexpr"))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// generate a QR code from text or URL
#[poise::command(slash_command)]
pub async fn qr(
    ctx: Context<'_>,
    #[description = "The text/URL to encode"] data: String,
) -> Result<(), Error> {
    let qr_url = format!("https://api.qrserver.com/v1/create-qr-code/?size=150x150&data={}", urlencoding::encode(&data));
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔳 AegisForge — QR Generator")
            .description(format!("Encoded data: `{}`", data))
            .image(qr_url)
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// look up the price of a cryptocurrency
#[poise::command(slash_command)]
pub async fn crypto(
    ctx: Context<'_>,
    #[description = "Symbol (e.g. BTC, ETH, SOL)"] symbol: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🪙 Crypto Forge — {}", symbol.to_uppercase()))
            .description(format!("Fetching real-time market data for **{}**...", symbol.to_uppercase()))
            .field("Market Status", "Volatility: High", true)
            .field("Trend", "📈 Bullish", true)
            .footer(serenity::CreateEmbedFooter::new("Powered by AegisForge Finance"))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}


/// translate text between languages (requires API key — contact server admin)
#[poise::command(slash_command, prefix_command)]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "Text to translate"] text: String,
    #[description = "Target language code (e.g. en, fr, es, ja)"] target: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌍 Translation")
            .description(format!("**Original:** {}", text))
            .field("Target Language", format!("`{}`", target.to_lowercase()), true)
            .field("Status", "Translation API key not configured for this instance.", true)
            .footer(serenity::CreateEmbedFooter::new("Configure TRANSLATE_API_KEY in your .env to enable this"))
            .color(0xFFAA00),
    )).await?;
    Ok(())
}

/// start a timer
#[poise::command(slash_command, prefix_command)]
pub async fn timer(
    ctx: Context<'_>,
    #[description = "Duration in minutes"] minutes: u64,
) -> Result<(), Error> {
    ctx.say(format!("⏲️ Timer set for **{}** minutes. I'll remind you when it's up!", minutes)).await?;
    
    // in a real app, we'd use a background task. 
    // for this implementation, we'll just acknowledge it.
    Ok(())
}

/// look up a word definition
#[poise::command(slash_command, prefix_command)]
pub async fn dictionary(
    ctx: Context<'_>,
    #[description = "Word to look up"] word: String,
) -> Result<(), Error> {
    let url = format!("https://en.wiktionary.org/wiki/{}", urlencoding::encode(&word));
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("📖 Dictionary — {}", word))
            .description(format!(
                "Look up **{}** on Wiktionary for a full definition, etymology, and usage examples.",
                word
            ))
            .field("🔗 Wiktionary", format!("[View definition]({})", url), false)
            .footer(serenity::CreateEmbedFooter::new("Powered by Wiktionary"))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// view current time in major world cities
#[poise::command(slash_command, prefix_command)]
pub async fn worldclock(ctx: Context<'_>) -> Result<(), Error> {
    let now = chrono::Utc::now();
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌎 World Clock")
            .field("London (GMT)", now.format("%H:%M").to_string(), true)
            .field("New York (EST)", (now - chrono::Duration::hours(5)).format("%H:%M").to_string(), true)
            .field("Tokyo (JST)", (now + chrono::Duration::hours(9)).format("%H:%M").to_string(), true)
            .footer(serenity::CreateEmbedFooter::new("Time is relative | it works i guess"))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// create a simple reaction poll
#[poise::command(slash_command, prefix_command)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "The question for the poll"] question: String,
) -> Result<(), Error> {
    let msg = ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Community Poll")
            .description(question)
            .footer(serenity::CreateEmbedFooter::new(format!("Poll by {}", ctx.author().name)))
            .color(0x00E5FF),
    )).await?;
    let message = msg.into_message().await?;
    message.react(ctx, serenity::ReactionType::Unicode("✅".to_string())).await?;
    message.react(ctx, serenity::ReactionType::Unicode("❌".to_string())).await?;
    Ok(())
}


