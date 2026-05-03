use poise::serenity_prelude as serenity;
use tracing::info;

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
        serenity::FullEvent::MessageDelete { channel_id, deleted_message_id, guild_id: _ } => {
            info!("Message deleted in channel {}: {}", channel_id, deleted_message_id);
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            info!("New member joined: {}", new_member.user.name);
        }
        _ => {}
    }
    Ok(())
}
