use crate::{models::sentinel::SentinelConfig, Context, Error};
use poise::serenity_prelude as serenity;

/// set the moderation log channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn logs(
    ctx: Context<'_>,
    #[description = "The channel to send mod logs to"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_mod_log_channel(
        &ctx.data().database.pool,
        guild_id,
        channel.id.get() as i64,
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!("✅ Mod log channel set to <#{}>.", channel.id))
        .await?;
    Ok(())
}

/// set the welcome message channel
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn welcome(
    ctx: Context<'_>,
    #[description = "The channel for welcome messages"] channel: serenity::GuildChannel,
    #[description = "Custom welcome message (use {user} as placeholder)"] message: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let msg = message.unwrap_or_else(|| "Welcome, {user}! 🎉".to_string());
    crate::db::guild::set_welcome_channel(
        &ctx.data().database.pool,
        guild_id,
        channel.id.get() as i64,
        &msg,
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Welcome channel set to <#{}>.\nMessage: `{}`",
        channel.id, msg
    ))
    .await?;
    Ok(())
}

/// set the auto-role assigned to new members
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn autorole(
    ctx: Context<'_>,
    #[description = "The role to auto-assign to new members"] role: serenity::Role,
) -> Result<(), Error> {
    crate::db::guild::set_auto_role(
        &ctx.data().database.pool,
        ctx.guild_id().unwrap().get() as i64,
        role.id.get() as i64,
    )
    .await?;
    ctx.data()
        .database
        .invalidate_cache(ctx.guild_id().unwrap().get() as i64);
    ctx.say(format!("✅ Auto-role set to **{}**.", role.name))
        .await?;
    Ok(())
}

/// set the bot's command prefix
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn prefix(
    ctx: Context<'_>,
    #[description = "The new prefix (e.g. !, ?, . )"] new_prefix: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    match new_prefix {
        Some(p) => {
            if p.len() > 5 {
                ctx.say("❌ Prefix must be 5 characters or less.").await?;
                return Ok(());
            }
            crate::db::guild::set_prefix(&ctx.data().database.pool, guild_id, &p).await?;
            ctx.data().database.invalidate_cache(guild_id);
            ctx.say(format!("✅ Prefix updated to: `{}`", p)).await?;
        }
        None => {
            let config = ctx.data().database.get_guild_config(guild_id).await?;
            ctx.say(format!("The current prefix is: `{}`", config.prefix))
                .await?;
        }
    }
    Ok(())
}

/// set the role assigned to muted/shadow-banned members
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn muterole(
    ctx: Context<'_>,
    #[description = "The role to apply when a member is muted or shadow banned"]
    role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_mute_role(&ctx.data().database.pool, guild_id, role.id.get() as i64)
        .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Mute role set to **{}**. This role will be applied on `/mute` and `/shadowban`.",
        role.name
    ))
    .await?;
    Ok(())
}

/// view all current server configurations
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let config = ctx.data().database.get_guild_config(guild_id).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("⚙️ {} — Configuration", ctx.guild().unwrap().name))
                .field("Prefix", format!("`{}`", config.prefix), true)
                .field(
                    "Mod Logs",
                    if let Some(channel_id) = config.mod_log_channel {
                        format!("<#{}>", channel_id)
                    } else {
                        "_Not Set_".to_string()
                    },
                    true,
                )
                .field(
                    "Auto-Role",
                    if let Some(role_id) = config.auto_role_id {
                        format!("<@&{}>", role_id)
                    } else {
                        "_Not Set_".to_string()
                    },
                    true,
                )
                .field(
                    "Welcome Channel",
                    if let Some(channel_id) = config.welcome_channel {
                        format!("<#{}>", channel_id)
                    } else {
                        "_Not Set_".to_string()
                    },
                    true,
                )
                .field(
                    "Welcome Message",
                    format!("`{}`", config.welcome_message),
                    false,
                )
                .color(0x00E5FF)
                .footer(serenity::CreateEmbedFooter::new(
                    "AegisForge — Security & Automation",
                )),
        ),
    )
    .await?;
    Ok(())
}

/// manage the Sentinel anti-raid system
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only,
    subcommands(
        "sentinel_enable",
        "sentinel_disable",
        "sentinel_threshold",
        "sentinel_status"
    )
)]
pub async fn sentinel(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// enable Sentinel anti-raid detection for this server
#[poise::command(slash_command, prefix_command, rename = "enable", guild_only)]
pub async fn sentinel_enable(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get();
    ctx.data()
        .sentinel_settings
        .entry(guild_id)
        .and_modify(|s| s.enabled = true)
        .or_insert_with(|| SentinelConfig {
            enabled: true,
            ..Default::default()
        });

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🛡️ Sentinel Activated")
            .description("Anti-raid detection is now **enabled**.\nUsers will be auto-kicked if the join rate exceeds the configured threshold.")
            .field("Default Threshold", "`5 joins in 10s`", true)
            .field("Change With", "`/sentinel threshold`", true)
            .footer(serenity::CreateEmbedFooter::new("AegisForge Sentinel — Raid Protection"))
            .color(0x00FF88),
    ))
    .await?;
    Ok(())
}

