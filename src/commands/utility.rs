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
#[poise::command(slash_command, prefix_command)]
pub async fn user(
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
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
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

/// Flip a coin
#[poise::command(slash_command, prefix_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let result = if rand::random::<bool>() { "Heads" } else { "Tails" };
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🪙 Coin Flip")
            .description(format!("The coin landed on **{}**!", result))
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Roll a dice
#[poise::command(slash_command, prefix_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Number of sides (defaults to 6)"] sides: Option<u32>,
) -> Result<(), Error> {
    let sides = sides.unwrap_or(6).max(2);
    let result = (rand::random::<u32>() % sides) + 1;
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎲 Dice Roll")
            .description(format!("You rolled a **{}** (1-{})", result, sides))
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Create a simple poll
#[poise::command(slash_command, prefix_command)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "The question to ask"] question: String,
) -> Result<(), Error> {
    let msg = ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Poll")
            .description(question)
            .color(0x00ffff),
    ))
    .await?
    .into_message()
    .await?;

    msg.react(ctx, serenity::ReactionType::Unicode("👍".to_string())).await?;
    msg.react(ctx, serenity::ReactionType::Unicode("👎".to_string())).await?;
    msg.react(ctx, serenity::ReactionType::Unicode("🤷".to_string())).await?;

    Ok(())
}

/// Display detailed information about the current server
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild_id) = ctx.guild_id() {
        let guild = guild_id.to_partial_guild(&ctx.http()).await?;
        let members = guild.member_count;
        let created_at = guild.id.created_at();
        
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("Server Info: {}", guild.name))
                .description(format!("**Owner:** <@{}>\n**Members:** {}\n**Created:** <t:{}:F>", guild.owner_id, members, created_at.unix_timestamp()))
                .color(0x00ffff),
        ))
        .await?;
    } else {
        ctx.say("This command can only be used in a server!").await?;
    }
    Ok(())
}

/// Display detailed information about a specific user
#[poise::command(slash_command, prefix_command)]
pub async fn whois(
    ctx: Context<'_>,
    #[description = "The user to inspect"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or(ctx.author());
    let created_at = u.id.created_at();
    
    let mut embed = serenity::CreateEmbed::new()
        .title(format!("User Info: {}", u.name))
        .description(format!("**ID:** {}\n**Account Created:** <t:{}:F>", u.id, created_at.unix_timestamp()))
        .color(0x00ffff);
        
    if let Some(avatar_url) = u.avatar_url() {
        embed = embed.thumbnail(avatar_url);
    }
    
    if let Some(guild_id) = ctx.guild_id() {
        if let Ok(member) = guild_id.member(&ctx.http(), u.id).await {
            if let Some(joined_at) = member.joined_at {
                embed = embed.field("Joined Server", format!("<t:{}:F>", joined_at.unix_timestamp()), false);
            }
        }
    }
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
