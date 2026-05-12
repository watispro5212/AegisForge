#![allow(dead_code)]

use axum::{
    extract::{Json, State},
    http::HeaderMap,
    routing::get,
    routing::post,
    Router,
};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, Level};

#[derive(Debug, Deserialize)]
struct VotePayload {
    user: String,
    #[allow(dead_code)]
    bot: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    kind: String,
    #[serde(rename = "isWeekend")]
    is_weekend: bool,
}

mod commands;
mod db;
mod handler;
mod models;

use db::Database;

pub type SpamTracker =
    dashmap::DashMap<(u64, u64), std::collections::VecDeque<(std::time::Instant, String)>>;

#[derive(Debug)]
pub struct Data {
    pub database: Arc<Database>,
    pub start_time: std::time::Instant,
    pub http_client: reqwest::Client,
    pub raid_tracker: Arc<models::sentinel::RaidTracker>,
    pub sentinel_settings: Arc<dashmap::DashMap<u64, models::sentinel::SentinelConfig>>,
    pub spam_tracker: Arc<SpamTracker>,
}

#[derive(Serialize)]
struct ShardStatus {
    id: u32,
    latency_ms: u64,
    status: String,
}

async fn handle_vote(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<VotePayload>,
) -> impl axum::response::IntoResponse {
    info!("Received top.gg vote from user {}", payload.user);

    if let Ok(secret) = std::env::var("TOPGG_WEBHOOK_SECRET") {
        let auth = headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        let token = headers
            .get("x-topgg-authorization")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();

        let valid = auth == secret || auth == format!("Bearer {}", secret) || token == secret;
        if !valid {
            return axum::http::StatusCode::UNAUTHORIZED;
        }
    }

    let user_id: i64 = payload.user.parse().unwrap_or(0);
    if user_id == 0 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    // give bonus
    let bonus = if payload.is_weekend { 2000 } else { 1000 };
    let pool = &state.database.pool;

    // update all records for this user across all guilds
    let _ = sqlx::query(
        "UPDATE users_economy SET balance = balance + $1, total_earned = total_earned + $1 WHERE user_id = $2"
    )
    .bind(bonus)
    .bind(user_id)
    .execute(pool)
    .await;

    // notify webhook
    if let Ok(webhook_url) = std::env::var("STATUS_WEBHOOK_URL") {
        tokio::spawn(async move {
            let http = serenity::http::Http::new("");
            if let Ok(webhook) =
                serenity::model::webhook::Webhook::from_url(&http, &webhook_url).await
            {
                let embed = serenity::builder::CreateEmbed::new()
                    .title("AegisForge Vote Reward")
                    .description(format!(
                        "<@{}> voted on Top.gg and received **${}** across their economy profiles.",
                        payload.user, bonus
                    ))
                    .field("Reward", format!("`${}`", bonus), true)
                    .field(
                        "Multiplier",
                        if payload.is_weekend {
                            "Weekend 2x"
                        } else {
                            "Standard"
                        },
                        true,
                    )
                    .footer(serenity::builder::CreateEmbedFooter::new(
                        "AegisForge v4.2 - Vote Reward",
                    ))
                    .timestamp(serenity::Timestamp::now())
                    .color(0x00FF88);
                let builder = serenity::builder::ExecuteWebhook::new().embed(embed);
                let _ = webhook.execute(&http, false, builder).await;
            }
        });
    }

    axum::http::StatusCode::OK
}

#[derive(Serialize)]
struct Stats {
    server_count: usize,
    user_count: usize,
    uptime_seconds: u64,
    economy_activity: i64,
    xp_gain_24h: i64,
    total_commands_executed: i64,
    total_economy_transactions: i64,
    inventory_items: i64,
    shards_total: u64,
    shards_online: u64,
    shards: Vec<ShardStatus>,
    version: &'static str,
}

