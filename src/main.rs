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

// bot data stuff i guess
#[derive(Debug)]
pub struct Data {
    pub database: Arc<Database>,
    pub start_time: std::time::Instant,
}

#[derive(Serialize)]
struct ShardStatus {
    id: u32,
    latency_ms: u64,
    status: String,
}

#[derive(Serialize)]
struct Stats {
    server_count: usize,
    user_count: usize,
    uptime_seconds: u64,
    economy_activity: i64,
    xp_gain_24h: i64,
    shards_total: u64,
    shards_online: u64,
    shards: Vec<ShardStatus>,
}

#[derive(Clone)]
struct AppState {
    cache: Arc<serenity::cache::Cache>,
    database: Arc<Database>,
    shard_manager: Arc<serenity::ShardManager>,
    start_time: std::time::Instant,
}

async fn get_stats(State(state): State<AppState>) -> Json<Stats> {
    let guilds = state.cache.guild_count();
    let users = state.cache.user_count();
    
    let pool = &state.database.pool;
    let total_wealth = crate::db::economy::get_total_wealth(pool).await.unwrap_or(0);
    let total_xp = crate::db::leveling::get_total_xp(pool).await.unwrap_or(0);

    let mut shard_statuses = Vec::new();
    let runners = state.shard_manager.runners.lock().await;
    
    for (id, runner) in runners.iter() {
        shard_statuses.push(ShardStatus {
            id: id.0,
            latency_ms: runner.latency.map(|d| d.as_millis() as u64).unwrap_or(0),
            status: format!("{:?}", runner.stage),
        });
    }

    let shards_total = shard_statuses.len() as u64;
    let shards_online = shard_statuses.iter().filter(|s| s.status == "Connected").count() as u64;
    
    Json(Stats {
        server_count: guilds,
        user_count: users,
        uptime_seconds: state.start_time.elapsed().as_secs(),
        economy_activity: total_wealth,
        xp_gain_24h: total_xp,
        shards_total,
        shards_online,
        shards: shard_statuses,
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

    let direct_url = env::var("DATABASE_URL").map_err(|_| "Missing DATABASE_URL environment variable")?;
    let pool_url = env::var("DATABASE_POOL_URL").unwrap_or_else(|_| direct_url.clone());
    
    // doing the migrations lol
    info!("doing migrations lol...");
    let migrate_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&direct_url)
        .await
        .expect("Failed to connect for migrations");
    sqlx::migrate!("./migrations").run(&migrate_pool).await?;
    migrate_pool.close().await;

    // connecting the bot to the db
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&pool_url)
        .await
        .map_err(|e| format!("Failed to connect to application database: {}", e))?;
    let database = Arc::new(Database::new(pool));

    let token = env::var("DISCORD_TOKEN").map_err(|_| "Missing DISCORD_TOKEN environment variable")?;
    let start_time = std::time::Instant::now();

    // setting up the commands
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // utility
                commands::utility::help(),
                commands::utility::ping(),
                commands::utility::avatar(),
                commands::utility::uptime(),
                commands::utility::stats(),
                commands::utility::timestamp(),
                commands::utility::serverinfo(),
                commands::utility::whois(),
                commands::utility::embed(),
                commands::utility::math(),
                commands::utility::qr(),
                commands::utility::crypto(),
                commands::utility::translate(),
                commands::utility::timer(),
                commands::utility::dictionary(),
                commands::utility::worldclock(),
                commands::utility::poll(),
                // fun
                commands::fun::fun(),
                // economy
                commands::economy::economy(),
                // leveling
                commands::leveling::leveling(),
                // moderation
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
                // role management
                commands::role::add(),
                commands::role::remove(),
                commands::role::list(),
                // config
                commands::config::logs(),
                commands::config::welcome(),
                commands::config::autorole(),
                commands::config::prefix(),
                commands::config::settings(),
                // reminders
                commands::remind::create(),
            ],
            post_command: |ctx| {
                Box::pin(async move {
                    if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
                        let http = &ctx.serenity_context().http;
                        let command_name = ctx.command().name.clone();
                        let user_name = ctx.author().name.clone();
                        let guild_name = ctx.guild().map(|g| g.name.clone()).unwrap_or_else(|| "DMs".to_string());

                        if let Ok(webhook) = serenity::model::webhook::Webhook::from_url(http, &webhook_url).await {
                            let embed = serenity::builder::CreateEmbed::new()
                                .title("⚡ command used lol")
                                .description(format!("**{}** used `/{}` in **{}**", user_name, command_name, guild_name))
                                .color(0x00E5FF);
                            let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                            let _ = webhook.execute(http, false, builder).await;
                        }
                    }
                })
            },
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
            prefix_options: poise::PrefixFrameworkOptions {
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
        .setup({
            let database = Arc::clone(&database);
            move |ctx, ready, framework| {
                let db = Arc::clone(&database);
                Box::pin(async move {
                    info!("🔩 AegisForge online as {}!", ready.user.name);
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data { 
                        database: db,
                        start_time,
                    })
                })
            }
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await?;

    let shard_manager = Arc::clone(&client.shard_manager);

    // the stats api thingy
    let app_state = AppState {
        cache: Arc::clone(&client.cache),
        database: Arc::clone(&database),
        shard_manager,
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

    // start the bot i guess
    client.start().await?;

    Ok(())
}

