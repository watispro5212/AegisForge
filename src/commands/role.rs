use crate::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, EditRole, Role, User};
use std::cmp::Reverse;

/// role management
#[poise::command(
    slash_command,
    subcommands("add", "remove", "list", "create", "delete"),
    category = "Roles",
    guild_only
)]
pub async fn role(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// add a role to a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The member to add the role to"] user: User,
    #[description = "The role to add"] role: Role,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.add_role(ctx.http(), role.id).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎭 Role Added")
                .description(format!("Successfully assigned **{}** to <@{}>.", role.name, user.id))
                .footer(CreateEmbedFooter::new("AegisForge v4.2 | Role Management"))
                .color(0x00FF88),
        ),
    )
    .await?;
    Ok(())
}

/// remove a role from a member
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_ROLES",
    guild_only
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The member to remove the role from"] user: User,
    #[description = "The role to remove"] role: Role,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let member = guild_id.member(ctx.http(), user.id).await?;
    member.remove_role(ctx.http(), role.id).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎭 Role Removed")
                .description(format!("Successfully removed **{}** from <@{}>.", role.name, user.id))
                .footer(CreateEmbedFooter::new("AegisForge v4.2 | Role Management"))
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}

/// list all roles in the server
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();
    let mut roles: Vec<_> = guild.roles.values().collect();
    roles.sort_by_key(|role| Reverse(role.position));

    let role_list: String = roles
        .iter()
        .take(20)
        .map(|r| format!("• <@&{}> — `{}`", r.id, r.id))
        .collect::<Vec<_>>()
        .join("\n");

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("🎭 Roles in {}", guild.name))
                .description(format!("Total Roles: **{}**\n\n{}", roles.len(), role_list))
                .footer(CreateEmbedFooter::new("Showing top 20 roles by position"))
                .color(0x00E5FF),
        ),
    )
    .await?;
    Ok(())
}

/// create a new role
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES", guild_only)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Name of the role"] name: String,
    #[description = "Hex color (e.g. #00E5FF)"] color: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let color_val = color
        .and_then(|c| u32::from_str_radix(c.trim_start_matches('#'), 16).ok())
        .unwrap_or(0);

    let role = guild_id.create_role(ctx.http(), EditRole::new().name(name).colour(color_val)).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎭 Role Created")
                .description(format!("Successfully created role <@&{}>.", role.id))
                .color(0x00FF88),
        ),
    )
    .await?;
    Ok(())
}

/// delete a role
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES", guild_only)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The role to delete"] role: Role,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    guild_id.delete_role(ctx.http(), role.id).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎭 Role Deleted")
                .description(format!("Successfully deleted role **{}**.", role.name))
                .color(0xFF4500),
        ),
    )
    .await?;
    Ok(())
}
