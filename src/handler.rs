use poise::serenity_prelude as serenity;
use std::time::{Duration, Instant};
use tracing::{info, warn};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, crate::Error>,
    data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            info!(
                "🔩 aegisforge online as {} ({})",
                data_about_bot.user.name, data_about_bot.user.id
            );

            let guild_count = ctx.cache.guild_count();
            set_presence(ctx, guild_count, 0);
<<<<<<< HEAD
=======

            // rotating presence — cycles every 30s through branded status messages
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                let mut idx: usize = 1;
                loop {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    let guilds = ctx_clone.cache.guild_count();
                    set_presence(&ctx_clone, guilds, idx);
                    idx = (idx + 1) % 6;
                }
            });

            // startup webhook notification — only on shard 0 to avoid spam lol
            if ctx.shard_id.0 == 0 {
                if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
                    let http = ctx.http.clone();
                    tokio::spawn(async move {
                        match serenity::model::webhook::Webhook::from_url(&http, &webhook_url).await
                        {
                            Ok(webhook) => {
                                let embed = serenity::builder::CreateEmbed::new()
                                    .title("✅ AegisForge Online")
                                    .description(format!(
                                        "Bot initialized successfully across **{}** guild(s).",
                                        guild_count
                                    ))
                                    .field(
                                        "Version",
                                        format!("`v{}`", env!("CARGO_PKG_VERSION")),
                                        true,
                                    )
                                    .field("Language", "`Rust`", true)
                                    .field("Status", "🟢 Operational", true)
                                    .footer(serenity::builder::CreateEmbedFooter::new(format!(
                                        "AegisForge v{}",
                                        env!("CARGO_PKG_VERSION")
                                    )))
                                    .timestamp(serenity::Timestamp::now())
                                    .color(0x57F287);
                                let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                                if let Err(e) = webhook.execute(&http, false, builder).await {
                                    error!("Failed to send startup webhook: {:?}", e);
                                }
                            }
                            Err(e) => error!("Failed to load status webhook: {:?}", e),
                        }
                    });
                }
            }
