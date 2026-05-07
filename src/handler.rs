use poise::serenity_prelude as serenity;
use tracing::info;

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, crate::Error>,
    _data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            info!("bot is up lol: {}", data_about_bot.user.name);
            let guild_count = _ctx.cache.guild_count();
            _ctx.set_presence(Some(serenity::ActivityData::watching(format!("over {} servers | aegisforge.com", guild_count))), serenity::OnlineStatus::Online);
            
            if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
                if let Ok(webhook) = serenity::model::webhook::Webhook::from_url(&_ctx.http, &webhook_url).await {
                    let count = _ctx.cache.guild_count();
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("🚀 up now")
                        .description(format!("bot is online for **{}** servers.", count))
                        .color(0x00E5FF);
                    let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                    let _ = webhook.execute(&_ctx.http, false, builder).await;
                }
            }
        }
        serenity::FullEvent::GuildCreate { guild, is_new } => {
            let guild_count = _ctx.cache.guild_count();
            _ctx.set_presence(Some(serenity::ActivityData::watching(format!("over {} servers | aegisforge.com", guild_count))), serenity::OnlineStatus::Online);

            if let Some(true) = is_new {
                if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
                    if let Ok(webhook) = serenity::model::webhook::Webhook::from_url(&_ctx.http, &webhook_url).await {
                        let member_count = guild.member_count;
                        let embed = serenity::builder::CreateEmbed::new()
                            .title("📥 new server join")
                            .description(format!("bot joined **{}**! it has **{}** people.", guild.name, member_count))
                            .color(0x57F287);
                        let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                        let _ = webhook.execute(&_ctx.http, false, builder).await;
                    }
                }
            }
        }
        serenity::FullEvent::GuildDelete { incomplete, .. } => {
            let guild_count = _ctx.cache.guild_count();
            _ctx.set_presence(Some(serenity::ActivityData::watching(format!("over {} servers | aegisforge.com", guild_count))), serenity::OnlineStatus::Online);
        }

        serenity::FullEvent::MessageDelete { channel_id, deleted_message_id, guild_id: _ } => {
            info!("Message deleted in channel {}: {}", channel_id, deleted_message_id);
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let guild_id = new_member.guild_id.get() as i64;
            let db = &_data.database;
            
            if let Ok(config) = db.get_guild_config(guild_id).await {
                // Autorole
                if let Some(role_id) = config.auto_role_id {
                    let _ = new_member.add_role(&_ctx.http, serenity::RoleId::new(role_id as u64)).await;
                }

                // welcome stuff
                if let Some(channel_id) = config.welcome_channel {
                    let template = &config.welcome_message;
                    if !template.is_empty() {
                        let server_name = _ctx.cache.guild(new_member.guild_id).map(|g| g.name.clone()).unwrap_or_else(|| "the server".to_string());
                        let msg = template
                            .replace("{user}", &format!("<@{}>", new_member.user.id))
                            .replace("{server}", &server_name);
                        
                        let _ = serenity::ChannelId::new(channel_id as u64).say(&_ctx.http, msg).await;
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

            let db = &_data.database;
            let config = match db.get_guild_config(guild_id).await {
                Ok(c) => c,
                Err(_) => return Ok(()),
            };

            // ── Leveling ────────────────────────────────────────────
            if config.leveling_enabled {
                match crate::db::leveling::add_xp(&db.pool, guild_id, new_message.author.id.get() as i64, 15).await {
                    Ok(true) => {
                        // level up lol
                        if let Ok(user_lvl) = crate::db::leveling::get_user_leveling(&db.pool, guild_id, new_message.author.id.get() as i64).await {
                            let template = if config.level_up_message.is_empty() {
                                "GG {user}, you leveled up to **Level {level}**!".to_string()
                            } else {
                                config.level_up_message.clone()
                            };
                            let msg = template
                                .replace("{user}", &format!("<@{}>", new_message.author.id))
                                .replace("{level}", &user_lvl.level.to_string());
                            
                            let _ = new_message.channel_id.say(&_ctx.http, msg).await;

                            // Check for level roles
                            if let Ok(roles) = crate::db::leveling::get_level_roles(&db.pool, guild_id).await {
                                for lr in roles {
                                    if user_lvl.level >= lr.level {
                                        let _ = _ctx.http.add_member_role(
                                            new_message.guild_id.unwrap(),
                                            new_message.author.id,
                                            serenity::RoleId::new(lr.role_id as u64),
                                            Some("Level role reward"),
                                        ).await;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            // ── Automod ─────────────────────────────────────────────
            let content = new_message.content.to_lowercase();
            
            // get blacklist
            let blacklisted = vec!["badword1", "badword2", "spamlink.com", "freemoney.com"];
            if let Ok(guild_blacklist) = db.get_automod_blacklist(guild_id).await {
                for phrase in guild_blacklist {
                    if content.contains(&phrase.to_lowercase()) {
                        let _ = new_message.delete(_ctx).await;
                        let _ = new_message.channel_id.say(_ctx, format!("🛡️ **automod:** <@{}> no bad words lol.", new_message.author.id)).await;
                        return Ok(());
                    }
                }
            }
            
            if blacklisted.iter().any(|word| content.contains(word)) {
                let _ = new_message.delete(_ctx).await;
                let _ = new_message.channel_id.say(_ctx, format!("🛡️ **Aegis Automod:** <@{}> Blacklisted word detected.", new_message.author.id)).await;
            }
        }
        _ => {}
    }
    Ok(())
}
