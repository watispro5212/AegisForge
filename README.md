# AegisForge

AegisForge is a high-performance Discord bot built in Rust for moderation, economy, leveling, server configuration, utility commands, and live status telemetry.

It is designed for communities that need practical tools without a slow or fragile runtime: Serenity and Poise handle Discord, Tokio powers async work, SQLx persists data in PostgreSQL, and Axum exposes a lightweight stats API for the website.

## Features

- Moderation: ban, softban, unban, kick, timeout, mute, unmute, warn, purge, nuke, slowmode, lock, and unlock.
- Economy: wallet, bank, daily rewards, work, fishing, hunting, crime, slots, robbery, payments, and leaderboards.
- Leveling: message XP, rank lookup, local/global leaderboards, and rank card customization.
- Configuration: logs, welcome messages, autorole, prefixes, and guild settings.
- Utility: ping, stats, bot info, server/user lookup, avatars, embeds, timestamps, math, QR codes, dictionary lookup, timers, polls, and world clock.
- Website: static front end with live API-backed bot status and command reference.

## Tech Stack

- Rust
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

## Local Development

```bash
cargo check
cargo run
```

For offline SQLx builds:

```bash
SQLX_OFFLINE=true cargo check
```

## Deployment Notes

- The bot exposes `GET /` for health checks.
- The live stats API is available at `GET /api/stats`.
- Top.gg vote webhooks are handled at `POST /api/vote`.
- Fly.io should route to internal port `8080`.
- The website reads live stats from the configured API endpoint.

## Useful Links

- Website: https://aegisforge-vert.vercel.app
- Support Server: https://discord.gg/8p5Epc8Qd8
- Top.gg: https://top.gg/bot/1500582485367722004

## Contributing

Contributions are welcome. Keep changes focused, make command behavior match the website/docs, and run `cargo check` before submitting changes.

## License

MIT. See `LICENSE`.
