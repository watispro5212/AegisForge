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
            info!("AegisForge connected as {}", data_about_bot.user.name);
            _ctx.set_presence(Some(serenity::ActivityData::watching("over AegisForge HQ | v2.0")), serenity::OnlineStatus::Online);
        }
        serenity::FullEvent::MessageDelete { channel_id, deleted_message_id, guild_id: _ } => {
            info!("Message deleted in channel {}: {}", channel_id, deleted_message_id);
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            info!("New member joined: {}", new_member.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            let content = new_message.content.to_lowercase();
            let bad_words = ["badword1", "badword2", "spamlink.com", "freemoney.com"];
            
            if bad_words.iter().any(|word| content.contains(word)) {
                if let Err(e) = new_message.delete(_ctx).await {
                    tracing::error!("Failed to delete auto-modded message: {:?}", e);
                } else {
                    info!("Auto-deleted message from {} containing blacklisted word.", new_message.author.name);
                    let _ = new_message.channel_id.say(_ctx, format!("<@{}> Please refrain from using blacklisted words or links.", new_message.author.id)).await;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
