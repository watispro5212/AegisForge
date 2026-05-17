use crate::db::mod_cases::NewModCase;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedFooter, Timestamp};
use serenity::all::ChannelType;
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
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    guild_id
        .kick_with_reason(ctx.http(), user.id, reason_str)
        .await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Kick,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("👢 Member Kicked")
                .description(format!(
                    "**{}** has been kicked from the server.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .field("📝 Reason", reason_str, false)
                .footer(CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
                ))
                .timestamp(Timestamp::now())
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
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    let until =
        Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + (minutes as i64 * 60))
            .map_err(|_| "Invalid timestamp")?;

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member
        .disable_communication_until_datetime(ctx.http(), until)
        .await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Timeout,
            reason: Some(reason_str),
            duration_secs: Some(minutes as i64 * 60),
            expires_at: Some(
                chrono::DateTime::from_timestamp(until.unix_timestamp(), 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            ),
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("⏳ Member Timed Out")
                .description(format!(
                    "**{}** has been placed in temporary stasis.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("⏱️ Duration", format!("{} minute(s)", minutes), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
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
    ctx.defer().await?;
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
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Ban,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔨 Member Banned")
                .description(format!(
                    "**{}** has been permanently banned from the server.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🗑️ History Cleared", format!("{} day(s)", days), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
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
    ctx.defer().await?;
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
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Softban,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("💨 Member Softbanned")
                .description(format!(
                    "**{}** has been softbanned (kicked + messages cleared).",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🗑️ History Cleared", format!("{} day(s)", days), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
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
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    guild_id.unban(ctx.http(), user_id).await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user_id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Unban,
            reason: None,
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("✅ Member Unbanned")
                .description(format!(
                    "User `{}` has been unbanned from the server.",
                    user_id
                ))
                .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
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
    ctx.defer().await?;
    // log to DB
    let pool = &ctx.data().database.pool;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let case = crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Warn,
            reason: Some(&reason),
            duration_secs: None,
            expires_at: None,
        },
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
            CreateEmbed::new()
                .title("⚠️ Warning Issued")
                .description(format!("**{}** has received a formal warning.", user.name))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🆔 Case", format!("#{}", case.case_number), true)
                .field("📝 Reason", &reason, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
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
    ctx.defer().await?;
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
    ctx.defer().await?;
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
        CreateEmbed::new()
            .title("💥 Channel Nuked")
            .description("The forge has been reset. All previous messages were deleted.")
            .image("https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExMngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3MngxNXN3JmVwPXYxX2ludGVybmFsX2dpZl9ieV9pZCZjdD1n/HhTXt43pk1I1W/giphy.gif")
            .footer(serenity::CreateEmbedFooter::new("Forged anew | AegisForge v4.2"))
            .color(0x00E5FF)
    )).await?;

    Ok(())
}

/// set slowmode for the current or a specific channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "Seconds of slowmode (0 to disable)"] seconds: u64,
    #[description = "Channel to apply slowmode to (defaults to current)"] channel: Option<
        serenity::GuildChannel,
    >,
) -> Result<(), Error> {
    ctx.defer().await?;
    let target_channel = channel.map(|c| c.id).unwrap_or(ctx.channel_id());

    target_channel
        .edit(
            ctx.http(),
            serenity::EditChannel::new().rate_limit_per_user(seconds as u16),
        )
        .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🐌 Slowmode Updated")
                .description(format!(
                    "Slowmode for <#{}> set to **{}s**.",
                    target_channel, seconds
                ))
                .color(0xFFAA00)
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.2")),
        ),
    )
    .await?;

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
            CreateEmbed::new()
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
            CreateEmbed::new()
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
    ctx.defer().await?;
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
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Mute,
            reason: Some(reason_str),
            duration_secs: Some(m as i64 * 60),
            expires_at: Some(
                chrono::DateTime::from_timestamp(until.unix_timestamp(), 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            ),
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔇 Member Muted")
                .description(format!(
                    "**{}** has been muted for **{}** minute(s).",
                    user.name, m
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("⏱️ Duration", format!("{} minute(s)", m), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFEE75C),
        ),
    )
    .await?;
    Ok(())
}

/// silently restrict a member without notifying them
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only,
    ephemeral
)]
pub async fn shadowban(
    ctx: Context<'_>,
    #[description = "The user to shadow ban"] user: serenity::User,
    #[description = "Internal reason (never shown to the target)"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");
    let pool = &ctx.data().database.pool;
    let guild_id_i64 = guild_id.get() as i64;

    let config = ctx.data().database.get_guild_config(guild_id_i64).await?;
    let mute_role_id = match config.mute_role_id {
        Some(id) => serenity::RoleId::new(id as u64),
        None => {
            ctx.say("❌ No mute role configured. Set one first with `/muterole <role>`.")
                .await?;
            return Ok(());
        }
    };

    if user.id == ctx.author().id {
        ctx.say("❌ You cannot shadow ban yourself.").await?;
        return Ok(());
    }

    // apply mute role silently — no DM, no public message
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.add_role(ctx.http(), mute_role_id).await?;

    // record in shadow_bans table
    crate::db::guild::add_shadow_ban(
        pool,
        guild_id_i64,
        user.id.get() as i64,
        ctx.author().id.get() as i64,
        Some(reason_str),
    )
    .await?;

    // log to mod_cases
    let case = crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id_i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::ShadowBan,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    // log to mod log channel only
    if let Some(log_channel) = config.mod_log_channel {
        let embed = CreateEmbed::new()
            .title("👤 Shadow Ban Applied")
            .description(format!(
                "**{}** (`{}`) has been silently restricted.",
                user.name, user.id
            ))
            .field("👤 Target", format!("<@{}>", user.id), true)
            .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
            .field("🆔 Case", format!("#{}", case.case_number), true)
            .field("📝 Reason", reason_str, false)
            .field("ℹ️ Note", "The target has not been notified.", false)
            .footer(serenity::CreateEmbedFooter::new(
                "AegisForge Shadow Ban | Visible to moderators only",
            ))
            .timestamp(serenity::Timestamp::now())
            .color(0x2C2F33);

        let _ = serenity::ChannelId::new(log_channel as u64)
            .send_message(
                ctx.http(),
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }

    // ephemeral confirmation — only the invoking mod sees this
    ctx.send(
        poise::CreateReply::default().ephemeral(true).embed(
            CreateEmbed::new()
                .title("👤 Shadow Ban Applied")
                .description(format!(
                    "**{}** has been silently restricted. They have not been notified.",
                    user.name
                ))
                .field("🆔 Case", format!("#{}", case.case_number), true)
                .field("📝 Reason", reason_str, false)
                .color(0x2C2F33),
        ),
    )
    .await?;
    Ok(())
}

/// lift a shadow ban and restore a member's ability to interact
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only,
    ephemeral
)]
pub async fn unshadowban(
    ctx: Context<'_>,
    #[description = "The user to un-shadow ban"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let pool = &ctx.data().database.pool;
    let guild_id_i64 = guild_id.get() as i64;

    let config = ctx.data().database.get_guild_config(guild_id_i64).await?;
    let mute_role_id = match config.mute_role_id {
        Some(id) => serenity::RoleId::new(id as u64),
        None => {
            ctx.say("❌ No mute role configured. Set one with `/muterole <role>`.")
                .await?;
            return Ok(());
        }
    };

    let was_banned =
        crate::db::guild::remove_shadow_ban(pool, guild_id_i64, user.id.get() as i64).await?;
    if !was_banned {
        ctx.say(format!("❌ **{}** is not shadow banned.", user.name))
            .await?;
        return Ok(());
    }

    // remove mute role silently
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.remove_role(ctx.http(), mute_role_id).await?;

    // log to mod_cases
    let case = crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id_i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::ShadowUnban,
            reason: None,
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    // log to mod log channel
    if let Some(log_channel) = config.mod_log_channel {
        let embed = CreateEmbed::new()
            .title("👤 Shadow Ban Lifted")
            .description(format!(
                "**{}** (`{}`) has been silently restored.",
                user.name, user.id
            ))
            .field("👤 Target", format!("<@{}>", user.id), true)
            .field("🛡️ Moderator", format!("<@{}>", ctx.author().id), true)
            .field("🆔 Case", format!("#{}", case.case_number), true)
            .footer(serenity::CreateEmbedFooter::new(
                "AegisForge Shadow Ban | Visible to moderators only",
            ))
            .timestamp(serenity::Timestamp::now())
            .color(0x57F287);

        let _ = serenity::ChannelId::new(log_channel as u64)
            .send_message(
                ctx.http(),
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }

    ctx.send(
        poise::CreateReply::default().ephemeral(true).embed(
            CreateEmbed::new()
                .title("👤 Shadow Ban Lifted")
                .description(format!("**{}** can interact normally again.", user.name))
                .field("🆔 Case", format!("#{}", case.case_number), true)
                .color(0x57F287),
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
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("No reason provided");

    let mut member = guild_id.member(ctx.http(), user.id).await?;
    member.enable_communication(ctx.http()).await?;

    // log to DB
    let pool = &ctx.data().database.pool;
    crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Unmute,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔊 Member Unmuted")
                .description(format!("**{}** has been unmuted.", user.name))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x57F287),
        ),
    )
    .await?;
    Ok(())
}

