#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use poise::serenity_prelude as serenity;
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, Level};
use std::{env, sync::Arc};
use dotenvy::dotenv;

mod commands;
mod db;
mod handler;
mod models;

use db::Database;

/// The shared application state passed into every command context.
#[derive(Debug)]
pub struct Data {
    pub database: Arc<Database>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let direct_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in .env");
    let pool_url = env::var("DATABASE_POOL_URL").unwrap_or_else(|_| direct_url.clone());


    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env");

    // ── Migration pool (direct, no PgBouncer) ───────────────────
    // Neon docs: migrations must use the direct connection — DDL
    // statements are not safe through PgBouncer transaction mode.
    info!("Running database migrations (direct connection)...");
    let migrate_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&direct_url)
        .await
        .expect("Failed to connect for migrations. Is DATABASE_URL correct?");
    sqlx::migrate!("./migrations").run(&migrate_pool).await?;
    migrate_pool.close().await;
    info!("Migrations complete.");

    // ── App pool (pooled, through Neon PgBouncer) ───────────────
    // PgBouncer handles up to 10,000 client connections.
    // SQLx holds a small number of persistent server connections to PgBouncer;
    // PgBouncer multiplexes all bot queries across them.
    info!("Connecting app pool via Neon PgBouncer...");
    let pool = PgPoolOptions::new()
        .max_connections(
            env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        )
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(300))
        .max_lifetime(std::time::Duration::from_secs(900))
        .connect(&pool_url)
        .await
        .expect("Failed to connect app pool. Is DATABASE_POOL_URL correct?");

    let database = Arc::new(Database::new(pool));

    // ── Poise framework ──────────────────────────────────────
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
                commands::utility::coinflip(),
                commands::utility::dice(),
                commands::utility::poll(),
                commands::utility::serverinfo(),
                commands::utility::whois(),
                commands::utility::eightball(),
                commands::utility::joke(),
                commands::utility::embed(),
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
                            tracing::error!(
                                command = ctx.command().name,
                                "Command error: {:?}", error
                            );
                            let _ = ctx
                                .send(poise::CreateReply::default()
                                    .content(format!("❌ {}", error))
                                    .ephemeral(true))
                                .await;
                        }
                        other => tracing::error!("Framework error: {:?}", other),
                    }
                })
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            let db = Arc::clone(&database);
            Box::pin(async move {
                info!("🔩 AegisForge online as {}!", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database: db })
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
