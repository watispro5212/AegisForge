# AegisForge

AegisForge v4.1 is a high-performance Discord bot built in Rust for moderation, economy, shop inventory, leveling, server configuration, utility commands, and live status telemetry.

The bot runs on Serenity, Poise, Tokio, SQLx, Neon Postgres, DashMap, and Axum. The public website is static, but its status and about pages read real live bot stats from the Rust API instead of hardcoded counters.

## Current Release

- Version: `4.1.2` website/docs refresh on top of bot version `4.1.0`
- Bot/API status: `GET /api/health`
- Live stats: `GET /api/stats`
- Website stats route: `web/api/stats.js` proxies the live Rust API first, then falls back to Neon database totals if the bot API is unavailable.

## Features

- Moderation: ban, softban, unban, kick, timeout, mute, unmute, warn, purge, nuke, slowmode, lock, and unlock.
- Economy: wallet, bank, daily rewards, work, fishing, hunting, crime, slots, robbery, payments, global shop purchases, inventory, and leaderboards.
- Leveling: message XP, rank lookup, local/global leaderboards, and rank card customization.
- Configuration: logs, welcome messages, autorole, prefixes, and guild settings.
- Utility: ping, stats, bot info, server/user lookup, avatars, embeds, timestamps, math, QR codes, dictionary lookup, timers, polls, and world clock.
- Website: static front end with live API-backed status, shard telemetry, about stats, command registry, economy guide, changelog, and policy pages.

## Tech Stack

- Rust 2021
- Tokio
- Serenity
- Poise
- SQLx
- Neon/PostgreSQL
- Axum
- DashMap
- Vercel for the website
- Fly.io for the bot/API deployment

## Environment

Create a `.env` file based on `.env.example` and configure:

```env
DISCORD_TOKEN=
DATABASE_URL=
DATABASE_POOL_URL=
STATUS_WEBHOOK_URL=
PORT=8080
```

`DATABASE_URL` should be a direct PostgreSQL connection string for migrations. `DATABASE_POOL_URL` can point at the pooled Neon/PgBouncer endpoint for normal app traffic.

For the website API route, optionally set:

```env
BOT_STATS_URL=https://aegisforge.fly.dev/api/stats
```

If `BOT_STATS_URL` is not set, the website API route uses the production Fly.io stats endpoint by default.

## Local Development

```bash
cargo check
cargo run
```

For offline SQLx builds:

```bash
SQLX_OFFLINE=true cargo check
```

For the static website:

```bash
cd web
npm test
python -m http.server 4173
```

Then open `http://127.0.0.1:4173/`.

## Verification

Before submitting changes, run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cd web && npm test
```

## Deployment Notes

- The bot exposes `GET /` for health checks.
- The live stats API is available at `GET /api/stats`.
- Top.gg vote webhooks are handled at `POST /api/vote`.
- Fly.io should route to internal port `8080`.
- The website should load stats through `/api/stats` in production and can use the Fly.io stats API directly during local static previews.

## Useful Links

- Website: https://aegisforge-vert.vercel.app
- Support Server: https://discord.gg/HbmafcgjNa
- Top.gg: https://top.gg/bot/1500582485367722004

## Contributing

Contributions are welcome. Keep changes focused, make command behavior match the website/docs, and run the verification commands before submitting changes.

## License

MIT. See `LICENSE`.