// ── Tactical Commands ────────────────────────────────────────────────────────

/// advanced tactical moderation operations
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only,
    subcommands("report", "intercept", "restore", "breach")
)]
pub async fn tactical(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// generate a full moderation history report for a user
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn report(
    ctx: Context<'_>,
    #[description = "The user to pull the report for"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let pool = &ctx.data().database.pool;
    let guild_id_i64 = guild_id.get() as i64;

    let cases =
        crate::db::mod_cases::get_cases_for_user(pool, guild_id_i64, user.id.get() as i64).await?;

    if cases.is_empty() {
        ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title(format!("📋 Tactical Report — {}", user.name))
                    .description("No moderation history found for this user.")
                    .color(0x2C2F33),
            ),
        )
        .await?;
        return Ok(());
    }

    // tally action counts
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for c in &cases {
        *counts.entry(c.action.as_str()).or_insert(0) += 1;
    }

    // build case log — last 10 entries to avoid embed overflow
    let recent: Vec<_> = cases.iter().rev().take(10).collect();
    let case_log = recent
        .iter()
        .map(|c| {
            let ts = c.created_at.timestamp();
            format!(
                "`#{}` **{}** — <t:{}:R>{}",
                c.case_number,
                c.action,
                ts,
                c.reason
                    .as_deref()
                    .map(|r| format!("\n> {}", r))
                    .unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let summary = counts
        .iter()
        .map(|(k, v)| format!("`{}` ×{}", k, v))
        .collect::<Vec<_>>()
        .join("  ");

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("📋 Tactical Report — {}", user.name))
                .description(format!(
                    "**{}** total case(s) on record.\n\n{}",
                    cases.len(),
                    summary
                ))
                .thumbnail(user.face())
                .field(
                    format!("Recent Cases (showing {}/{})", recent.len(), cases.len()),
                    case_log,
                    false,
                )
                .field("🆔 User ID", format!("`{}`", user.id), true)
                .field(
                    "📅 First Case",
                    format!("<t:{}:D>", cases.first().unwrap().created_at.timestamp()),
                    true,
                )
                .field(
                    "📅 Latest Case",
                    format!("<t:{}:D>", cases.last().unwrap().created_at.timestamp()),
                    true,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "AegisForge v4.2 Report | Moderator eyes only",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x2C2F33),
        ),
    )
    .await?;
    Ok(())
}