/// disable Sentinel anti-raid detection for this server
#[poise::command(slash_command, prefix_command, rename = "disable", guild_only)]
pub async fn sentinel_disable(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get();
    ctx.data()
        .sentinel_settings
        .entry(guild_id)
        .and_modify(|s| s.enabled = false)
        .or_insert_with(|| SentinelConfig {
            enabled: false,
            ..Default::default()
        });

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔒 Sentinel Deactivated")
                .description("Anti-raid detection has been **disabled** for this server.")
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}

/// set the raid detection threshold
#[poise::command(slash_command, prefix_command, rename = "threshold", guild_only)]
pub async fn sentinel_threshold(
    ctx: Context<'_>,
    #[description = "Max joins allowed in the window before a raid is declared (default: 5)"]
    joins: u32,
    #[description = "Detection window in seconds (default: 10)"] window: u32,
) -> Result<(), Error> {
    if joins < 2 {
        ctx.say("❌ Threshold must be at least 2.").await?;
        return Ok(());
    }
    if !(5..=300).contains(&window) {
        ctx.say("❌ Window must be between 5 and 300 seconds.")
            .await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().unwrap().get();
    ctx.data()
        .sentinel_settings
        .entry(guild_id)
        .and_modify(|s| {
            s.threshold = joins as usize;
            s.window_secs = window as u64;
        })
        .or_insert_with(|| SentinelConfig {
            enabled: false,
            threshold: joins as usize,
            window_secs: window as u64,
        });

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("⚙️ Sentinel Threshold Updated")
                .field("Joins", format!("`{}`", joins), true)
                .field("Window", format!("`{}s`", window), true)
                .description(format!(
                    "Sentinel will now trigger if **{}** or more users join within **{}s**.",
                    joins, window
                ))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// show the current Sentinel configuration for this server
#[poise::command(slash_command, prefix_command, rename = "status", guild_only)]
pub async fn sentinel_status(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get();
    let config = ctx
        .data()
        .sentinel_settings
        .get(&guild_id)
        .map(|s| (s.enabled, s.threshold, s.window_secs))
        .unwrap_or((false, 5, 10));

    let (enabled, threshold, window_secs) = config;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🛡️ Sentinel — Status")
            .field(
                "State",
                if enabled { "`🟢 Active`" } else { "`🔴 Inactive`" },
                true,
            )
            .field("Threshold", format!("`{} joins`", threshold), true)
            .field("Window", format!("`{}s`", window_secs), true)
            .description("Configure with `/sentinel enable`, `/sentinel disable`, or `/sentinel threshold`.")
            .footer(serenity::CreateEmbedFooter::new(
                "AegisForge Sentinel — Raid Protection",
            ))
            .color(if enabled { 0x00FF88 } else { 0xFF4500 }),
    ))
    .await?;
    Ok(())
}

// ── AutoMod Command Group ────────────────────────────────────────────────────

/// configure the AutoMod system
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only,
    subcommands(
        "automod_enable",
        "automod_disable",
        "automod_status",
        "automod_spam",
        "automod_invites",
        "automod_caps",
        "automod_mentions",
        "automod_blacklist_add",
        "automod_blacklist_remove",
        "automod_blacklist_list"
    )
)]
pub async fn automod(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "enable", guild_only)]
pub async fn automod_enable(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_enabled(&ctx.data().database.pool, guild_id, true).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🛡️ AutoMod Enabled")
            .description("Automatic moderation is now **active**. Use `/automod status` to review modules.")
            .color(0x00FF88),
    )).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "disable", guild_only)]
pub async fn automod_disable(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_enabled(&ctx.data().database.pool, guild_id, false).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🔓 AutoMod Disabled")
                .description("Automatic moderation has been turned off.")
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "status", guild_only)]
pub async fn automod_status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let config = ctx.data().database.get_guild_config(guild_id).await?;
    let blacklist = crate::db::guild::list_blacklist_phrases(&ctx.data().database.pool, guild_id)
        .await
        .unwrap_or_default();

    let on = |b: bool| if b { "`🟢 On`" } else { "`🔴 Off`" };

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🛡️ AutoMod — Status")
                .field("AutoMod", on(config.automod_enabled), true)
                .field("Anti-Spam", on(config.automod_spam), true)
                .field("Anti-Invite", on(config.automod_invites), true)
                .field("Anti-Caps", on(config.automod_caps), true)
                .field("Anti-Mentions", on(config.automod_mentions), true)
                .field(
                    "Blacklist",
                    if blacklist.is_empty() {
                        "_Empty_".into()
                    } else {
                        format!("`{}`", blacklist.join("`, `"))
                    },
                    false,
                )
                .color(if config.automod_enabled {
                    0x00E5FF
                } else {
                    0x2C2F33
                }),
        ),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "spam", guild_only)]
