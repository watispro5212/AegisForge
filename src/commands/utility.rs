use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Check the bot's latency and connection status
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.say("Measuring latency...").await?;
    let elapsed = start.elapsed().as_millis();

    msg.edit(ctx, poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title("⚡ AegisForge — Latency")
            .field("API Round-trip", format!("{}ms", elapsed), true)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Display information about this server
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title(format!("🔷 Server — {}", guild.name))
            .thumbnail(guild.icon_url().unwrap_or_default())
            .field("Owner", format!("<@{}>", guild.owner_id), true)
            .field("Members", guild.member_count.to_string(), true)
            .field("Channels", guild.channels.len().to_string(), true)
            .field("Roles", guild.roles.len().to_string(), true)
            .field("Boost Level", format!("Tier {}", guild.premium_tier.num()), true)
            .field("Created", format!("<t:{}:R>", guild.id.created_at().unix_timestamp()), true)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Display information about a user
#[poise::command(slash_command, prefix_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to look up (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
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
        serenity::CreateEmbed::default()
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
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title("🔩 AegisForge — System Status")
            .field("Version", env!("CARGO_PKG_VERSION"), true)
            .field("Language", "Rust 🦀", true)
            .field("Framework", "Poise + Serenity", true)
            .footer(serenity::CreateEmbedFooter::new("Forged with precision."))
            .color(0x00ffff),
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
