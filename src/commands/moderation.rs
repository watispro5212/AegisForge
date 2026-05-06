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
    
    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Kick,
        Some(reason_str),
        None,
        None,
    ).await?;

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

    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Timeout,
        Some(reason_str),
        Some(minutes as i64 * 60),
        Some(chrono::DateTime::from_timestamp(until.unix_timestamp(), 0).unwrap().with_timezone(&chrono::Utc)),
    ).await?;

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

    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Ban,
        Some(reason_str),
        None,
        None,
    ).await?;

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

    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user_id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Unban,
        None,
        None,
        None,
    ).await?;

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
    // Log to DB
    let pool = &ctx.data().database.pool;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let case = crate::db::mod_cases::create_case(
        pool,
        guild_id,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Warn,
        Some(&reason),
        None,
        None,
    ).await?;

    crate::db::warnings::create(
        pool,
        guild_id,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        &reason,
        Some(case.id),
    ).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Warning Issued")
            .description(format!("**{}** has been warned.", user.name))
            .field("Reason", &reason, false)
            .field("Case Number", format!("#{}", case.case_number), true)
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

/// Set the slowmode for the current channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_CHANNELS", guild_only)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "Delay in seconds (0 to disable)"] seconds: u64,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    
    // We use edit_channel to update rate_limit_per_user
    channel_id.edit(ctx.http(), serenity::EditChannel::default().rate_limit_per_user(seconds as u16)).await?;

    if seconds == 0 {
        ctx.say("✅ Slowmode has been **disabled** for this channel.").await?;
    } else {
        ctx.say(format!("✅ Slowmode has been set to **{} seconds**.", seconds)).await?;
    }
    
    Ok(())
}

/// Lock the current channel (denies @everyone SEND_MESSAGES)
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_CHANNELS", guild_only)]
pub async fn lock(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let role_id = guild_id.everyone_role();

    let overwrite = serenity::PermissionOverwrite {
        allow: serenity::Permissions::empty(),
        deny: serenity::Permissions::SEND_MESSAGES,
        kind: serenity::PermissionOverwriteType::Role(role_id),
    };

    channel_id.create_permission(ctx.http(), overwrite).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔒 Channel Locked")
            .description("Public messaging has been disabled in this channel.")
            .color(0xff0000),
    )).await?;

    Ok(())
}

/// Unlock the current channel (removes @everyone SEND_MESSAGES deny)
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_CHANNELS", guild_only)]
pub async fn unlock(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let role_id = guild_id.everyone_role();

    channel_id.delete_permission(ctx.http(), serenity::PermissionOverwriteType::Role(role_id)).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔓 Channel Unlocked")
            .description("Public messaging has been re-enabled.")
            .color(0x00ff00),
    )).await?;

    Ok(())
}

/// Mute a member (alias for timeout with 1 hour default)
#[poise::command(slash_command, prefix_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The user to mute"] user: serenity::User,
    #[description = "Duration in minutes (defaults to 60)"] minutes: Option<u64>,
    #[description = "The reason for the mute"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let m = minutes.unwrap_or(60);

    let until = serenity::Timestamp::from_unix_timestamp(
        chrono::Utc::now().timestamp() + (m as i64 * 60),
    )?;

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member.disable_communication_until_datetime(ctx.http(), until).await?;

    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Mute,
        Some(reason_str),
        Some(m as i64 * 60),
        Some(chrono::DateTime::from_timestamp(until.unix_timestamp(), 0).unwrap().with_timezone(&chrono::Utc)),
    ).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Member Muted")
            .description(format!("**{}** has been muted for **{} minute(s)**.", user.name, m))
            .field("Reason", reason_str, false)
            .color(0xff4500),
    ))
    .await?;
    Ok(())
}


/// Unmute a member (removes timeout)
#[poise::command(slash_command, prefix_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "The user to unmute"] user: serenity::User,
    #[description = "The reason for the unmute"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member.enable_communication(ctx.http()).await?;

    // Log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Unmute,
        Some(reason_str),
        None,
        None,
    ).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("Member Unmuted")
            .description(format!("**{}** has been unmuted.", user.name))
            .field("Reason", reason_str, false)
            .color(0x00ff00),
    ))
    .await?;
    Ok(())
}