pub async fn automod_spam(
    ctx: Context<'_>,
    #[description = "on or off"] state: String,
) -> Result<(), Error> {
    let val = state.trim().to_lowercase() == "on";
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_spam(&ctx.data().database.pool, guild_id, val).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Anti-spam is now **{}**.",
        if val { "on" } else { "off" }
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "invites", guild_only)]
pub async fn automod_invites(
    ctx: Context<'_>,
    #[description = "on or off"] state: String,
) -> Result<(), Error> {
    let val = state.trim().to_lowercase() == "on";
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_invites(&ctx.data().database.pool, guild_id, val).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Anti-invite is now **{}**.",
        if val { "on" } else { "off" }
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "caps", guild_only)]
pub async fn automod_caps(
    ctx: Context<'_>,
    #[description = "on or off"] state: String,
) -> Result<(), Error> {
    let val = state.trim().to_lowercase() == "on";
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_caps(&ctx.data().database.pool, guild_id, val).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Anti-caps is now **{}**.",
        if val { "on" } else { "off" }
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "mentions", guild_only)]
pub async fn automod_mentions(
    ctx: Context<'_>,
    #[description = "on or off"] state: String,
) -> Result<(), Error> {
    let val = state.trim().to_lowercase() == "on";
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_automod_mentions(&ctx.data().database.pool, guild_id, val).await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Anti-mention-spam is now **{}**.",
        if val { "on" } else { "off" }
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "blacklist-add", guild_only)]
pub async fn automod_blacklist_add(
    ctx: Context<'_>,
    #[description = "Phrase to block"] phrase: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::add_blacklist_phrase(&ctx.data().database.pool, guild_id, phrase.trim())
        .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!("✅ `{}` added to the blacklist.", phrase.trim()))
        .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "blacklist-remove", guild_only)]
pub async fn automod_blacklist_remove(
    ctx: Context<'_>,
    #[description = "Phrase to unblock"] phrase: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let removed = crate::db::guild::remove_blacklist_phrase(
        &ctx.data().database.pool,
        guild_id,
        phrase.trim(),
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    if removed {
        ctx.say(format!(
            "✅ `{}` removed from the blacklist.",
            phrase.trim()
        ))
        .await?;
    } else {
        ctx.say(format!("❌ `{}` was not on the blacklist.", phrase.trim()))
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "blacklist-list", guild_only)]
pub async fn automod_blacklist_list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let list = crate::db::guild::list_blacklist_phrases(&ctx.data().database.pool, guild_id)
        .await
        .unwrap_or_default();
    let body = if list.is_empty() {
        "_No phrases on the blacklist._".into()
    } else {
        list.iter()
            .map(|p| format!("`{}`", p))
            .collect::<Vec<_>>()
            .join("\n")
    };
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!(
                    "🚫 Blacklist ({} phrase{})",
                    list.len(),
                    if list.len() == 1 { "" } else { "s" }
                ))
                .description(body)
                .color(0x2C2F33),
        ),
    )
    .await?;
    Ok(())
}

// ── Logging Channel Commands ──────────────────────────────────────────────────

/// set the channel for message edit and delete logs
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn msglogs(
    ctx: Context<'_>,
    #[description = "Channel for message logs"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_message_log_channel(
        &ctx.data().database.pool,
        guild_id,
        channel.id.get() as i64,
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Message logs will be sent to <#{}>.",
        channel.id
    ))
    .await?;
    Ok(())
}

/// set the channel for member join and leave logs
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn memberlogs(
    ctx: Context<'_>,
    #[description = "Channel for member join/leave logs"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    crate::db::guild::set_member_log_channel(
        &ctx.data().database.pool,
        guild_id,
        channel.id.get() as i64,
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!("✅ Member logs will be sent to <#{}>.", channel.id))
        .await?;
    Ok(())
}

/// set the goodbye message channel and text
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn goodbye(
    ctx: Context<'_>,
    #[description = "Channel for goodbye messages"] channel: serenity::GuildChannel,
    #[description = "Message text (use {user} and {server} as placeholders)"] message: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let msg = message.unwrap_or_else(|| "Goodbye, {user}. We will miss you.".into());
    crate::db::guild::set_goodbye_channel(
        &ctx.data().database.pool,
        guild_id,
        channel.id.get() as i64,
        &msg,
    )
    .await?;
    ctx.data().database.invalidate_cache(guild_id);
    ctx.say(format!(
        "✅ Goodbye messages will be sent to <#{}>.\nMessage: `{}`",
        channel.id, msg
    ))
    .await?;
    Ok(())
}
