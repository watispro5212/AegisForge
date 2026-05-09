use poise::serenity_prelude as serenity;
use std::time::Duration;
use tracing::{error, info};

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

            // rotating presence — cycles every 60s through branded status messages
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                let mut idx: usize = 1;
                loop {
                    tokio::time::sleep(Duration::from_secs(60)).await;
                    let guilds = ctx_clone.cache.guild_count();
                    set_presence(&ctx_clone, guilds, idx);
                    idx = (idx + 1) % 4;
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
                                    .footer(serenity::builder::CreateEmbedFooter::new(
                                        format!(
                                            "AegisForge v{} - High-Performance Discord Automation",
                                            env!("CARGO_PKG_VERSION")
                                        ),
                                    ))
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
                        if let Ok(webhook) =
                            serenity::model::webhook::Webhook::from_url(&http, &webhook_url).await
                        {
                            let embed = serenity::builder::CreateEmbed::new()
                                .title("📥 New Server Joined")
                                .description(format!("AegisForge was added to **{}**.", guild_name))
                                .field("Members", format!("`{}`", member_count), true)
                                .field("Total Servers", format!("`{}`", guild_count), true)
                                .field("Server ID", format!("`{}`", guild_id), true)
                                .footer(serenity::builder::CreateEmbedFooter::new(
                                    format!(
                                        "AegisForge v{} - Guild Join Event",
                                        env!("CARGO_PKG_VERSION")
                                    ),
                                ))
                                .timestamp(serenity::Timestamp::now())
                                .color(0x57F287);
                            let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                            let _ = webhook.execute(&http, false, builder).await;
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
            let db = &data.database;

            if let Ok(config) = db.get_guild_config(guild_id).await {
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

            // ── Automod — guild-specific blacklist only ──────────────
            if config.automod_enabled {
                let content = new_message.content.to_lowercase();

                if let Ok(guild_blacklist) = db.get_automod_blacklist(guild_id).await {
                    let hit = guild_blacklist
                        .iter()
                        .find(|phrase| content.contains(&phrase.to_lowercase()));

                    if let Some(_phrase) = hit {
                        let _ = new_message.delete(ctx).await;

                        let embed = serenity::builder::CreateEmbed::new()
                            .title("🛡️ AutoMod — Message Removed")
                            .description(format!(
                                "<@{}>'s message was removed for containing a blacklisted phrase.",
                                new_message.author.id
                            ))
                            .field("Reason", "Blacklisted phrase detected", false)
                            .footer(serenity::builder::CreateEmbedFooter::new(
                                "Configure your blacklist with /automod settings",
                            ))
                            .color(0xFF3B3B);

                        let _ = new_message
                            .channel_id
                            .send_message(ctx, serenity::builder::CreateMessage::new().embed(embed))
                            .await;

                        return Ok(());
                    }
                }
            }
        }

        _ => {}
    }

    Ok(())
}

/// set the bot's Discord presence. `idx` rotates through 4 branded status messages.
fn set_presence(ctx: &serenity::Context, guild_count: usize, idx: usize) {
    let (activity, status) = match idx % 4 {
        0 => (
            serenity::ActivityData::watching(format!("{} servers | /help", guild_count)),
            serenity::OnlineStatus::Online,
        ),
        1 => (
            serenity::ActivityData::playing("economy | /economy balance"),
            serenity::OnlineStatus::Online,
        ),
        2 => (
            serenity::ActivityData::watching(format!("over {} communities grow", guild_count)),
            serenity::OnlineStatus::Online,
        ),
        _ => (
            serenity::ActivityData::listening("aegisforge-vert.vercel.app"),
            serenity::OnlineStatus::Online,
        ),
    };
    ctx.set_presence(Some(activity), status);
}