>>>>>>> 464415d48bbb577285feea95e643bf0a924170dd
        }

        serenity::FullEvent::GuildCreate { guild, is_new } => {
            let guild_count = ctx.cache.guild_count();
            set_presence(ctx, guild_count, 0);

            if let Some(true) = is_new {
                info!("📥 joined new server: {} ({})", guild.name, guild.id);
                if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
                    let http = ctx.http.clone();
                    let guild_name = guild.name.clone();
                    let member_count = guild.member_count;
                    let guild_id = guild.id;

                    tokio::spawn(async move {
                        match serenity::model::webhook::Webhook::from_url(&http, &webhook_url).await
                        {
                            Ok(webhook) => {
                                let embed = serenity::builder::CreateEmbed::new()
                                    .title("📥 New Server Joined")
                                    .description(format!("AegisForge was added to **{}**.", guild_name))
                                    .field("Members", format!("`{}`", member_count), true)
                                    .field("Total Servers", format!("`{}`", guild_count), true)
                                    .field("Server ID", format!("`{}`", guild_id), true)
                                    .footer(serenity::builder::CreateEmbedFooter::new(format!(
                                        "AegisForge v{} - Member Joined",
                                        env!("CARGO_PKG_VERSION")
                                    )))
                                    .timestamp(serenity::Timestamp::now())
                                    .color(0x57F287);
                                let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                                if let Err(e) = webhook.execute(&http, false, builder).await {
                                    error!("Failed to execute join webhook: {}", e);
                                }
                            }
                            Err(e) => error!("Failed to load join webhook: {}", e),
                        }
                    });
                }
            }
        }

        serenity::FullEvent::GuildDelete { incomplete, .. } => {
            let guild_count = ctx.cache.guild_count();
            set_presence(ctx, guild_count, 0);
            info!("📤 Left server: {}", incomplete.id);
        }

        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let guild_id = new_member.guild_id.get() as i64;
            let guild_id_u64 = new_member.guild_id.get();
            let db = &data.database;

            // ── Sentinel Anti-Raid ───────────────────────────────────
            {
                let (enabled, threshold, window_secs) = data
                    .sentinel_settings
                    .get(&guild_id_u64)
                    .map(|s| (s.enabled, s.threshold, s.window_secs))
                    .unwrap_or((false, 5, 10));

                if enabled {
                    let now = Instant::now();
                    let window = Duration::from_secs(window_secs);
                    let user_id = new_member.user.id.get();

                    let count = {
<<<<<<< HEAD
                        let mut entry = data.raid_tracker.entry(guild_id_u64).or_default();
=======
                        let mut entry = data
                            .raid_tracker
                            .entry(guild_id_u64)
                            .or_default();
>>>>>>> 464415d48bbb577285feea95e643bf0a924170dd
                        entry.retain(|(t, _)| now.duration_since(*t) < window);
                        entry.push_back((now, user_id));
                        entry.len()
                    };

                    if count >= threshold {
                        warn!(
                            "Sentinel: raid detected in guild {} — {} joins in {}s",
                            guild_id_u64, count, window_secs
                        );

                        let kick_result = ctx
                            .http
                            .kick_member(
                                new_member.guild_id,
                                new_member.user.id,
                                Some("Sentinel Anti-Raid: abnormal join rate detected"),
                            )
                            .await;

                        if let Ok(config) = db.get_guild_config(guild_id).await {
                            if let Some(log_channel) = config.mod_log_channel {
                                let action = if kick_result.is_ok() {
                                    "`Kicked`"
                                } else {
                                    "`Kick Failed — check bot permissions`"
                                };
                                let embed = serenity::builder::CreateEmbed::new()
                                    .title("🚨 Sentinel — Join Spike Detected")
                                    .description(format!(
                                        "**{}** users joined within **{}s**. <@{}> has been kicked.",
                                        count, window_secs, user_id
                                    ))
                                    .field("Action", action, true)
                                    .field(
                                        "User",
                                        format!("<@{}> (`{}`)", user_id, user_id),
                                        true,
                                    )
                                    .footer(serenity::builder::CreateEmbedFooter::new(
                                        "AegisForge Sentinel",
                                    ))
                                    .timestamp(serenity::Timestamp::now())
                                    .color(0xFF4500);

                                let _ = serenity::ChannelId::new(log_channel as u64)
                                    .send_message(
                                        &ctx.http,
                                        serenity::builder::CreateMessage::new().embed(embed),
                                    )
                                    .await;
                            }
                        }

                        return Ok(());
                    }
                }
            }

            if let Ok(config) = db.get_guild_config(guild_id).await {
                // Member log
                if let Some(lc) = config.member_log_channel {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("📥 Member Joined")
                        .description(format!("**{}** joined the server.", new_member.user.name))
                        .thumbnail(new_member.user.face())
                        .field("User ID", format!("`{}`", new_member.user.id), true)
                        .field(
                            "Account Age",
                            format!("<t:{}:R>", new_member.user.id.created_at().unix_timestamp()),
                            true,
                        )
                        .footer(serenity::builder::CreateEmbedFooter::new(
                            "AegisForge Member Log",
                        ))
                        .timestamp(serenity::Timestamp::now())
                        .color(0x57F287);
                    let _ = serenity::ChannelId::new(lc as u64)
                        .send_message(
                            &ctx.http,
                            serenity::builder::CreateMessage::new().embed(embed),
                        )
                        .await;
                }
                // auto-role
                if let Some(role_id) = config.auto_role_id {
                    let _ = new_member
                        .add_role(&ctx.http, serenity::RoleId::new(role_id as u64))
                        .await;
                }

                // welcome message
                if let Some(channel_id) = config.welcome_channel {
                    if !config.welcome_message.is_empty() {
                        let server_name = ctx
                            .cache
                            .guild(new_member.guild_id)
                            .map(|g| g.name.clone())
                            .unwrap_or_else(|| "the server".to_string());

                        let msg = config
                            .welcome_message
                            .replace("{user}", &format!("<@{}>", new_member.user.id))
                            .replace("{server}", &server_name);

                        let embed = serenity::builder::CreateEmbed::new()
                            .title(format!("👋 Welcome to {}!", server_name))
                            .description(&msg)
                            .thumbnail(new_member.user.face())
                            .footer(serenity::builder::CreateEmbedFooter::new(format!(
                                "Member #{}",
                                ctx.cache
                                    .guild(new_member.guild_id)
                                    .map(|g| g.member_count)
                                    .unwrap_or(0)
                            )))
                            .color(0x00E5FF);

                        let _ = serenity::ChannelId::new(channel_id as u64)
                            .send_message(
                                &ctx.http,
                                serenity::builder::CreateMessage::new().embed(embed),
                            )
                            .await;
                    }
                }
            }
        }

        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            let guild_id = match new_message.guild_id {
                Some(id) => id.get() as i64,
                None => return Ok(()),
            };

            let db = &data.database;
            let config = match db.get_guild_config(guild_id).await {
                Ok(c) => c,
                Err(_) => return Ok(()),
            };

            // ── Leveling ────────────────────────────────────────────
            if config.leveling_enabled {
                if let Ok(true) = crate::db::leveling::add_xp(
                    &db.pool,
                    guild_id,
                    new_message.author.id.get() as i64,
                    15,
                )
                .await
                {
                    if let Ok(user_lvl) = crate::db::leveling::get_user_leveling(
                        &db.pool,
                        guild_id,
                        new_message.author.id.get() as i64,
                    )
                    .await
                    {
                        let template = if config.level_up_message.is_empty() {
                            "Congratulations {user}, you reached **Level {level}**! Keep it up. 🚀"
                                .to_string()
                        } else {
                            config.level_up_message.clone()
                        };

                        let msg_text = template
                            .replace("{user}", &format!("<@{}>", new_message.author.id))
                            .replace("{level}", &user_lvl.level.to_string());

                        let embed = serenity::builder::CreateEmbed::new()
                            .title("⬆️ Level Up!")
                            .description(&msg_text)
                            .thumbnail(new_message.author.face())
                            .field("New Level", format!("`{}`", user_lvl.level), true)
                            .field("Total XP", format!("`{}`", user_lvl.xp), true)
                            .color(0xBF5AF2);

                        let _ = new_message
                            .channel_id
                            .send_message(
                                &ctx.http,
                                serenity::builder::CreateMessage::new().embed(embed),
                            )
                            .await;

                        // assign level roles
                        if let Ok(roles) =
                            crate::db::leveling::get_level_roles(&db.pool, guild_id).await
                        {
                            for lr in roles {
                                if user_lvl.level >= lr.level {
                                    let _ = ctx
                                        .http
                                        .add_member_role(
                                            new_message.guild_id.unwrap(),
                                            new_message.author.id,
                                            serenity::RoleId::new(lr.role_id as u64),
                                            Some("Level role reward"),
                                        )
                                        .await;
                                }
                            }
                        }
                    }
                }
            }

            // ── AutoMod ──────────────────────────────────────────────
            if config.automod_enabled {
                let content_lower = new_message.content.to_lowercase();
                let content_raw = &new_message.content;
                let author_id = new_message.author.id;
                let channel_id = new_message.channel_id;
                let log_channel = config.mod_log_channel;

                let send_automod_alert = |reason: &'static str, detail: String| {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("🛡️ AutoMod — Message Removed")
                        .description(format!("<@{}>'s message was removed.", author_id))
                        .field("Reason", reason, true)
                        .field("Detail", detail, false)
                        .footer(serenity::builder::CreateEmbedFooter::new(
                            "AegisForge AutoMod",
                        ))
                        .timestamp(serenity::Timestamp::now())
                        .color(0xFF3B3B);
                    embed
                };

                // Blacklist phrases
                if let Ok(blacklist) = db.get_automod_blacklist(guild_id).await {
                    if let Some(phrase) = blacklist
                        .iter()
                        .find(|p| content_lower.contains(p.as_str()))
                    {
                        let _ = new_message.delete(ctx).await;
                        let embed =
                            send_automod_alert("Blacklisted phrase", format!("`{}`", phrase));
                        let _ = channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;
                        if let Some(lc) = log_channel {
                            let log_embed = serenity::builder::CreateEmbed::new()
                                .title("🛡️ AutoMod — Blacklist Hit")
                                .field("User", format!("<@{}>", author_id), true)
                                .field("Channel", format!("<#{}>", channel_id), true)
                                .field("Phrase", format!("`{}`", phrase), false)
                                .color(0xFF3B3B)
                                .timestamp(serenity::Timestamp::now());
                            let _ = serenity::ChannelId::new(lc as u64)
                                .send_message(
                                    ctx,
                                    serenity::builder::CreateMessage::new().embed(log_embed),
                                )
                                .await;
                        }
                        return Ok(());
                    }
                }

                // Anti-invite links
                if config.automod_invites {
                    let has_invite = content_lower.contains("discord.gg/")
                        || content_lower.contains("discord.com/invite/")
                        || content_lower.contains("discordapp.com/invite/");
                    if has_invite {
                        let _ = new_message.delete(ctx).await;
                        let embed = send_automod_alert(
                            "Discord invite link",
                            "Invite links are not allowed.".into(),
                        );
                        let _ = channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;
                        if let Some(lc) = log_channel {
                            let log_embed = serenity::builder::CreateEmbed::new()
                                .title("🛡️ AutoMod — Invite Link Blocked")
                                .field("User", format!("<@{}>", author_id), true)
                                .field("Channel", format!("<#{}>", channel_id), true)
                                .color(0xFF3B3B)
                                .timestamp(serenity::Timestamp::now());
                            let _ = serenity::ChannelId::new(lc as u64)
                                .send_message(
                                    ctx,
                                    serenity::builder::CreateMessage::new().embed(log_embed),
                                )
                                .await;
                        }
                        return Ok(());
                    }
                }

                // Anti-caps (>70% uppercase, message must be >15 chars)
                if config.automod_caps && content_raw.len() > 15 {
                    let letters = content_raw.chars().filter(|c| c.is_alphabetic()).count();
                    let caps = content_raw.chars().filter(|c| c.is_uppercase()).count();
                    if letters > 5 && caps * 100 / letters >= 70 {
                        let _ = new_message.delete(ctx).await;
                        let embed = send_automod_alert(
                            "Excessive caps",
                            format!("{}% uppercase", caps * 100 / letters),
                        );
                        let _ = channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;
                        if let Some(lc) = log_channel {
                            let log_embed = serenity::builder::CreateEmbed::new()
                                .title("🛡️ AutoMod — Caps Violation")
                                .field("User", format!("<@{}>", author_id), true)
                                .field("Caps %", format!("{}%", caps * 100 / letters), true)
                                .color(0xFF3B3B)
                                .timestamp(serenity::Timestamp::now());
                            let _ = serenity::ChannelId::new(lc as u64)
                                .send_message(
                                    ctx,
                                    serenity::builder::CreateMessage::new().embed(log_embed),
                                )
                                .await;
                        }
                        return Ok(());
                    }
                }

                // Anti-mention spam (≥5 unique user or role mentions)
                if config.automod_mentions {
                    let mention_count =
                        new_message.mentions.len() + new_message.mention_roles.len();
                    if mention_count >= 5 {
                        let _ = new_message.delete(ctx).await;
                        let embed = send_automod_alert(
                            "Mass mentions",
                            format!("{} mentions in one message", mention_count),
                        );
                        let _ = channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;
                        if let Some(lc) = log_channel {
                            let log_embed = serenity::builder::CreateEmbed::new()
                                .title("🛡️ AutoMod — Mass Mentions")
                                .field("User", format!("<@{}>", author_id), true)
                                .field("Mentions", format!("`{}`", mention_count), true)
                                .color(0xFF3B3B)
                                .timestamp(serenity::Timestamp::now());
                            let _ = serenity::ChannelId::new(lc as u64)
                                .send_message(
                                    ctx,
                                    serenity::builder::CreateMessage::new().embed(log_embed),
                                )
                                .await;
                        }
                        return Ok(());
                    }
                }

                // Anti-spam (same message ≥3 times within 10 seconds)
                if config.automod_spam {
                    let guild_id_u64 = new_message.guild_id.unwrap().get();
                    let user_id_u64 = author_id.get();
                    let now = Instant::now();
                    let msg_text = content_raw.clone();

                    let dupe_count = {
                        let mut entry = data
                            .spam_tracker
                            .entry((guild_id_u64, user_id_u64))
                            .or_default();
                        entry.retain(|(t, _)| now.duration_since(*t) < Duration::from_secs(10));
                        entry.push_back((now, msg_text.clone()));
                        entry.iter().filter(|(_, c)| c == &msg_text).count()
                    };

                    if dupe_count >= 3 {
                        let _ = new_message.delete(ctx).await;
                        let embed = send_automod_alert(
                            "Message spam",
                            format!("Same message sent {} times in 10s", dupe_count),
                        );
                        let _ = channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;
                        if let Some(lc) = log_channel {
                            let log_embed = serenity::builder::CreateEmbed::new()
                                .title("🛡️ AutoMod — Spam Detected")
                                .field("User", format!("<@{}>", author_id), true)
                                .field("Repeated", format!("`{}x`", dupe_count), true)
                                .color(0xFF3B3B)
                                .timestamp(serenity::Timestamp::now());
                            let _ = serenity::ChannelId::new(lc as u64)
                                .send_message(
                                    ctx,
                                    serenity::builder::CreateMessage::new().embed(log_embed),
                                )
                                .await;
                        }
                        return Ok(());
                    }
                }
            }
        }

        // ── Message Delete Logging ───────────────────────────────────
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id: Some(gid),
            ..
        } => {
            let guild_id_i64 = gid.get() as i64;
            if let Ok(config) = data.database.get_guild_config(guild_id_i64).await {
                if let Some(lc) = config.message_log_channel {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("🗑️ Message Deleted")
                        .field("Channel", format!("<#{}>", channel_id), true)
                        .field("Message ID", format!("`{}`", deleted_message_id), true)
                        .footer(serenity::builder::CreateEmbedFooter::new(
                            "AegisForge Message Log",
                        ))
                        .timestamp(serenity::Timestamp::now())
                        .color(0xFF4500);
                    let _ = serenity::ChannelId::new(lc as u64)
                        .send_message(
                            &ctx.http,
                            serenity::builder::CreateMessage::new().embed(embed),
                        )
                        .await;
                }
            }
        }

        // ── Message Edit Logging ─────────────────────────────────────
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {
            let guild_id = match event.guild_id {
                Some(g) => g,
                None => return Ok(()),
            };
            // skip bot edits
            if new.as_ref().map(|m| m.author.bot).unwrap_or(false) {
                return Ok(());
            }
            let guild_id_i64 = guild_id.get() as i64;
            if let Ok(config) = data.database.get_guild_config(guild_id_i64).await {
                if let Some(lc) = config.message_log_channel {
                    let author_str = new
                        .as_ref()
                        .map(|m| format!("<@{}>", m.author.id))
                        .unwrap_or_else(|| "Unknown".into());
                    let old_content = old_if_available
                        .as_ref()
                        .map(|m| m.content.chars().take(512).collect::<String>())
                        .unwrap_or_else(|| "_Not cached_".into());
                    let new_content = new
                        .as_ref()
                        .map(|m| m.content.chars().take(512).collect::<String>())
                        .unwrap_or_else(|| "_Unknown_".into());

                    let embed = serenity::builder::CreateEmbed::new()
                        .title("✏️ Message Edited")
                        .field("Author", author_str, true)
                        .field("Channel", format!("<#{}>", event.channel_id), true)
                        .field("Before", &old_content, false)
                        .field("After", &new_content, false)
                        .footer(serenity::builder::CreateEmbedFooter::new(
                            "AegisForge Message Log",
                        ))
                        .timestamp(serenity::Timestamp::now())
                        .color(0xFEE75C);
                    let _ = serenity::ChannelId::new(lc as u64)
                        .send_message(
                            &ctx.http,
                            serenity::builder::CreateMessage::new().embed(embed),
                        )
                        .await;
                }
            }
        }

        // ── Member Leave Logging + Goodbye Message ───────────────────
        serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            let guild_id_i64 = guild_id.get() as i64;
            if let Ok(config) = data.database.get_guild_config(guild_id_i64).await {
                // Member log
                if let Some(lc) = config.member_log_channel {
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("📤 Member Left")
                        .description(format!("**{}** left the server.", user.name))
                        .thumbnail(user.face())
                        .field("User ID", format!("`{}`", user.id), true)
                        .field(
                            "Account Age",
                            format!("<t:{}:R>", user.id.created_at().unix_timestamp()),
                            true,
                        )
                        .footer(serenity::builder::CreateEmbedFooter::new(
                            "AegisForge Member Log",
                        ))
                        .timestamp(serenity::Timestamp::now())
                        .color(0xFF4500);
                    let _ = serenity::ChannelId::new(lc as u64)
                        .send_message(
                            &ctx.http,
                            serenity::builder::CreateMessage::new().embed(embed),
                        )
                        .await;
                }

                // Goodbye message
                if let Some(gc) = config.goodbye_channel {
                    if !config.goodbye_message.is_empty() {
                        let server_name = ctx
                            .cache
                            .guild(*guild_id)
                            .map(|g| g.name.clone())
                            .unwrap_or_else(|| "the server".to_string());
                        let msg = config
                            .goodbye_message
                            .replace("{user}", &user.name)
                            .replace("{server}", &server_name);
                        let embed = serenity::builder::CreateEmbed::new()
                            .title(format!("👋 Goodbye from {}!", server_name))
                            .description(&msg)
                            .thumbnail(user.face())
                            .color(0x2C2F33);
                        let _ = serenity::ChannelId::new(gc as u64)
                            .send_message(
                                &ctx.http,
                                serenity::builder::CreateMessage::new().embed(embed),
                            )
                            .await;
                    }
                }
            }
        }

        _ => {}
    }

    Ok(())
}

