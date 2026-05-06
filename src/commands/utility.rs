use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Check the bot's latency and connection status
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

/// Display information about this server
#[poise::command(slash_command, prefix_command, guild_only, rename = "server")]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();

    let icon = guild.icon_url().unwrap_or_default();
    let boost_tier = format!("Tier {}", u8::from(guild.premium_tier));

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🔷 Server — {}", guild.name))
            .thumbnail(icon)
            .field("Owner", format!("<@{}>", guild.owner_id), true)
            .field("Members", guild.member_count.to_string(), true)
            .field("Channels", guild.channels.len().to_string(), true)
            .field("Roles", guild.roles.len().to_string(), true)
            .field("Boost Level", boost_tier, true)
            .field("Created", format!("<t:{}:R>", guild.id.created_at().unix_timestamp()), true)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Display information about a user
#[poise::command(slash_command, prefix_command, rename = "user")]
pub async fn whois(
    ctx: Context<'_>,
    #[description = "The user to look up (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("👤 User — {}", target.name))
            .thumbnail(target.face())
            .field("ID", target.id.to_string(), true)
            .field("Bot", if target.bot { "Yes" } else { "No" }, true)
            .field("Created", format!("<t:{}:R>", target.id.created_at().unix_timestamp()), true)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Get the avatar of a user
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to get the avatar of (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());
    let avatar_url = target.face();

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("{}'s Avatar", target.name))
            .image(&avatar_url)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Show the bot's uptime and version info
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
            .field("🦀 Core", "Rust 1.95 (Tokio)", true)
            .field("⚙️ Framework", "Poise v0.6", true)
            .field("🔋 Shard", "0 / 1", true)
            .footer(serenity::CreateEmbedFooter::new("Forged with precision | AegisForge v3"))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// Send a custom embed to the current channel
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

/// Generate a Discord timestamp for a given UNIX timestamp
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

/// Displays all available commands
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
            .title("🛡️ AegisForge v3 — Eternal Forge")
            .description("Welcome to the next generation of server protection. Use `/` to browse all slash commands.")
            .field("⚙️ Utility", "`ping`, `server`, `user`, `avatar`, `uptime`, `timestamp`, `help`", false)
            .field("🔨 Moderation", "`ban`, `kick`, `mute`, `unmute`, `purge`, `warn`, `timeout` (Audit logging enabled)", false)
            .field("💰 Economy", "`balance`, `daily`, `work`, `pay`, `leaderboard`", false)
            .field("📈 Leveling", "`rank`, `leaderboard`", false)
            .field("🔧 Config", "`logs`, `welcome`, `autorole`, `prefix`", false)
            .field("🔗 Links", "[Website](https://aegisforge.fly.dev) | [Invite](https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands)", false)
            .footer(serenity::CreateEmbedFooter::new("Forged with precision | Type /help <command> for details"))
            .timestamp(serenity::Timestamp::now())
            .color(0x00E5FF),
    )).await?;
    
    Ok(())
}
