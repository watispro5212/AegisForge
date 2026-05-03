# ⚙ AegisForge

> A fast, secure, and customizable Rust-powered Discord bot for moderation, automation, and server utilities.

---

## Features

| Category | Commands |
|----------|----------|
| **Moderation** | `/mod ban` `/mod unban` `/mod kick` `/mod timeout` `/mod warn` `/mod purge` |
| **Utility** | `/util ping` `/util server` `/util user` `/util avatar` `/util uptime` `/util timestamp` |
| **Roles** | `/role add` `/role remove` `/role list` |
| **Config** | `/config logs` `/config welcome` `/config autorole` |
| **Reminders** | `/remind create` |

---

## Stack

- **Language:** Rust 🦀
- **Framework:** [Poise](https://github.com/serenity-rs/poise) + [Serenity](https://github.com/serenity-rs/serenity)
- **Async:** Tokio
- **Database:** SQLx + SQLite (swap for Postgres in production)
- **Logging:** Tracing

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.75+)
- A Discord bot token from the [Developer Portal](https://discord.com/developers/applications)

### Setup

```bash
# 1. Clone the repository
git clone https://github.com/your-username/AegisForge.git
cd AegisForge

# 2. Copy and fill in the environment file
cp .env.example .env
# Edit .env and paste your bot token

# 3. Run migrations (creates aegisforge.db)
sqlx migrate run

# 4. Build and run
cargo run --release
```

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `DISCORD_TOKEN` | ✅ | Your bot's token |
| `DATABASE_URL` | ✅ | **Direct** Neon URL — used only for migrations at startup |
| `DATABASE_POOL_URL` | ✅ | **Pooled** Neon URL — used for all bot queries |
| `DB_MAX_CONNECTIONS` | ❌ | SQLx pool size to PgBouncer (default: `10`) |
| `RUST_LOG` | ❌ | Log level, e.g. `aegisforge=info,sqlx=warn` |

> **Why two URLs?** Neon runs PgBouncer in transaction mode for pooled connections.
> Transaction mode doesn't support DDL statements (`CREATE TABLE`, etc.),
> so migrations **must** use the direct URL. Normal queries use the pooled URL
> which handles up to 10,000 client connections through Neon's built-in PgBouncer.

### Getting Your Neon URLs

The two URLs differ by only `-pooler` in the hostname:

```
# Direct (DATABASE_URL) — for migrations:
postgresql://USER:PASS@ep-your-endpoint.region.aws.neon.tech/neondb?sslmode=require

# Pooled (DATABASE_POOL_URL) — for the bot:
postgresql://USER:PASS@ep-your-endpoint-pooler.region.aws.neon.tech/neondb?sslmode=require
```

Get both from the Neon Console → your project → **Connect** → toggle **Connection pooling**.

---

## Project Structure

```
AegisForge/
├── src/
│   ├── main.rs              # Entry point, bot setup
│   ├── handler.rs           # Gateway event handler
│   ├── commands/
│   │   ├── mod.rs           # Module exports
│   │   ├── utility.rs       # Utility commands
│   │   ├── moderation.rs    # Moderation commands
│   │   ├── role.rs          # Role management
│   │   ├── config.rs        # Server configuration
│   │   └── remind.rs        # Reminder system
│   └── models/
│       ├── mod.rs
│       └── config.rs        # Data models
├── migrations/
│   └── 0001_initial.sql     # Database schema
├── web/
│   ├── index.html           # Landing page
│   ├── style.css
│   └── script.js
├── .env.example
├── .gitignore
├── Cargo.toml
└── context.md               # Project brief & design spec
```

---

## Permissions

AegisForge requests only what it needs:
- `Read Messages` / `Send Messages`
- `Embed Links`
- `Manage Messages`
- `Moderate Members`
- `Manage Roles`
- `View Audit Log`
- `Use Application Commands`

---

## License

MIT — built with 🦀 and precision.