/// set the bot's Discord presence. `idx` rotates through branded status messages.
fn set_presence(ctx: &serenity::Context, guild_count: usize, idx: usize) {
    let (activity, status) = match idx % 6 {
        0 => (
<<<<<<< HEAD
            serenity::ActivityData::watching(format!(
                "{} servers | v{}",
                guild_count,
                env!("CARGO_PKG_VERSION")
            )),
=======
            serenity::ActivityData::watching(format!("{} servers | v4.3", guild_count)),
>>>>>>> 464415d48bbb577285feea95e643bf0a924170dd
            serenity::OnlineStatus::Online,
        ),
        1 => (
            serenity::ActivityData::playing("AegisForge | /help"),
            serenity::OnlineStatus::Online,
        ),
        2 => (
            serenity::ActivityData::watching(format!("over {} communities grow", guild_count)),
            serenity::OnlineStatus::Online,
        ),
        3 => (
            serenity::ActivityData::listening("aegisforge-vert.vercel.app"),
            serenity::OnlineStatus::Online,
        ),
        4 => (
            serenity::ActivityData::playing("Sentinel Anti-Raid Active"),
            serenity::OnlineStatus::Online,
        ),
        _ => (
            serenity::ActivityData::playing("AutoMod Protection Active"),
            serenity::OnlineStatus::Online,
        ),
    };
    ctx.set_presence(Some(activity), status);
}