/// lock all text channels across the entire server simultaneously
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn intercept(
    ctx: Context<'_>,
    #[description = "Reason for the server-wide lockdown"] reason: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason
        .as_deref()
        .unwrap_or("Tactical intercept ordered by staff");

    let channels = guild_id.channels(ctx.http()).await?;
    let text_channels: Vec<_> = channels
        .values()
        .filter(|c| c.kind == ChannelType::Text)
        .collect();

    let total = text_channels.len();
    let everyone = guild_id.everyone_role();
    let overwrite = serenity::PermissionOverwrite {
        allow: serenity::Permissions::empty(),
        deny: serenity::Permissions::SEND_MESSAGES,
        kind: serenity::PermissionOverwriteType::Role(everyone),
    };

    // acknowledge immediately — the channel loop takes time
    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔒 Lockdown — Initiating")
                .description(format!(
                    "Locking **{}** channel(s) server-wide. Stand by.",
                    total
                ))
                .color(0xFF4500),
        ),
    )
    .await?;

    let mut locked = 0usize;
    for channel in &text_channels {
        if channel
            .create_permission(ctx.http(), overwrite.clone())
            .await
            .is_ok()
        {
            locked += 1;
        }
    }

    // log to mod channel
    let config = ctx
        .data()
        .database
        .get_guild_config(guild_id.get() as i64)
        .await?;
    if let Some(log_channel) = config.mod_log_channel {
        let embed = CreateEmbed::new()
            .title("🔒 Lockdown Deployed")
            .description(format!(
                "Server-wide lockdown initiated by <@{}>.",
                ctx.author().id
            ))
            .field("Channels Locked", format!("`{}/{}`", locked, total), true)
            .field("📝 Reason", reason_str, false)
            .footer(serenity::CreateEmbedFooter::new(
                "Use /unlock on each channel or /tactical restore to lift",
            ))
            .timestamp(serenity::Timestamp::now())
            .color(0xFF4500);

        let _ = serenity::ChannelId::new(log_channel as u64)
            .send_message(
                ctx.http(),
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔒 Lockdown — Complete")
                .description(format!(
                    "**{}/{}** channels locked.\n📝 Reason: {}",
                    locked, total, reason_str
                ))
                .footer(serenity::CreateEmbedFooter::new(
                    "Use /unlock per channel to restore, or /tactical restore for full lift",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}

/// lift a server-wide intercept — removes the @everyone SEND_MESSAGES deny from all channels
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_CHANNELS",
    guild_only
)]
pub async fn restore(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let channels = guild_id.channels(ctx.http()).await?;
    let text_channels: Vec<_> = channels
        .values()
        .filter(|c| c.kind == ChannelType::Text)
        .collect();

    let total = text_channels.len();
    let everyone = guild_id.everyone_role();

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔓 Tactical Restore — Initiating")
                .description(format!("Unlocking **{}** channel(s). Stand by.", total))
                .color(0x57F287),
        ),
    )
    .await?;

    let mut restored = 0usize;
    for channel in &text_channels {
        if channel
            .delete_permission(
                ctx.http(),
                serenity::PermissionOverwriteType::Role(everyone),
            )
            .await
            .is_ok()
        {
            restored += 1;
        }
    }

    // log to mod channel
    let config = ctx
        .data()
        .database
        .get_guild_config(guild_id.get() as i64)
        .await?;
    if let Some(log_channel) = config.mod_log_channel {
        let embed = CreateEmbed::new()
            .title("🔓 Tactical Restore Complete")
            .description(format!(
                "Server-wide lockdown lifted by <@{}>.",
                ctx.author().id
            ))
            .field(
                "Channels Restored",
                format!("`{}/{}`", restored, total),
                true,
            )
            .timestamp(serenity::Timestamp::now())
            .color(0x57F287);

        let _ = serenity::ChannelId::new(log_channel as u64)
            .send_message(
                ctx.http(),
                serenity::builder::CreateMessage::new().embed(embed),
            )
            .await;
    }

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🔓 Tactical Restore — Complete")
                .description(format!(
                    "**{}/{}** channels unlocked. The server is open again.",
                    restored, total
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0x57F287),
        ),
    )
    .await?;
    Ok(())
}

