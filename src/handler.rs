use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tracing::{info, warn};
use poise::serenity_prelude as serenity;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, crate::Error>,
    _data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            info!("AegisForge connected as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::MessageDelete { channel_id, message_id, guild_id } => {
            info!("Message deleted in channel {}: {}", channel_id, message_id);
            // Logging logic here
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            info!("New member joined: {}", new_member.user.name);
            // Auto-role and welcome logic here
        }
        _ => {}
    }
    Ok(())
}
