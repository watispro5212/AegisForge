use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::time::Duration;

/// kick someone out
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "KICK_MEMBERS",
    guild_only
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: serenity::User,
    #[description = "The reason for the kick"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    guild_id
        .kick_with_reason(ctx.http(), user.id, reason_str)
        .await?;

    // log to DB
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
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("👢 Member Kicked")
                .description(format!(
                    "**{}** has been expelled from the forge.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFF5722),
        ),
    )
    .await?;
    Ok(())
}

/// mute someone for a bit
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
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
    member
        .disable_communication_until_datetime(ctx.http(), until)
        .await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Timeout,
        Some(reason_str),
        Some(minutes as i64 * 60),
        Some(
            chrono::DateTime::from_timestamp(until.unix_timestamp(), 0)
                .unwrap()
                .with_timezone(&chrono::Utc),
        ),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("⏳ Member Timed Out")
                .description(format!(
                    "**{}** has been placed in temporary stasis.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("⏱️ Duration", format!("{} minute(s)", minutes), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFEE75C),
        ),
    )
    .await?;
    Ok(())
}

/// ban someone forever
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "BAN_MEMBERS",
    guild_only
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban"] user: serenity::User,
    #[description = "Delete message history (days, 0-7)"] delete_days: Option<u8>,
    #[description = "The reason for the ban"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let days = delete_days.unwrap_or(0).min(7);

    guild_id
        .ban_with_reason(ctx.http(), user.id, days, reason_str)
        .await?;

    // log to DB
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
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔨 Member Banned")
                .description(format!(
                    "**{}** has been permanently severed from the forge.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🗑️ History Cleared", format!("{} day(s)", days), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xED4245),
        ),
    )
    .await?;
    Ok(())
}

/// ban and then immediately unban to clear messages
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "BAN_MEMBERS",
    guild_only
)]
pub async fn softban(
    ctx: Context<'_>,
    #[description = "The user to softban"] user: serenity::User,
    #[description = "Delete message history (days, 1-7)"] delete_days: Option<u8>,
    #[description = "The reason for the softban"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let days = delete_days.unwrap_or(1).clamp(1, 7);

    guild_id
        .ban_with_reason(ctx.http(), user.id, days, reason_str)
        .await?;
    guild_id.unban(ctx.http(), user.id).await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Softban,
        Some(reason_str),
        None,
        None,
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("💨 Member Softbanned")
                .description(format!(
                    "**{}** has been softbanned (kicked + messages cleared).",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🗑️ History Cleared", format!("{} day(s)", days), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFF5722),
        ),
    )
    .await?;
    Ok(())
}

/// unban a user by ID
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "BAN_MEMBERS",
    guild_only
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user ID to unban"] user_id: serenity::UserId,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    guild_id.unban(ctx.http(), user_id).await?;

    // log to DB
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
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("✅ Member Unbanned")
                .description(format!(
                    "User `{}` has been unbanned from the forge.",
                    user_id
                ))
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x57F287),
        ),
    )
    .await?;
    Ok(())
}

/// warn a member and log the infraction
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "The user to warn"] user: serenity::User,
    #[description = "The reason for the warning"] reason: String,
) -> Result<(), Error> {
    // log to DB
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
    )
    .await?;

    crate::db::warnings::create(
        pool,
        guild_id,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        &reason,
        Some(case.id),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("⚠️ Warning Issued")
                .description(format!("**{}** has received a formal warning.", user.name))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🆔 Case", format!("#{}", case.case_number), true)
                .field("📝 Reason", &reason, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFEE75C),
        ),
    )
    .await?;
    Ok(())
}

/// purge a number of messages from this channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
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

/// completely clear the channel by re-creating it
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn nuke(ctx: Context<'_>) -> Result<(), Error> {
    let channel = ctx
        .guild_channel()
        .await
        .ok_or("Must be in a guild channel")?;
    let position = channel.position;

    let mut builder = serenity::CreateChannel::new(channel.name.clone())
        .kind(channel.kind)
        .topic(channel.topic.clone().unwrap_or_default())
        .nsfw(channel.nsfw)
        .permissions(channel.permission_overwrites.clone())
        .position(position as u16);

    if let Some(parent) = channel.parent_id {
        builder = builder.category(parent);
    }

    let new_channel = channel.guild_id.create_channel(ctx.http(), builder).await?;

    channel.delete(ctx.http()).await?;

    new_channel.send_message(ctx.http(), serenity::CreateMessage::new().embed(
        serenity::CreateEmbed::new()
            .title("💥 Channel Nuked")
            .description("The forge has been reset. All previous messages were vaporized.")
            .image("https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExMngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3JmVwPXYxX2ludGVybmFsX2dpZl9ieV9pZCZjdD1n/HhTXt43pk1I1W/giphy.gif")
            .footer(serenity::CreateEmbedFooter::new("Forged anew | AegisForge v4"))
            .color(0x00E5FF)
    )).await?;

    Ok(())
}

/// set the slowmode for the current channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "Delay in seconds (0 to disable)"] seconds: u64,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();

    // we use edit_channel to update rate_limit_per_user
    channel_id
        .edit(
            ctx.http(),
            serenity::EditChannel::default().rate_limit_per_user(seconds as u16),
        )
        .await?;

    if seconds == 0 {
        ctx.say("✅ Slowmode has been **disabled** for this channel.")
            .await?;
    } else {
        ctx.say(format!(
            "✅ Slowmode has been set to **{} seconds**.",
            seconds
        ))
        .await?;
    }

    Ok(())
}

/// lock the current channel (denies @everyone SEND_MESSAGES)
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
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

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔒 Channel Locked")
                .description("Public messaging has been disabled in this channel.")
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .timestamp(serenity::Timestamp::now())
                .color(0xED4245),
        ),
    )
    .await?;

    Ok(())
}

/// unlock the current channel (removes @everyone SEND_MESSAGES deny)
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn unlock(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let role_id = guild_id.everyone_role();

    channel_id
        .delete_permission(ctx.http(), serenity::PermissionOverwriteType::Role(role_id))
        .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔓 Channel Unlocked")
                .description("Public messaging has been re-enabled.")
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .timestamp(serenity::Timestamp::now())
                .color(0x57F287),
        ),
    )
    .await?;

    Ok(())
}

/// mute a member (alias for timeout with 1 hour default)
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The user to mute"] user: serenity::User,
    #[description = "Duration in minutes (defaults to 60)"] minutes: Option<u64>,
    #[description = "The reason for the mute"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let m = minutes.unwrap_or(60);

    let until =
        serenity::Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + (m as i64 * 60))?;

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member
        .disable_communication_until_datetime(ctx.http(), until)
        .await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        guild_id.get() as i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        crate::models::mod_case::ModAction::Mute,
        Some(reason_str),
        Some(m as i64 * 60),
        Some(
            chrono::DateTime::from_timestamp(until.unix_timestamp(), 0)
                .unwrap()
                .with_timezone(&chrono::Utc),
        ),
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔇 Member Muted")
                .description(format!(
                    "**{}** has been muted for **{}** minute(s).",
                    user.name, m
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("⏱️ Duration", format!("{} minute(s)", m), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFEE75C),
        ),
    )
    .await?;
    Ok(())
}

/// unmute a member (removes timeout)
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "The user to unmute"] user: serenity::User,
    #[description = "The reason for the unmute"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member.enable_communication(ctx.http()).await?;

    // log to DB
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
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔊 Member Unmuted")
                .description(format!("**{}** has been unmuted.", user.name))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x57F287),
        ),
    )
    .await?;
    Ok(())
}