/// kick a user and purge their recent messages from the current channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "KICK_MEMBERS",
    guild_only
)]
pub async fn breach(
    ctx: Context<'_>,
    #[description = "The user to breach"] user: serenity::User,
    #[description = "Messages to purge from this channel (1-50, default 25)"] purge_count: Option<
        u8,
    >,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let reason_str = reason.as_deref().unwrap_or("Tactical breach");
    let limit = purge_count.unwrap_or(25).clamp(1, 50);
    let pool = &ctx.data().database.pool;

    // fetch recent messages and filter to target user
    let messages = ctx
        .channel_id()
        .messages(ctx.http(), serenity::GetMessages::default().limit(100))
        .await
        .map_err(|e| e.to_string())?;

    let target_msgs: Vec<serenity::MessageId> = messages
        .iter()
        .filter(|m| m.author.id == user.id)
        .take(limit as usize)
        .map(|m| m.id)
        .collect();

    let purged = target_msgs.len();
    if !target_msgs.is_empty() {
        let _ = ctx
            .channel_id()
            .delete_messages(ctx.http(), &target_msgs)
            .await;
    }

    // kick
    guild_id
        .kick_with_reason(ctx.http(), user.id, reason_str)
        .await?;

    // log to mod_cases
    let case = crate::db::mod_cases::create_case(
        pool,
        NewModCase {
            guild_id: guild_id.get() as i64,
            target_id: user.id.get() as i64,
            moderator_id: ctx.author().id.get() as i64,
            action: crate::models::mod_case::ModAction::Kick,
            reason: Some(reason_str),
            duration_secs: None,
            expires_at: None,
        },
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("⚡ Moderation Action")
                .description(format!(
                    "**{}** has been kicked and their message trail purged.",
                    user.name
                ))
                .field("👤 Target", format!("<@{}>", user.id), true)
                .field("🗑️ Messages Purged", format!("`{}`", purged), true)
                .field("🆔 Case", format!("#{}", case.case_number), true)
                .field("📝 Reason", reason_str, false)
                .footer(serenity::CreateEmbedFooter::new(
                    "Moderation Action Logged | AegisForge v4.2",
                ))
                .timestamp(serenity::Timestamp::now())
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}

/// view the moderation history for a user
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
pub async fn cases(
    ctx: Context<'_>,
    #[description = "The user to check"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await.map_err(|e| e.to_string())?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let pool = &ctx.data().database.pool;

    let cases = crate::db::mod_cases::get_cases_for_user(pool, guild_id, user.id.get() as i64)
        .await
        .map_err(|e| e.to_string())?;

    if cases.is_empty() {
        ctx.say(format!(
            "**{}** has a clean record. No cases found.",
            user.name
        ))
        .await
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let mut description = String::new();
    for case in cases.iter().rev().take(10) {
        description.push_str(&format!(
            "**Case #{}** — `{}`\nModerator: <@{}>\nReason: {}\nDate: <t:{}:d>\n\n",
            case.case_number,
            case.action,
            case.moderator_id,
            case.reason.as_deref().unwrap_or("No reason provided"),
            case.created_at.timestamp()
        ));
    }

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("📜 Mod History — {}", user.name))
                .description(description)
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Showing latest {}/{} cases",
                    cases.len().min(10),
                    cases.len()
                )))
                .color(0x00E5FF),
        ),
    )
    .await?;

    Ok(())
}

