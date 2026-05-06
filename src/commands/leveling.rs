use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use crate::db::leveling;

/// Leveling commands
#[poise::command(
    slash_command,
    subcommands("rank", "leaderboard"),
    category = "Leveling"
)]
pub async fn leveling(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Check your or someone else's rank
#[poise::command(slash_command)]
pub async fn rank(
    ctx: Context<'_>,
    #[description = "User to check rank of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target = user.as_ref().unwrap_or(ctx.author());
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    
    let lvl = leveling::get_user_leveling(&ctx.data().database.pool, guild_id, target.id.get() as i64).await?;
    
    // Calculate progress to next level
    let current_level_xp = (lvl.level as f64 * 5.0).powi(2) as i64;
    let next_level_xp = ((lvl.level + 1) as f64 * 5.0).powi(2) as i64;
    let progress = lvl.xp - current_level_xp;
    let total_needed = next_level_xp - current_level_xp;
    let percent = (progress as f64 / total_needed as f64 * 100.0).max(0.0).min(100.0);

    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title(format!("📈 {}'s Rank", target.name))
            .field("Level", format!("`{}`", lvl.level), true)
            .field("XP", format!("`{}/{}`", lvl.xp, next_level_xp), true)
            .field("Progress", format!("`{:.1}%`", percent), true)
            .color(0x00E5FF)
        )).await?;
    
    Ok(())
}

/// View the most active users in the server
#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let lb = leveling::get_leaderboard(&ctx.data().database.pool, guild_id, 10).await?;
    
    let mut content = String::new();
    for (i, lvl) in lb.iter().enumerate() {
        content.push_str(&format!("{}. <@{}> — Level `{}` (`{} XP`)\n", i + 1, lvl.user_id, lvl.level, lvl.xp));
    }
    
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("🏆 Activity Leaderboard")
            .description(content)
            .color(0x00E5FF)
        )).await?;
    
    Ok(())
}
