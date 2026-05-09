use crate::{Context, Error};
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

/// view all current server configurations
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    guild_only
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let config = ctx.data().database.get_guild_config(guild_id).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("⚙️ {} — Configuration", ctx.guild().unwrap().name))
                .field("Prefix", format!("`{}`", config.prefix), true)
                .field(
                    "Mod Logs",
                    if config.mod_log_channel.is_some() {
                        format!("<#{}>", config.mod_log_channel.unwrap())
                    } else {
                        "_Not Set_".to_string()
                    },
                    true,
                )
                .field(
                    "Auto-Role",
                    if config.auto_role_id.is_some() {
                        format!("<@&{}>", config.auto_role_id.unwrap())
                    } else {
                        "_Not Set_".to_string()
                    },
                    true,
                )
                .field(
                    "Welcome Channel",
                    if config.welcome_channel.is_some() {
                        format!("<#{}>", config.welcome_channel.unwrap())
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
