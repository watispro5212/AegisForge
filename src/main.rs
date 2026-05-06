#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use poise::serenity_prelude as serenity;
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, Level};
use std::{env, sync::Arc};
use dotenvy::dotenv;
use serenity::prelude::*;
use axum::{routing::get, Json, Router, extract::State};
use tower_http::cors::{CorsLayer, Any};
use serde::Serialize;

mod commands;
mod db;
mod handler;
mod models;

use db::Database;

/// The shared application state passed into every command context.
#[derive(Debug)]
pub struct Data {
    pub database: Arc<Database>,
    pub start_time: std::time::Instant,
}

#[derive(Serialize)]
struct Stats {
    server_count: usize,
    user_count: usize,
    uptime_seconds: u64,
}

#[derive(Clone)]
struct AppState {
    cache: Arc<serenity::cache::Cache>,
    start_time: std::time::Instant,
}

async fn get_stats(State(state): State<AppState>) -> Json<Stats> {
    let guilds = state.cache.guild_count();
    let users = state.cache.user_count();
    
    Json(Stats {
        server_count: guilds,
        user_count: users,
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let direct_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in .env");
    
    // ── Migration pool ──────────────────────────────────────────
    info!("Running database migrations...");
    let migrate_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&direct_url)
        .await
        .expect("Failed to connect for migrations");
    sqlx::migrate!("./migrations").run(&migrate_pool).await?;
    migrate_pool.close().await;

    // ── App pool ────────────────────────────────────────────────
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&direct_url)
        .await?;
    let database = Arc::new(Database::new(pool));

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env");
    let start_time = std::time::Instant::now();

    // ── Poise Framework ─────────────────────────────────────────
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Utility
                commands::utility::help(),
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
                commands::utility::fact(),
                commands::utility::cat(),
                commands::utility::cookie(),
                // Moderation
                commands::moderation::ban(),
                commands::moderation::unban(),
                commands::moderation::kick(),
                commands::moderation::slowmode(),
                commands::moderation::lock(),
                commands::moderation::unlock(),
                commands::moderation::timeout(),
                commands::moderation::mute(),
                commands::moderation::unmute(),
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
                commands::config::prefix(),
                // Reminders
                commands::remind::create(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            error!("Command error in {}: {:?}", ctx.command().name, error);
                            let _ = ctx.send(poise::CreateReply::default()
                                .content(format!("❌ **Error:** {}", error))
                                .ephemeral(true)).await;
                        }
                        other => error!("Framework error: {:?}", other),
                    }
                })
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(handler::event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixOptions {
                prefix: Some("!".into()),
                dynamic_prefix: Some(|ctx| {
                    Box::pin(async move {
                        let guild_id = ctx.guild_id.map(|id| id.get() as i64).unwrap_or(0);
                        if guild_id == 0 { return Ok(Some("!".into())); }
                        let config = ctx.data.database.get_guild_config(guild_id).await.ok();
                        Ok(config.map(|c| c.prefix))
                    })
                }),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            let db = Arc::clone(&database);
            Box::pin(async move {
                info!("🔩 AegisForge online as {}!", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { 
                    database: db,
                    start_time,
                })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await?;

    // ── Stats Server (Axum) ─────────────────────────────────────
    let app_state = AppState {
        cache: Arc::clone(&client.cache),
        start_time,
    };

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/api/stats", get(get_stats))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any))
        .with_state(app_state);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("📊 Stats API listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("Stats server error: {}", e);
        }
    });

    // ── Start Bot ───────────────────────────────────────────────
    client.start().await?;

    Ok(())
}
