use crate::db::leveling;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// leveling commands
#[poise::command(
    slash_command,
    subcommands("rank", "leaderboard", "customize", "set_xp", "reset_user"),
    category = "Leveling"
)]
pub async fn leveling(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// check your or someone else's rank
#[poise::command(slash_command, guild_only)]
pub async fn rank(
    ctx: Context<'_>,
    #[description = "User to check rank of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    let lvl =
        leveling::get_user_leveling(&ctx.data().database.pool, guild_id, target.id.get() as i64)
            .await?;

    // calculate progress to next level
    let current_level_xp = (lvl.level as f64 * 5.0).powi(2) as i64;
    let next_level_xp = ((lvl.level + 1) as f64 * 5.0).powi(2) as i64;
    let progress = lvl.xp - current_level_xp;
    let total_needed = next_level_xp - current_level_xp;
    let percent = (progress as f64 / total_needed as f64 * 100.0).clamp(0.0, 100.0);

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("📈 {}'s Rank", target.name))
                .thumbnail(target.face())
                .field("Level", format!("`{}`", lvl.level), true)
                .field("XP", format!("`{}/{}`", lvl.xp, next_level_xp), true)
                .field("Progress", format!("`{:.1}%`", percent), true)
                .field(
                    "Customization",
                    format!(
                        "Background: `{}`\nColor: `{}`",
                        lvl.rank_card_background, lvl.rank_card_color
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Use /leveling customize to change your rank card!",
                ))
                .color(
                    u32::from_str_radix(lvl.rank_card_color.trim_start_matches('#'), 16)
                        .unwrap_or(0x00E5FF),
                ),
        ),
    )
    .await?;

    Ok(())
}

/// view the most active users
#[poise::command(slash_command, guild_only)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Show global leaderboard across all servers"] global: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let is_global = global.unwrap_or(false);

    let mut content = String::new();
    let title = if is_global {
        let lb = leveling::get_global_leaderboard(&ctx.data().database.pool, 10).await?;
        for (i, entry) in lb.iter().enumerate() {
            content.push_str(&format!(
                "**{}**. <@{}> — `{} XP`\n",
                i + 1,
                entry.user_id,
                entry.total_xp
            ));
        }
        "🏆 Global Activity Leaderboard"
    } else {
        let guild_id = ctx.guild_id().unwrap().get() as i64;
        let lb = leveling::get_leaderboard(&ctx.data().database.pool, guild_id, 10).await?;
        for (i, lvl) in lb.iter().enumerate() {
            content.push_str(&format!(
                "**{}**. <@{}> — Level `{}` (`{} XP`)\n",
                i + 1,
                lvl.user_id,
                lvl.level,
                lvl.xp
            ));
        }
        "🏆 Server Activity Leaderboard"
    };

    if content.is_empty() {
        content = "_No data found yet._".to_string();
    }

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(title)
                .description(content)
                .footer(serenity::CreateEmbedFooter::new(if is_global {
                    "Top 10 Most Active Across All Realms"
                } else {
                    "Top 10 Most Active in This Server"
                }))
                .timestamp(serenity::Timestamp::now())
                .color(0x00E5FF),
        ),
    )
    .await?;

    Ok(())
}

/// customize your rank card
#[poise::command(slash_command, guild_only)]
pub async fn customize(
    ctx: Context<'_>,
    #[description = "Background name or URL"] background: Option<String>,
    #[description = "Hex color (e.g. #00E5FF)"] color: Option<String>,
    #[description = "Text color (e.g. #FFFFFF)"] text_color: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;

    leveling::update_rank_card_customization(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        background,
        color,
        text_color,
    )
    .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🎨 Rank Card Updated")
                .description("Your rank card preferences have been saved successfully!")
                .footer(serenity::CreateEmbedFooter::new(
                    "AegisForge v4.2 Customization",
                ))
                .color(0x00FF88),
        ),
    )
    .await?;

    Ok(())
}

/// set a user's XP (staff only)
#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_GUILD")]
pub async fn set_xp(
    ctx: Context<'_>,
    #[description = "The user to modify"] user: serenity::User,
    #[description = "The new XP amount"] xp: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let pool = &ctx.data().database.pool;
    
    // calculate level from XP (simplified: level = sqrt(xp) / 5)
    let level = ((xp as f64).sqrt() / 5.0).floor() as i32;
    
    sqlx::query!(
        "UPDATE users_leveling SET xp = $1, level = $2 WHERE guild_id = $3 AND user_id = $4",
        xp, level, guild_id, user.id.get() as i64
    )
    .execute(pool)
    .await?;

    ctx.say(format!("✅ Set **{}**'s XP to `{}` (Level `{}`).", user.name, xp, level)).await?;
    Ok(())
}

/// reset a user's leveling data (staff only)
#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_GUILD")]
pub async fn reset_user(
    ctx: Context<'_>,
    #[description = "The user to reset"] user: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let pool = &ctx.data().database.pool;
    
    sqlx::query!(
        "DELETE FROM users_leveling WHERE guild_id = $1 AND user_id = $2",
        guild_id, user.id.get() as i64
    )
    .execute(pool)
    .await?;

    ctx.say(format!("✅ Reset all leveling data for **{}**.", user.name)).await?;
    Ok(())
}