/// set a global slowmode for all text channels
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn slowmode_global(
    ctx: Context<'_>,
    #[description = "Seconds of slowmode (0 to disable)"] seconds: u64,
) -> Result<(), Error> {
    ctx.defer().await.map_err(|e| e.to_string())?;
    let guild_id = ctx.guild_id().unwrap();
    let channels = guild_id
        .channels(ctx.http())
        .await
        .map_err(|e| e.to_string())?;

    let mut count = 0;
    for channel in channels.values() {
        if channel.kind == serenity::ChannelType::Text
            && channel
                .id
                .edit(
                    ctx.http(),
                    serenity::EditChannel::new().rate_limit_per_user(seconds as u16),
                )
                .await
                .is_ok()
        {
            count += 1;
        }
    }

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🐌 Global Slowmode Deployed")
                .description(format!(
                    "Slowmode of **{}s** applied to **{}** text channels.",
                    seconds, count
                ))
                .color(0xFFAA00)
                .footer(serenity::CreateEmbedFooter::new(
                    "AegisForge v4.2 | Crowd Control",
                )),
        ),
    )
    .await?;

    Ok(())
}

/// view active warnings for a user
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
pub async fn warns(
    ctx: Context<'_>,
    #[description = "The user to check"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let pool = &ctx.data().database.pool;

    let warns =
        crate::db::mod_cases::get_warns_for_user(pool, guild_id, user.id.get() as i64).await?;

    if warns.is_empty() {
        ctx.say(format!("**{}** has no active warnings.", user.name))
            .await?;
        return Ok(());
    }

    let mut description = String::new();
    for warn in warns.iter().rev() {
        description.push_str(&format!(
            "**Case #{}** — Moderator: <@{}>\nReason: {}\nDate: <t:{}:d>\n\n",
            warn.case_number,
            warn.moderator_id,
            warn.reason.as_deref().unwrap_or("No reason provided"),
            warn.created_at.timestamp()
        ));
    }

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("⚠️ Active Warnings — {}", user.name))
                .description(description)
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Total Active Warnings: {}",
                    warns.len()
                )))
                .color(0xFEE75C),
        ),
    )
    .await?;

    Ok(())
}

/// clear all active warnings for a user
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
pub async fn clearwarns(
    ctx: Context<'_>,
    #[description = "The user to clear warnings for"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await.map_err(|e| e.to_string())?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let pool = &ctx.data().database.pool;

    let affected = crate::db::mod_cases::clear_warns_for_user(pool, guild_id, user.id.get() as i64)
        .await
        .map_err(|e| e.to_string())?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🧹 Warnings Cleared")
                .description(format!(
                    "Successfully cleared **{}** active warning(s) for **{}**.",
                    affected, user.name
                ))
                .color(0x57F287)
                .footer(serenity::CreateEmbedFooter::new("AegisForge v4.2")),
        ),
    )
    .await?;

    Ok(())
}
