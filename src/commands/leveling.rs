use crate::db::leveling;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CosmeticSlot {
    Background,
    Badge,
    Frame,
}

#[derive(Clone, Copy)]
struct RankCardCosmetic {
    item_id: &'static str,
    label: &'static str,
    slot: CosmeticSlot,
    accent_color: &'static str,
    text_color: &'static str,
    display: &'static str,
}

const RANK_CARD_COSMETICS: &[RankCardCosmetic] = &[
    RankCardCosmetic {
        item_id: "copper_badge",
        label: "Copper Badge",
        slot: CosmeticSlot::Badge,
        accent_color: "#B87333",
        text_color: "#FFF4E6",
        display: "Copper Vanguard",
    },
    RankCardCosmetic {
        item_id: "silver_badge",
        label: "Silver Badge",
        slot: CosmeticSlot::Badge,
        accent_color: "#C0C7D1",
        text_color: "#101820",
        display: "Silver Sentinel",
    },
    RankCardCosmetic {
        item_id: "gold_badge",
        label: "Gold Badge",
        slot: CosmeticSlot::Badge,
        accent_color: "#FFD166",
        text_color: "#221A00",
        display: "Gold Standard",
    },
    RankCardCosmetic {
        item_id: "diamond_badge",
        label: "Diamond Badge",
        slot: CosmeticSlot::Badge,
        accent_color: "#7DE2FF",
        text_color: "#031A24",
        display: "Diamond Signal",
    },
    RankCardCosmetic {
        item_id: "forge_crown",
        label: "Forge Crown",
        slot: CosmeticSlot::Badge,
        accent_color: "#FFB703",
        text_color: "#1A1200",
        display: "Forge Crown",
    },
    RankCardCosmetic {
        item_id: "neon_nameplate",
        label: "Neon Nameplate",
        slot: CosmeticSlot::Background,
        accent_color: "#00E5FF",
        text_color: "#F8FEFF",
        display: "Neon Grid",
    },
    RankCardCosmetic {
        item_id: "carbon_nameplate",
        label: "Carbon Nameplate",
        slot: CosmeticSlot::Background,
        accent_color: "#8D99AE",
        text_color: "#EDF2F4",
        display: "Carbon Fiber",
    },
    RankCardCosmetic {
        item_id: "aurora_frame",
        label: "Aurora Frame",
        slot: CosmeticSlot::Frame,
        accent_color: "#B5179E",
        text_color: "#FFF6FF",
        display: "Aurora Frame",
    },
    RankCardCosmetic {
        item_id: "obsidian_frame",
        label: "Obsidian Frame",
        slot: CosmeticSlot::Frame,
        accent_color: "#2B2D42",
        text_color: "#F8F9FA",
        display: "Obsidian Frame",
    },
];

fn find_cosmetic(query: &str, slot: CosmeticSlot) -> Option<RankCardCosmetic> {
    let normalized = query.trim().to_lowercase().replace([' ', '-'], "_");
    RANK_CARD_COSMETICS.iter().copied().find(|item| {
        item.slot == slot
            && (item.item_id.eq_ignore_ascii_case(&normalized)
                || item.label.eq_ignore_ascii_case(query.trim())
                || item.label.to_lowercase().replace(' ', "_") == normalized)
    })
}

fn cosmetic_label(item_id: &str) -> String {
    if item_id == "none" || item_id == "default" {
        return "Default".to_string();
    }

    RANK_CARD_COSMETICS
        .iter()
        .find(|item| item.item_id == item_id)
        .map(|item| item.display.to_string())
        .unwrap_or_else(|| item_id.replace('_', " "))
}

fn parse_hex_color(input: &str) -> Option<u32> {
    let hex = input.trim().trim_start_matches('#');
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        None
    }
}

fn normalize_hex_color(input: &str) -> Option<String> {
    parse_hex_color(input)
        .map(|_| format!("#{}", input.trim().trim_start_matches('#').to_uppercase()))
}

fn progress_bar(percent: f64) -> String {
    let filled = (percent / 10.0).round().clamp(0.0, 10.0) as usize;
    format!("{}{}", "█".repeat(filled), "░".repeat(10 - filled))
}

fn next_level_xp(level: i32) -> i64 {
    ((level + 1) as f64 * 5.0).powi(2) as i64
}

fn current_level_xp(level: i32) -> i64 {
    (level as f64 * 5.0).powi(2) as i64
}

