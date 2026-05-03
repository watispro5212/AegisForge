use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Add a role to a member
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES", guild_only)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The member to add the role to"] user: serenity::User,
    #[description = "The role to add"] role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.add_role(ctx.http(), role.id).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("✅ Added **{}** to **{}**.", role.name, user.name))
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// Remove a role from a member
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES", guild_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The member to remove the role from"] user: serenity::User,
    #[description = "The role to remove"] role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.remove_role(ctx.http(), role.id).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("✅ Removed **{}** from **{}**.", role.name, user.name))
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}

/// List all roles in the server
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();
    let mut roles: Vec<_> = guild.roles.values().collect();
    roles.sort_by(|a, b| b.position.cmp(&a.position));

    let role_list: String = roles
        .iter()
        .take(25)
        .map(|r| format!("• {} `({})`", r.name, r.id))
        .collect::<Vec<_>>()
        .join("\n");

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title(format!("Roles in {}", guild.name))
            .description(role_list)
            .color(0x00ffff),
    ))
    .await?;
    Ok(())
}