#[derive(Serialize)]
struct Health {
    ok: bool,
    service: &'static str,
    version: &'static str,
    uptime_seconds: u64,
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
    let total_wealth = crate::db::economy::get_total_wealth(pool)
        .await
        .unwrap_or(0);
    let total_xp = crate::db::leveling::get_total_xp(pool).await.unwrap_or(0);
    let total_commands = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_commands_executed'), 0)",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    let economy_transactions = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_economy_transactions'), 0)",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    let inventory_items = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(SUM(quantity), 0)::BIGINT FROM economy_inventory",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);

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
    let shards_online = shard_statuses
        .iter()
        .filter(|s| s.status == "Connected")
        .count() as u64;

    Json(Stats {
        server_count: guilds,
        user_count: users,
        uptime_seconds: state.start_time.elapsed().as_secs(),
        economy_activity: total_wealth,
        xp_gain_24h: total_xp,
        total_commands_executed: total_commands,
        total_economy_transactions: economy_transactions,
        inventory_items,
        shards_total,
        shards_online,
        shards: shard_statuses,
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn get_health(State(state): State<AppState>) -> Json<Health> {
    Json(Health {
        ok: true,
        service: "aegisforge",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let direct_url =
        env::var("DATABASE_URL").map_err(|_| "Missing DATABASE_URL environment variable")?;
    let pool_url = env::var("DATABASE_POOL_URL").unwrap_or_else(|_| direct_url.clone());

    info!("Running database migrations...");
    let migrate_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&direct_url)
        .await
        .expect("Failed to connect for migrations");
    if let Err(e) = sqlx::migrate!("./migrations").run(&migrate_pool).await {
        tracing::warn!("Migration error (likely VersionMismatch due to CRLF/LF): {}. Continuing anyway...", e);
    }
    migrate_pool.close().await;

    info!("Connecting application database pool...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&pool_url)
        .await
        .map_err(|e| format!("Failed to connect to application database: {}", e))?;
    let database = Arc::new(Database::new(pool));

    let token =
        env::var("DISCORD_TOKEN").map_err(|_| "Missing DISCORD_TOKEN environment variable")?;
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
                commands::utility::botinfo(),
                commands::utility::serverinfo(),
                commands::utility::whois(),
                commands::utility::vote(),
                commands::utility::qr(),
                commands::utility::embed(),
                commands::utility::timestamp(),
                commands::utility::math(),
                commands::utility::weather(),
                commands::utility::crypto(),
                commands::utility::translate(),
                commands::utility::remind(),
                commands::utility::dictionary(),
                commands::utility::worldclock(),
                commands::utility::poll(),
                // fun
                commands::fun::fun(),
                commands::fun::games(),
                // economy
                commands::economy::economy(),
                // leveling
                commands::leveling::leveling(),
                // moderation
                commands::moderation::ban(),
                commands::moderation::softban(),
                commands::moderation::unban(),
                commands::moderation::kick(),
                commands::moderation::mute(),
                commands::moderation::unmute(),
                commands::moderation::timeout(),
                commands::moderation::warn(),
                commands::moderation::warns(),
                commands::moderation::clearwarns(),
                commands::moderation::purge(),
                commands::moderation::nuke(),
                commands::moderation::slowmode(),
                commands::moderation::slowmode_global(),
                commands::moderation::cases(),
                commands::moderation::lock(),
                commands::moderation::unlock(),
                commands::moderation::tactical(),
                // giveaways
                commands::giveaway::giveaway(),
                // config
                commands::config::automod(),
            ],
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "Processing /{} for {}",
                        ctx.command().name,
                        ctx.author().name
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    let pool = &ctx.data().database.pool;
                    let _ = sqlx::query(
                        "INSERT INTO global_stats (stat_key, stat_value) \
                         VALUES ('total_commands_executed', 1) \
                         ON CONFLICT (stat_key) \
                         DO UPDATE SET stat_value = global_stats.stat_value + 1",
                    )
                    .execute(pool)
                    .await;
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            error!("Command error in {}: {:?}", ctx.command().name, error);
                            let _ = ctx
                                .send(
                                    poise::CreateReply::default()
                                        .embed(
                                            serenity::CreateEmbed::new()
                                                .title("Command Error")
                                                .description(format!("{}", error))
                                                .color(0xFF3B3B),
                                        )
                                        .ephemeral(true),
                                )
                                .await;
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
                        if guild_id == 0 {
                            return Ok(Some("!".into()));
                        }
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
                let ctx_clone = ctx.clone();
                Box::pin(async move {
                    info!("AegisForge online as {}", ready.user.name);
                    
                    tokio::spawn(async move {
                        let statuses = [
                            "v4.3 Elite",
                            "Sentinel Anti-Raid",
                            "/help | aegisforge.com",
                            "AutoMod Active"
                        ];
                        let mut i = 0;
                        loop {
                            let activity = serenity::ActivityData::playing(statuses[i % statuses.len()]);
                            ctx_clone.set_presence(Some(activity), serenity::OnlineStatus::Online);
                            i += 1;
                            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        }
                    });
                    
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {
                        database: db,
                        start_time,
                        http_client: reqwest::Client::new(),
                        raid_tracker: Arc::new(dashmap::DashMap::new()),
                        sentinel_settings: Arc::new(dashmap::DashMap::new()),
                        spam_tracker: Arc::new(dashmap::DashMap::new()),
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

    info!("Starting stats API...");
    let app_state = AppState {
        cache: Arc::clone(&client.cache),
        database: Arc::clone(&database),
        shard_manager,
        start_time,
    };

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/api/health", get(get_health))
        .route("/api/stats", get(get_stats))
        .route("/api/vote", post(handle_vote))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("Stats API listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("Stats server error: {}", e);
        }
    });

    info!("Starting Discord client...");
    client.start().await?;

    Ok(())
}
