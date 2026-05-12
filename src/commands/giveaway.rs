use crate::{Context, Error};
use dashmap::DashMap;
use poise::serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedFooter, Timestamp};
use rand::seq::SliceRandom;
use std::sync::OnceLock;

// ── In-memory giveaway store (DashMap — no extra dependencies) ───────────────
#[derive(Debug, Clone)]
pub struct ActiveGiveaway {
    pub channel_id: u64,
    pub message_id: u64,
    pub guild_id: u64,
    pub host_id: u64,
    pub prize: String,
    pub winners: u32,
    pub ends_at: i64,
    pub ended: bool,
}

static GIVEAWAYS: OnceLock<DashMap<String, ActiveGiveaway>> = OnceLock::new();

fn store() -> &'static DashMap<String, ActiveGiveaway> {
    GIVEAWAYS.get_or_init(DashMap::new)
}

/// giveaway management
#[poise::command(
    slash_command,
    subcommands("start", "end", "reroll", "list"),
    category = "Giveaways",
    guild_only
)]
pub async fn giveaway(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// start a giveaway in this channel
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Prize to give away"] prize: String,
    #[description = "Duration in minutes"] duration_minutes: u32,
    #[description = "Number of winners (1-10)"] winners: Option<u32>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get();
    let channel_id = ctx.channel_id().get();
    let winners = winners.unwrap_or(1).clamp(1, 10);
    let ends_at = chrono::Utc::now().timestamp() + (duration_minutes as i64 * 60);
    let winners_label = if winners == 1 { "winner" } else { "winners" };

    let embed = CreateEmbed::new()
        .title(format!("🎉 GIVEAWAY — {}", prize))
        .description(format!(
            "React with 🎉 to enter!\n\n\
            **Prize:** {}\n\
            **Winners:** {} {}\n\
            **Hosted by:** <@{}>\n\
            **Ends:** <t:{}:R> (<t:{}:F>)",
            prize, winners, winners_label, ctx.author().id, ends_at, ends_at
        ))
        .color(0xFFD700)
        .footer(CreateEmbedFooter::new(format!(
            "{} {} • AegisForge Giveaways",
            winners, winners_label
        )))
        .timestamp(
            Timestamp::from_unix_timestamp(ends_at).unwrap_or_else(|_| Timestamp::now()),
        );

    let reply = ctx
        .send(poise::CreateReply::default().embed(embed))
        .await?;
    let msg = reply.into_message().await?;
    msg.react(
        ctx.http(),
        serenity::ReactionType::Unicode("🎉".to_string()),
    )
    .await?;

    let key = format!("{}-{}", guild_id, msg.id.get());
    store().insert(
        key,
        ActiveGiveaway {
            channel_id,
            message_id: msg.id.get(),
            guild_id,
            host_id: ctx.author().id.get(),
            prize,
            winners,
            ends_at,
            ended: false,
        },
    );

    Ok(())
}

/// end a giveaway early and pick winners
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Message ID of the giveaway to end"] message_id: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get();
    let key = format!("{}-{}", guild_id, message_id);

    let ga = store()
        .get(&key)
        .map(|r| r.value().clone())
        .ok_or("Giveaway not found. Check the message ID.")?;

    if ga.ended {
        return Err("This giveaway has already ended.".into());
    }

    let msg_id = serenity::MessageId::new(ga.message_id);
    let chan_id = serenity::ChannelId::new(ga.channel_id);

    let reaction_users = chan_id
        .reaction_users(
            ctx.http(),
            msg_id,
            serenity::ReactionType::Unicode("🎉".to_string()),
            Some(100),
            None,
        )
        .await
        .unwrap_or_default();

    let mut eligible: Vec<_> = reaction_users.iter().filter(|u| !u.bot).collect();
    eligible.shuffle(&mut rand::thread_rng());

    let winner_count = ga.winners as usize;
    let picked: Vec<_> = eligible.iter().take(winner_count).collect();

    let (desc, color) = if picked.is_empty() {
        (
            format!("No valid entries for **{}**.", ga.prize),
            0xFF3B3B,
        )
    } else {
        let mentions = picked
            .iter()
            .map(|u| format!("<@{}>", u.id))
            .collect::<Vec<_>>()
            .join(", ");
        (
            format!(
                "**Prize:** {}\n**Winner(s):** {}\n\nCongratulations! 🎉",
                ga.prize, mentions
            ),
            0x00FF88,
        )
    };

    if let Some(mut g) = store().get_mut(&key) {
        g.ended = true;
    }

    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title("🎊 Giveaway Ended!")
            .description(desc)
            .color(color)
            .footer(CreateEmbedFooter::new("AegisForge Giveaways"))
            .timestamp(Timestamp::now()),
    ))
    .await?;

    Ok(())
}

/// reroll a giveaway to pick a new winner
#[poise::command(slash_command, required_permissions = "MANAGE_GUILD", guild_only)]
pub async fn reroll(
    ctx: Context<'_>,
    #[description = "Message ID of the ended giveaway"] message_id: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get();
    let key = format!("{}-{}", guild_id, message_id);

    let ga = store()
        .get(&key)
        .map(|r| r.value().clone())
        .ok_or("Giveaway not found.")?;

    let msg_id = serenity::MessageId::new(ga.message_id);
    let chan_id = serenity::ChannelId::new(ga.channel_id);

    let reaction_users = chan_id
        .reaction_users(
            ctx.http(),
            msg_id,
            serenity::ReactionType::Unicode("🎉".to_string()),
            Some(100),
            None,
        )
        .await
        .unwrap_or_default();

    let mut eligible: Vec<_> = reaction_users.iter().filter(|u| !u.bot).collect();
    eligible.shuffle(&mut rand::thread_rng());

    let desc = if let Some(winner) = eligible.first() {
        format!(
            "🔄 **Rerolled!**\nNew winner for **{}**: <@{}>",
            ga.prize, winner.id
        )
    } else {
        "No eligible participants to reroll from.".into()
    };

    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title("🎰 Giveaway Rerolled")
            .description(desc)
            .color(0x00E5FF)
            .footer(CreateEmbedFooter::new("AegisForge Giveaways"))
            .timestamp(Timestamp::now()),
    ))
    .await?;

    Ok(())
}

/// list active giveaways in this server
#[poise::command(slash_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = ctx.guild_id().unwrap().get();

    let active_list: Vec<_> = store()
        .iter()
        .filter(|r| r.guild_id == guild_id && !r.ended)
        .map(|r| r.value().clone())
        .collect();

    if active_list.is_empty() {
        ctx.send(poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title("🎉 Active Giveaways")
                .description("No active giveaways in this server.")
                .color(0x00E5FF),
        ))
        .await?;
        return Ok(());
    }

    let lines: String = active_list
        .iter()
        .map(|g| {
            format!(
                "• **{}** — {} winner(s) — ends <t:{}:R>\n  └ msg ID: `{}`\n",
                g.prize, g.winners, g.ends_at, g.message_id
            )
        })
        .collect();

    ctx.send(poise::CreateReply::default().embed(
        CreateEmbed::new()
            .title(format!("🎉 Active Giveaways — {}", active_list.len()))
            .description(lines)
            .color(0xFFD700)
            .footer(CreateEmbedFooter::new(
                "Use /giveaway end <message_id> to end early",
            ))
            .timestamp(Timestamp::now()),
    ))
    .await?;

    Ok(())
}
