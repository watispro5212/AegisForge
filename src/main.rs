use poise::serenity_prelude as serenity;
use tracing::info;
use std::env;
use dotenvy::dotenv;

mod commands;
mod handler;
mod models;

pub struct Data {
    pub database: sqlx::SqlitePool,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:aegisforge.db".to_string());

    info!("Connecting to database at {}", database_url);
    let database = sqlx::SqlitePool::connect(&database_url).await?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Utility
                commands::utility::ping(),
                commands::utility::server(),
                commands::utility::user(),
                commands::utility::avatar(),
                commands::utility::uptime(),
                commands::utility::timestamp(),
                // Moderation
                commands::moderation::ban(),
                commands::moderation::unban(),
                commands::moderation::kick(),
                commands::moderation::timeout(),
                commands::moderation::warn(),
                commands::moderation::purge(),
                // Role management
                commands::role::add(),
                commands::role::remove(),
                commands::role::list(),
                // Config
                commands::config::logs(),
                commands::config::welcome(),
                commands::config::autorole(),
                // Reminders
                commands::remind::create(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            tracing::error!("Command error in '{}': {:?}", ctx.command().name, error);
                            let _ = ctx.say(format!("❌ Error: {}", error)).await;
                        }
                        other => {
                            tracing::error!("Unhandled framework error: {:?}", other);
                        }
                    }
                })
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("🔩 AegisForge is online as {}!", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}
