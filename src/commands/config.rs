use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Set the moderation log channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn logs(
    ctx: Context<'_>,
    #[description = "The channel to send mod logs to"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    // TODO: persist to database
    ctx.say(format!("✅ Mod log channel set to <#{}>.", channel_id)).await?;
    Ok(())
}

/// Set the welcome message channel
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn welcome(
    ctx: Context<'_>,
    #[description = "The channel for welcome messages"] channel: serenity::Channel,
    #[description = "Custom welcome message (use {user} as placeholder)"] message: Option<String>,
) -> Result<(), Error> {
    let channel_id = channel.id();
    let msg = message.unwrap_or_else(|| "Welcome, {user}! 🎉".to_string());
    // TODO: persist to database
    ctx.say(format!("✅ Welcome channel set to <#{}>.\nMessage: `{}`", channel_id, msg)).await?;
    Ok(())
}

/// Set the auto-role assigned to new members
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn autorole(
    ctx: Context<'_>,
    #[description = "The role to auto-assign to new members"] role: serenity::Role,
) -> Result<(), Error> {
    // TODO: persist to database
    ctx.say(format!("✅ Auto-role set to **{}**.", role.name)).await?;
    Ok(())
}
