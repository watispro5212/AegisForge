use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Set the moderation log channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn logs(
    ctx: Context<'_>,
    #[description = "The channel to send mod logs to"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    ctx.say(format!("✅ Mod log channel set to <#{}>.", channel.id)).await?;
    Ok(())
}

/// Set the welcome message channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn welcome(
    ctx: Context<'_>,
    #[description = "The channel for welcome messages"] channel: serenity::GuildChannel,
    #[description = "Custom welcome message (use {user} as placeholder)"] message: Option<String>,
) -> Result<(), Error> {
    let msg = message.unwrap_or_else(|| "Welcome, {user}! 🎉".to_string());
    ctx.say(format!("✅ Welcome channel set to <#{}>.\nMessage: `{}`", channel.id, msg)).await?;
    Ok(())
}

/// Set the auto-role assigned to new members
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn autorole(
    ctx: Context<'_>,
    #[description = "The role to auto-assign to new members"] role: serenity::Role,
) -> Result<(), Error> {
    crate::db::guild::set_auto_role(&ctx.data().database.pool, ctx.guild_id().unwrap().get() as i64, role.id.get() as i64).await?;
    ctx.data().database.invalidate_cache(ctx.guild_id().unwrap().get() as i64);
    ctx.say(format!("✅ Auto-role set to **{}**.", role.name)).await?;
    Ok(())
}

/// Set the bot's command prefix
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
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
        },
        None => {
            let config = ctx.data().database.get_guild_config(guild_id).await?;
            ctx.say(format!("The current prefix is: `{}`", config.prefix)).await?;
        }
    }
    Ok(())
}
