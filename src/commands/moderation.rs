use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::time::Duration;

/// Kick a member from the server
#[poise::command(slash_command, prefix_command, required_permissions = "KICK_MEMBERS", guild_only)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: serenity::User,
    #[description = "The reason for the kick"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    guild_id.kick_with_reason(ctx.http(), user.id, reason_str).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Member Kicked")
            .description(format!("**{}** has been kicked.", user.name))
            .field("Reason", reason_str, false)
            .color(0xff4500),
    ))
    .await?;
    Ok(())
}

/// Timeout (mute) a member temporarily
#[poise::command(slash_command, prefix_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "The user to timeout"] user: serenity::User,
    #[description = "Duration in minutes (1-40320)"] minutes: u64,
    #[description = "The reason for the timeout"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    let until = serenity::Timestamp::from_unix_timestamp(
        chrono::Utc::now().timestamp() + (minutes as i64 * 60),
    )?;

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member.disable_communication_until_datetime(ctx.http(), until).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Member Timed Out")
            .description(format!("**{}** has been timed out for **{} minute(s)**.", user.name, minutes))
            .field("Reason", reason_str, false)
            .color(0xff4500),
    ))
    .await?;
    Ok(())
}

/// Ban a member from the server
#[poise::command(slash_command, prefix_command, required_permissions = "BAN_MEMBERS", guild_only)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban"] user: serenity::User,
    #[description = "Delete message history (days, 0-7)"] delete_days: Option<u8>,
    #[description = "The reason for the ban"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let days = delete_days.unwrap_or(0).min(7);

    guild_id.ban_with_reason(ctx.http(), user.id, days, reason_str).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Member Banned")
            .description(format!("**{}** has been permanently banned.", user.name))
            .field("Reason", reason_str, false)
            .field("Messages Deleted", format!("{} day(s)", days), true)
            .color(0xff0000),
    ))
    .await?;
    Ok(())
}

/// Unban a user by ID
#[poise::command(slash_command, prefix_command, required_permissions = "BAN_MEMBERS", guild_only)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user ID to unban"] user_id: serenity::UserId,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    guild_id.unban(ctx.http(), user_id).await?;

    ctx.say(format!("Successfully unbanned user `{}`.", user_id)).await?;
    Ok(())
}

/// Warn a member and log the infraction
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_MESSAGES", guild_only)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "The user to warn"] user: serenity::User,
    #[description = "The reason for the warning"] reason: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Warning Issued")
            .description(format!("**{}** has been warned.", user.name))
            .field("Reason", &reason, false)
            .footer(serenity::CreateEmbedFooter::new("Moderation action logged."))
            .color(0xffaa00),
    ))
    .await?;
    Ok(())
}

/// Purge a number of messages from this channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_MESSAGES", guild_only)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages to delete (1-100)"] amount: u64,
) -> Result<(), Error> {
    let channel = ctx.channel_id();
    let limit = amount.clamp(1, 100) as u8;

    let messages = channel
        .messages(ctx.http(), serenity::GetMessages::default().limit(limit))
        .await?;

    let ids: Vec<serenity::MessageId> = messages.iter().map(|m| m.id).collect();
    let count = ids.len();
    channel.delete_messages(ctx.http(), &ids).await?;

    let reply = ctx
        .say(format!("🗑 Deleted **{}** message(s).", count))
        .await?;

    tokio::time::sleep(Duration::from_secs(4)).await;
    reply.delete(ctx).await?;
    Ok(())
}
