use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use chrono::{Utc, Duration};

/// Create a reminder for yourself
#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Minutes until the reminder fires"] minutes: u64,
    #[description = "What to remind you about"] message: String,
) -> Result<(), Error> {
    let author = ctx.author().clone();
    let channel = ctx.channel_id();
    let http = ctx.serenity_context().http.clone();
    let reminder_msg = message.clone();

    let fire_at = Utc::now() + Duration::minutes(minutes as i64);

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .title("⏰ Reminder Set")
            .description(format!("I'll remind you: **{}**", &message))
            .field("Fires in", format!("<t:{}:R>", fire_at.timestamp()), true)
            .color(0x00ffff),
    ))
    .await?;

    // Spawn a task to send the reminder after the delay
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(minutes * 60)).await;
        let _ = channel
            .send_message(&http, serenity::CreateMessage::default()
                .embed(
                    serenity::CreateEmbed::default()
                        .title("⏰ Reminder")
                        .description(format!("<@{}>, here's your reminder:\n**{}**", author.id, reminder_msg))
                        .color(0x00ffff),
                )
            )
            .await;
    });

    Ok(())
}