async fn ensure_owned_cosmetic(
    ctx: Context<'_>,
    guild_id: i64,
    user_id: i64,
    cosmetic: RankCardCosmetic,
) -> Result<(), Error> {
    let owns = leveling::user_owns_inventory_item(
        &ctx.data().database.pool,
        guild_id,
        user_id,
        cosmetic.item_id,
    )
    .await?;

    if owns {
        Ok(())
    } else {
        Err(format!(
            "You do not own **{}** yet. Buy it with `/economy buy {}` first.",
            cosmetic.label, cosmetic.item_id
        )
        .into())
    }
}

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

    let server_rank =
        leveling::get_user_rank(&ctx.data().database.pool, guild_id, target.id.get() as i64)
            .await?
            .map(|rank| format!("#{}", rank))
            .unwrap_or_else(|| "Unranked".to_string());

    let current_level_xp = current_level_xp(lvl.level);
    let next_level_xp = next_level_xp(lvl.level);
    let progress = (lvl.xp - current_level_xp).max(0);
    let total_needed = (next_level_xp - current_level_xp).max(1);
    let percent = (progress as f64 / total_needed as f64 * 100.0).clamp(0.0, 100.0);
    let accent = parse_hex_color(&lvl.rank_card_color).unwrap_or(0x00E5FF);
    let background = cosmetic_label(&lvl.rank_card_background);
    let badge = cosmetic_label(&lvl.rank_card_badge);
    let frame = cosmetic_label(&lvl.rank_card_frame);

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("📈 {}'s Rank", target.name))
                .description(format!(
                    "**Level {}** | **{} XP** | **{}**\n`{}` `{:.1}%`",
                    lvl.level,
                    lvl.xp,
                    server_rank,
                    progress_bar(percent),
                    percent
                ))
                .thumbnail(target.face())
                .field(
                    "Next Level",
                    format!("`{}/{}` XP", progress, total_needed),
                    true,
                )
                .field("Accent", format!("`{}`", lvl.rank_card_color), true)
                .field("Text", format!("`{}`", lvl.rank_card_text_color), true)
                .field(
                    "Equipped Cosmetics",
                    format!(
                        "Background: **{}**\nBadge: **{}**\nFrame: **{}**",
                        background, badge, frame
                    ),
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(
                    "Use /leveling customize to equip shop cosmetics or change colors.",
                ))
                .color(accent),
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
    #[description = "Owned background/nameplate item id or exact name"] background: Option<String>,
    #[description = "Owned badge item id or exact name"] badge: Option<String>,
    #[description = "Owned frame item id or exact name"] frame: Option<String>,
    #[description = "Hex color (e.g. #00E5FF)"] color: Option<String>,
    #[description = "Text color (e.g. #FFFFFF)"] text_color: Option<String>,
    #[description = "Reset all rank card cosmetics to default"] reset: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let user_id = ctx.author().id.get() as i64;
    let pool = &ctx.data().database.pool;

    leveling::get_user_leveling(pool, guild_id, user_id).await?;

    if reset.unwrap_or(false) {
        leveling::reset_rank_card_customization(pool, guild_id, user_id).await?;
        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title("Rank Card Reset")
                    .description("Your rank card is back to the default AegisForge style.")
                    .color(0x00E5FF),
            ),
        )
        .await?;
        return Ok(());
    }

    let mut applied = Vec::new();
    let mut preset_color = None;
    let mut preset_text_color = None;

    if let Some(value) = background.as_deref() {
        let cosmetic = find_cosmetic(value, CosmeticSlot::Background).ok_or_else(|| {
            "Unknown rank-card background. Try `neon_nameplate` or `carbon_nameplate`.".to_string()
        })?;
        ensure_owned_cosmetic(ctx, guild_id, user_id, cosmetic).await?;
        leveling::update_rank_card_customization(
            pool,
            guild_id,
            user_id,
            Some(cosmetic.item_id.to_string()),
            None,
            None,
        )
        .await?;
        preset_color = Some(cosmetic.accent_color.to_string());
        preset_text_color = Some(cosmetic.text_color.to_string());
        applied.push(format!("Background: **{}**", cosmetic.label));
    }

    if let Some(value) = badge.as_deref() {
        let cosmetic = find_cosmetic(value, CosmeticSlot::Badge).ok_or_else(|| {
            "Unknown rank-card badge. Try `copper_badge`, `gold_badge`, or `forge_crown`."
                .to_string()
        })?;
        ensure_owned_cosmetic(ctx, guild_id, user_id, cosmetic).await?;
        leveling::update_rank_card_badge(pool, guild_id, user_id, cosmetic.item_id).await?;
        preset_color = Some(cosmetic.accent_color.to_string());
        applied.push(format!("Badge: **{}**", cosmetic.label));
    }

    if let Some(value) = frame.as_deref() {
        let cosmetic = find_cosmetic(value, CosmeticSlot::Frame).ok_or_else(|| {
            "Unknown rank-card frame. Try `aurora_frame` or `obsidian_frame`.".to_string()
        })?;
        ensure_owned_cosmetic(ctx, guild_id, user_id, cosmetic).await?;
        leveling::update_rank_card_frame(pool, guild_id, user_id, cosmetic.item_id).await?;
        preset_color = Some(cosmetic.accent_color.to_string());
        preset_text_color = Some(cosmetic.text_color.to_string());
        applied.push(format!("Frame: **{}**", cosmetic.label));
    }

    let final_color = match color.as_deref() {
        Some(value) => Some(
            normalize_hex_color(value)
                .ok_or("Color must be a valid 6-digit hex value like `#00E5FF`.")?,
        ),
        None => preset_color,
    };

    let final_text_color = match text_color.as_deref() {
        Some(value) => Some(
            normalize_hex_color(value)
                .ok_or("Text color must be a valid 6-digit hex value like `#FFFFFF`.")?,
        ),
        None => preset_text_color,
    };

    if final_color.is_some() || final_text_color.is_some() {
        leveling::update_rank_card_customization(
            pool,
            guild_id,
            user_id,
            None,
            final_color.clone(),
            final_text_color.clone(),
        )
        .await?;
        if let Some(value) = &final_color {
            applied.push(format!("Accent: `{}`", value));
        }
        if let Some(value) = &final_text_color {
            applied.push(format!("Text: `{}`", value));
        }
    }

    if applied.is_empty() {
        let owned_ids = leveling::get_owned_rank_card_item_ids(pool, guild_id, user_id).await?;
        let owned: HashSet<&str> = owned_ids.iter().map(String::as_str).collect();
        let mut available = String::new();
        for slot in [
            CosmeticSlot::Background,
            CosmeticSlot::Badge,
            CosmeticSlot::Frame,
        ] {
            let title = match slot {
                CosmeticSlot::Background => "Backgrounds",
                CosmeticSlot::Badge => "Badges",
                CosmeticSlot::Frame => "Frames",
            };
            let lines = RANK_CARD_COSMETICS
                .iter()
                .filter(|item| item.slot == slot && owned.contains(item.item_id))
                .map(|item| format!("`{}` - {}", item.item_id, item.label))
                .collect::<Vec<_>>();
            if !lines.is_empty() {
                available.push_str(&format!("**{}**\n{}\n\n", title, lines.join("\n")));
            }
        }

        if available.is_empty() {
            available =
                "No rank-card cosmetics owned yet. Buy one from `/economy shop` first.".to_string();
        }

        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .title("Rank Card Customization")
                    .description(available.trim())
                    .field(
                        "Usage",
                        "`/leveling customize background:neon_nameplate badge:gold_badge frame:aurora_frame`",
                        false,
                    )
                    .color(0x00E5FF),
            ),
        )
        .await?;
        return Ok(());
    }

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("🎨 Rank Card Updated")
                .description(applied.join("\n"))
                .footer(serenity::CreateEmbedFooter::new(
                    "Run /leveling rank to preview your upgraded card.",
                ))
                .color(
                    final_color
                        .as_deref()
                        .and_then(parse_hex_color)
                        .unwrap_or(0x00FF88),
                ),
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

    sqlx::query(
        "UPDATE users_leveling SET xp = $1, level = $2 WHERE guild_id = $3 AND user_id = $4",
    )
    .bind(xp)
    .bind(level)
    .bind(guild_id)
    .bind(user.id.get() as i64)
    .execute(pool)
    .await?;

    ctx.say(format!(
        "✅ Set **{}**'s XP to `{}` (Level `{}`).",
        user.name, xp, level
    ))
    .await?;
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

    sqlx::query("DELETE FROM users_leveling WHERE guild_id = $1 AND user_id = $2")
        .bind(guild_id)
        .bind(user.id.get() as i64)
        .execute(pool)
        .await?;

    ctx.say(format!("✅ Reset all leveling data for **{}**.", user.name))
        .await?;
    Ok(())
}
