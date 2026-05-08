use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use chrono::Utc;

/// create a reminder for yourself
#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Minutes until the reminder fires"] minutes: u64,
    #[description = "What to remind you about"] message: String,
) -> Result<(), Error> {
    let author_id = ctx.author().id;
    let channel = ctx.channel_id();
    let http = ctx.serenity_context().http.clone();
    let reminder_msg = message.clone();

    let fire_at = Utc::now().timestamp() + (minutes as i64 * 60);

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⏰ Reminder Set")
            .description(format!("I'll remind you: **{}**", &message))
            .field("Fires in", format!("<t:{}:R>", fire_at), true)
            .color(0x00ffff),
    ))
    .await?;

    // spawn a background task — fires even if the command context is dropped
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(minutes * 60)).await;
        let _ = channel
            .send_message(&http, serenity::CreateMessage::default()
                .embed(
                    serenity::CreateEmbed::new()
                        .title("⏰ Reminder")
                        .description(format!("<@{}>, here's your reminder:\n**{}**", author_id, reminder_msg))
                        .color(0x00ffff),
                )
            )
            .await;
    });

    Ok(())
}


